use emuella_viewer_source::*;
use emuella_viewer_tools::*;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
fn profile() -> Profile {
    Profile {
        width: 516,
        height: 260,
        tile_edge: 256,
        decomposition_levels: 5,
        bits_per_sample: 11,
        components: 1,
        bits_per_pixel: 3.,
    }
}
fn region() -> Region {
    Region {
        x: 247,
        y: 7,
        width: 263,
        height: 249,
        discard: 2,
        components: vec![0],
    }
}
fn install(client: &mut SharedClient, service: &mut Service, manifest: &Manifest, region: &Region) {
    client.register(manifest.clone()).unwrap();
    for tile in client.missing_tiles(&manifest.tid, region).unwrap() {
        let response = service
            .route(&format!(
                "/descriptor/{}/{tile}?tid={}",
                manifest.target, manifest.tid
            ))
            .unwrap();
        client
            .install_descriptor(&manifest.tid, tile, &response.body)
            .unwrap();
    }
}
fn fields(response: &HttpResponse) -> jpip::ResponseFields {
    let get = |name: &str| {
        response
            .headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .unwrap()
            .1
            .as_str()
    };
    checked(jpip::ResponseFields::parse(
        get("JPIP-tid"),
        get("JPIP-fsiz"),
        get("JPIP-roff"),
        get("JPIP-rsiz"),
    ))
    .unwrap()
}
#[test]
fn restart_fragment_retry_reuse_and_reference() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("rep");
    let (manifest, _) = fixture(&root, "test", profile(), "fixture-revision").unwrap();
    let mut service = Service::open(std::slice::from_ref(&root), None, true).unwrap();
    let mut client = SharedClient::new(ClientLimits::default());
    let region = region();
    install(&mut client, &mut service, &manifest, &region);
    let request = client.request(&manifest.tid, &region, 512).unwrap();
    let response = service
        .route(&format!("/jpip?{}", checked(request.query()).unwrap()))
        .unwrap();
    let mut reader = client
        .begin_response(&manifest.tid, &fields(&response))
        .unwrap();
    client
        .receive(&mut reader, &response.body[..response.body.len() / 2])
        .unwrap();
    assert!(client.finish(reader).is_err());
    drop(service);
    let mut service = Service::open(std::slice::from_ref(&root), None, false).unwrap();
    for attempt in 0..100 {
        let request = client.request(&manifest.tid, &region, 1024).unwrap();
        let response = service
            .route(&format!("/jpip?{}", checked(request.query()).unwrap()))
            .unwrap();
        let mut reader = client
            .begin_response(&manifest.tid, &fields(&response))
            .unwrap();
        for bytes in response.body.chunks(7) {
            client.receive(&mut reader, bytes).unwrap();
        }
        client.finish(reader).unwrap();
        if client.decode(&manifest.tid, &region).is_ok() {
            break;
        }
        assert!(attempt < 99, "retry did not converge");
    }
    let decoded = client.decode(&manifest.tid, &region).unwrap();
    assert_eq!((decoded.width, decoded.height), (66, 62));
    let mut index = manifest.sparse().unwrap();
    for tile in region.tiles(&profile()).unwrap() {
        checked(index.import_tile_descriptor(
            &std::fs::read(root.join("descriptors").join(format!("{tile}.bin"))).unwrap(),
        ))
        .unwrap();
    }
    let bytes = std::fs::read(root.join("payload.j2c")).unwrap();
    let expected = checked(index.plan(region.rect(), region.discard, &region.components))
        .unwrap()
        .decode(
            |offset, out| {
                out.copy_from_slice(&bytes[offset as usize..offset as usize + out.len()]);
                Ok(())
            },
            &mut codec::ht_lossy::LossyHtSpatialRegionWorkspace::new(),
        )
        .unwrap();
    assert_eq!(decoded.planes, expected);
    let before = service.metrics.logical_read_bytes;
    let response = service
        .route(&format!(
            "/jpip?{}",
            checked(
                client
                    .request(&manifest.tid, &region, 1024)
                    .unwrap()
                    .query()
            )
            .unwrap()
        ))
        .unwrap();
    assert_eq!(response.body.len(), 3);
    assert_eq!(service.metrics.logical_read_bytes, before);
    assert!(service.route("/descriptor/../../bad/0?tid=x").is_err());
    let mut corrupted = manifest.clone();
    corrupted.identity.profile.bits_per_pixel += 1.;
    assert!(corrupted.validate().is_err());
}
#[test]
fn actual_http_loopback_catalogue_and_jpp() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("rep");
    let (manifest, _) = fixture(&root, "wire", profile(), "fixture-revision").unwrap();
    let mut service = Service::open(&[root], None, true).unwrap();
    let region = region();
    let mut client = SharedClient::new(ClientLimits::default());
    install(&mut client, &mut service, &manifest, &region);
    let query = checked(
        client
            .request(&manifest.tid, &region, 1 << 20)
            .unwrap()
            .query(),
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let handle = std::thread::spawn(move || {
        for _ in 0..2 {
            let (stream, _) = listener.accept().unwrap();
            service.handle(stream).unwrap();
        }
    });
    for path in ["/catalogue".to_string(), format!("/jpip?{query}")] {
        let mut stream = TcpStream::connect(address).unwrap();
        write!(stream, "GET {path} HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut bytes = Vec::new();
        stream.read_to_end(&mut bytes).unwrap();
        let end = bytes.windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
        let header = std::str::from_utf8(&bytes[..end]).unwrap();
        assert!(header.starts_with("HTTP/1.1 200"));
        if path == "/catalogue" {
            let catalogue: Vec<Manifest> = serde_json::from_slice(&bytes[end..]).unwrap();
            assert_eq!(catalogue[0], manifest);
        } else {
            let get = |name: &str| {
                header
                    .lines()
                    .filter_map(|l| l.split_once(": "))
                    .find(|(n, _)| n.eq_ignore_ascii_case(name))
                    .unwrap()
                    .1
            };
            let fields = checked(jpip::ResponseFields::parse(
                get("JPIP-tid"),
                get("JPIP-fsiz"),
                get("JPIP-roff"),
                get("JPIP-rsiz"),
            ))
            .unwrap();
            let mut reader = client.begin_response(&manifest.tid, &fields).unwrap();
            client.receive(&mut reader, &bytes[end..]).unwrap();
            client.finish(reader).unwrap();
            client.decode(&manifest.tid, &region).unwrap();
        }
    }
    handle.join().unwrap();
}
#[test]
fn incomplete_preparation_never_publishes() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("bad");
    let good = dir.path().join("good");
    let (m, _) = fixture(&good, "source", profile(), "fixture-revision").unwrap();
    assert!(
        prepare(&root, "bad", m.identity, |_, _| anyhow::bail!(
            "injected source failure"
        ))
        .is_err()
    );
    assert!(!root.exists());
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}
#[test]
fn global_identity_eviction_and_descriptor_authentication() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("rep");
    let (m, _) = fixture(&root, "first", profile(), "fixture-revision").unwrap();
    let mut client = SharedClient::new(ClientLimits {
        representations: 1,
        ..ClientLimits::default()
    });
    client.register(m.clone()).unwrap();
    let mut other = m.clone();
    other.identity.spatial_policy_sha256 = sha256(b"future-policy");
    other.seal().unwrap();
    client.register(other.clone()).unwrap();
    assert!(client.missing_tiles(&m.tid, &region()).is_err());
    let bytes = std::fs::read(root.join("descriptors/0.bin")).unwrap();
    let mut bad = bytes.clone();
    bad[0] ^= 1;
    assert!(client.install_descriptor(&other.tid, 0, &bad).is_err());
    client.install_descriptor(&other.tid, 0, &bytes).unwrap();
    assert!(client.resident_bytes().1 <= ClientLimits::default().descriptor_bytes);
}

#[test]
fn stored_payload_and_descriptor_corruption_are_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("rep");
    let (manifest, _) = fixture(&root, "integrity", profile(), "fixture-revision").unwrap();
    let payload = root.join("payload.j2c");
    let mut bytes = std::fs::read(&payload).unwrap();
    bytes[8] ^= 1;
    std::fs::write(&payload, &bytes).unwrap();
    assert!(Service::open(std::slice::from_ref(&root), None, true).is_err());
    // Explicit trusted-store mode skips payload hashing, but descriptor hashes remain mandatory.
    let mut service = Service::open(std::slice::from_ref(&root), None, false).unwrap();
    let descriptor = root.join("descriptors/0.bin");
    let mut bytes = std::fs::read(&descriptor).unwrap();
    bytes[10] ^= 1;
    std::fs::write(descriptor, bytes).unwrap();
    assert!(
        service
            .route(&format!("/descriptor/integrity/0?tid={}", manifest.tid))
            .is_err()
    );
}
#[test]
fn effective_resolution_rounding_and_quality_admission() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("rep");
    let (manifest, _) = fixture(&root, "rounding", profile(), "fixture-revision").unwrap();
    let request = jpip::Request {
        target: manifest.target.clone(),
        tid: manifest.tid.clone(),
        frame: [200, 100],
        offset: [91, 2],
        size: [109, 98],
        components: vec![0],
        layers: Some(1),
        max_length: 1024,
        extended: true,
        model: Default::default(),
    };
    let (region, fields) = effective_region(&manifest, &request).unwrap();
    assert_eq!(fields.frame, [129, 65]);
    assert_eq!(fields.offset, [58, 1]);
    assert_eq!(fields.size, [71, 64]);
    assert_eq!(
        (
            region.x,
            region.y,
            region.width,
            region.height,
            region.discard
        ),
        (232, 4, 284, 256, 2)
    );
    let mut unsupported = request;
    unsupported.layers = Some(2);
    assert!(effective_region(&manifest, &unsupported).is_err());
}

#[test]
fn representative_u16_cross_tile_reference() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("large");
    let p = Profile {
        width: 2048,
        height: 2048,
        tile_edge: 512,
        decomposition_levels: 5,
        bits_per_sample: 16,
        components: 1,
        bits_per_pixel: 2.,
    };
    let (manifest, _) = fixture(&root, "large", p, "fixture-revision").unwrap();
    let region = Region {
        x: 247,
        y: 117,
        width: 769,
        height: 513,
        discard: 2,
        components: vec![0],
    };
    let mut index = manifest.sparse().unwrap();
    for tile in region.tiles(&manifest.identity.profile).unwrap() {
        checked(index.import_tile_descriptor(
            &std::fs::read(root.join("descriptors").join(format!("{tile}.bin"))).unwrap(),
        ))
        .unwrap();
    }
    let bytes = std::fs::read(root.join("payload.j2c")).unwrap();
    let plan = checked(index.plan(region.rect(), region.discard, &region.components)).unwrap();
    let mut reads = 0;
    let expected = plan.decode(
        |offset, out| {
            reads += 1;
            out.copy_from_slice(&bytes[offset as usize..offset as usize + out.len()]);
            Ok(())
        },
        &mut codec::ht_lossy::LossyHtSpatialRegionWorkspace::new(),
    );
    assert!(
        expected.is_ok(),
        "direct file decode: {:?}, reads={reads}, blocks={}, workspace={}",
        expected.err(),
        plan.selected_code_blocks(),
        plan.required_workspace_bytes()
    );
}
