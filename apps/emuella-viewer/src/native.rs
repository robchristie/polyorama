use crate::engine::{CATALOGUE_LIMIT, Engine, Event, HTTP_LIMIT, Job};
use anyhow::{Result, anyhow, ensure};
use emuella_viewer_source::{Manifest, jpip::ResponseFields};
use polyorama_runtime::{RegionalRequest, RequestToken};
use std::{
    collections::BTreeSet,
    io::Read,
    sync::{Arc, Mutex, mpsc},
    time::{Duration, Instant},
};

pub struct Executor {
    sender: mpsc::SyncSender<Job>,
    events: mpsc::Receiver<Event>,
    cancelled: Arc<Mutex<BTreeSet<RequestToken>>>,
}
impl Executor {
    pub fn new(server: String, compressed: usize, context: egui::Context) -> Self {
        let (sender, jobs) = mpsc::sync_channel::<Job>(1);
        let (events_tx, events) = mpsc::channel();
        let cancelled = Arc::new(Mutex::new(BTreeSet::new()));
        let flags = cancelled.clone();
        std::thread::Builder::new()
            .name("emuella-regional-decoder".into())
            .spawn(move || {
                let mut engine = Engine::new(compressed);
                let http = reqwest::blocking::Client::builder()
                    .timeout(Duration::from_secs(15))
                    .build()
                    .expect("HTTP client");
                let catalogue = (|| -> Result<Vec<Manifest>> {
                    let bytes = body(
                        http.get(format!("{server}/catalogue")).send()?,
                        CATALOGUE_LIMIT,
                    )?;
                    let manifests: Vec<Manifest> = serde_json::from_slice(&bytes)?;
                    ensure!(manifests.len() <= 32, "catalogue capacity");
                    for m in &manifests {
                        m.validate()?;
                    }
                    Ok(manifests)
                })();
                let event = match catalogue {
                    Ok(c) => Event::Catalogue(c),
                    Err(e) => Event::Failed {
                        request: None,
                        error: e.to_string(),
                        metrics: engine.snapshot(),
                    },
                };
                let _ = events_tx.send(event);
                context.request_repaint();
                while let Ok(job) = jobs.recv() {
                    let started = Instant::now();
                    let stopped = || {
                        flags
                            .lock()
                            .expect("cancellation lock")
                            .contains(&job.request.token)
                    };
                    let result = (|| -> Result<_> {
                        ensure!(!stopped(), "cancelled");
                        engine.client.register(job.manifest.clone())?;
                        // Metadata admission may evict an earlier tile from this batch.
                        // Recheck the complete bounded region before requesting any bins.
                        for _ in 0..3 {
                            for tile in engine
                                .client
                                .missing_tiles(&job.manifest.tid, &job.region())?
                            {
                                ensure!(!stopped(), "cancelled");
                                let bytes = body(
                                    http.get(format!(
                                        "{server}/descriptor/{}/{tile}?tid={}",
                                        job.manifest.target, job.manifest.tid
                                    ))
                                    .send()?,
                                    HTTP_LIMIT,
                                )?;
                                ensure!(!stopped(), "cancelled");
                                engine.client.install_descriptor(
                                    &job.manifest.tid,
                                    tile,
                                    &bytes,
                                )?;
                            }
                            if engine
                                .client
                                .missing_tiles(&job.manifest.tid, &job.region())?
                                .is_empty()
                            {
                                break;
                            }
                        }
                        ensure!(
                            engine
                                .client
                                .missing_tiles(&job.manifest.tid, &job.region())?
                                .is_empty(),
                            "regional descriptors exceed admitted metadata budget"
                        );
                        if engine.client.ready(&job.manifest.tid, &job.region())? {
                            engine.metrics.cache_hits += 1;
                            return engine.decode(&job);
                        }
                        for attempt in 0..64 {
                            ensure!(!stopped(), "cancelled");
                            let request = engine.client.request(
                                &job.manifest.tid,
                                &job.region(),
                                (256 << 10) as u64,
                            )?;
                            engine.metrics.requests += 1;
                            if attempt > 0 {
                                engine.metrics.retries += 1;
                            }
                            let mut response = http
                                .get(format!(
                                    "{server}/jpip?{}",
                                    emuella_viewer_source::checked(request.query())?
                                ))
                                .send()?
                                .error_for_status()?;
                            let header = |name: &str| -> Result<String> {
                                Ok(response
                                    .headers()
                                    .get(name)
                                    .ok_or_else(|| anyhow!("missing {name}"))?
                                    .to_str()?
                                    .to_owned())
                            };
                            let fields = ResponseFields {
                                tid: header("JPIP-tid")?,
                                frame: pair(&header("JPIP-fsiz")?)?,
                                offset: pair(&header("JPIP-roff")?)?,
                                size: pair(&header("JPIP-rsiz")?)?,
                            };
                            ensure!(!stopped(), "cancelled");
                            let mut reader =
                                engine.client.begin_response(&job.manifest.tid, &fields)?;
                            let mut chunk = [0; 8192];
                            let mut received = 0;
                            loop {
                                ensure!(!stopped(), "cancelled");
                                let n = response.read(&mut chunk)?;
                                if n == 0 {
                                    break;
                                }
                                received += n;
                                ensure!(received <= HTTP_LIMIT, "response limit");
                                engine.client.receive(&mut reader, &chunk[..n])?;
                            }
                            engine.client.finish(reader)?;
                            ensure!(!stopped(), "cancelled");
                            if engine.client.ready(&job.manifest.tid, &job.region())? {
                                return engine.decode(&job);
                            }
                        }
                        Err(anyhow!(
                            "regional decode failed after bounded retry budget: {}",
                            engine.decode(&job).unwrap_err()
                        ))
                    })();
                    engine.metrics.elapsed_ms += started.elapsed().as_secs_f64() * 1000.;
                    let event = if stopped() {
                        engine.metrics.aborted += 1;
                        drop(result);
                        Event::Cancelled {
                            request: job.request.clone(),
                            metrics: engine.snapshot(),
                        }
                    } else {
                        match result {
                            Ok(pixels) => Event::Completed {
                                request: job.request.clone(),
                                pixels,
                                metrics: engine.snapshot(),
                            },
                            Err(e) => Event::Failed {
                                request: Some(job.request.clone()),
                                error: e.to_string(),
                                metrics: engine.snapshot(),
                            },
                        }
                    };
                    flags
                        .lock()
                        .expect("cancellation lock")
                        .remove(&job.request.token);
                    if events_tx.send(event).is_err() {
                        break;
                    }
                    context.request_repaint();
                }
            })
            .expect("decoder thread");
        Self {
            sender,
            events,
            cancelled,
        }
    }
    pub fn submit(&self, job: Job) -> Result<()> {
        self.sender
            .try_send(job)
            .map_err(|e| anyhow!(e.to_string()))
    }
    pub fn cancel(&self, request: &RegionalRequest) {
        self.cancelled
            .lock()
            .expect("cancellation lock")
            .insert(request.token);
    }
    pub fn drain(&self) -> Vec<Event> {
        self.events
            .try_iter()
            .inspect(|event| {
                let request = match event {
                    Event::Completed { request, .. } | Event::Cancelled { request, .. } => {
                        Some(request)
                    }
                    Event::Failed { request, .. } => request.as_ref(),
                    Event::Catalogue(_) => None,
                };
                if let Some(request) = request {
                    // Cancellation may race with worker publication after its own cleanup.
                    self.cancelled
                        .lock()
                        .expect("cancellation lock")
                        .remove(&request.token);
                }
            })
            .collect()
    }
}
fn body(response: reqwest::blocking::Response, limit: usize) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    response
        .error_for_status()?
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= limit, "HTTP body exceeds bound");
    Ok(bytes)
}
fn pair(text: &str) -> Result<[u32; 2]> {
    let parts = text
        .split(',')
        .take(2)
        .map(str::parse)
        .collect::<std::result::Result<Vec<_>, _>>()?;
    ensure!(parts.len() == 2, "invalid JPIP geometry header");
    Ok([parts[0], parts[1]])
}
