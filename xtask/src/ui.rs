use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, anyhow, bail};
use polyorama_gallery::{GalleryConfiguration, StoryId};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const FIXTURE_MANIFEST: &str = "docs/ui-snapshots/fixtures.json";
const EXPECTED_ROOT: &str = "docs/ui-snapshots/expected";
const SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FixtureManifest {
    schema_version: u32,
    fixtures: Vec<UiFixture>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct UiFixture {
    id: String,
    story: StoryId,
    viewport: Viewport,
    configuration: GalleryConfiguration,
    data_fixture: String,
    fonts: String,
    renderer: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Viewport {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct UiArguments {
    action: String,
    output_directory: PathBuf,
    fixture: Option<String>,
    all: bool,
}

#[derive(Debug, Deserialize)]
struct VisualDiff {
    dimensions_equal: bool,
    differing_pixels: u64,
    total_pixels: u64,
}

pub fn run(root: &Path, arguments: Vec<String>) -> Result<()> {
    let arguments = parse_arguments(arguments)?;
    let manifest = load_manifest(root)?;
    fs::create_dir_all(&arguments.output_directory).with_context(|| {
        format!(
            "create UI evidence directory {}",
            arguments.output_directory.display()
        )
    })?;

    match arguments.action.as_str() {
        "list" => list(&manifest, &arguments.output_directory),
        "render" | "inspect" => {
            let fixture = selected_fixture(&manifest, arguments.fixture.as_deref())?;
            capture(root, fixture, &arguments.output_directory, None)?;
            write_summary(
                &arguments.output_directory,
                &arguments.action,
                "passed",
                json!({ "fixture": fixture.id }),
            )
        }
        "audit-text" => audit_text(root, &manifest, &arguments),
        "verify" => verify(root, &arguments.output_directory),
        action => bail!(
            "unknown ui action {action:?}; expected list, render, inspect, audit-text or verify"
        ),
    }
}

pub fn verify(root: &Path, output_directory: &Path) -> Result<()> {
    let manifest = load_manifest(root)?;
    validate_evaluation_seed(root)?;
    validate_expected_fixture_set(root, &manifest)?;
    let captures = output_directory.join("captures");
    let failures = output_directory.join("failures");
    recreate_directory(&captures)?;
    recreate_directory(&failures)?;

    let mut failure_ids = Vec::new();
    for fixture in &manifest.fixtures {
        let actual = captures.join(&fixture.id);
        let expected = root.join(EXPECTED_ROOT).join(&fixture.id);
        let expected_visual = expected.join("visual.png");
        if !expected.is_dir() {
            failure_ids.push(fixture.id.clone());
            write_missing_baseline_failure(&failures, fixture)?;
            continue;
        }

        capture(root, fixture, &actual, Some(&expected_visual))?;
        let mut fixture_failed = false;
        let failure = failures.join(&fixture.id);
        for name in ["metadata.json", "semantic.json", "text.json"] {
            let expected_value = read_json(&expected.join(name))?;
            let actual_value = read_json(&actual.join(name))?;
            if expected_value != actual_value {
                fixture_failed = true;
            }
        }
        let visual_diff: VisualDiff =
            serde_json::from_value(read_json(&actual.join("visual-diff.json"))?)?;
        if !visual_diff.dimensions_equal || visual_diff.differing_pixels != 0 {
            fixture_failed = true;
        }
        if fixture_failed {
            failure_ids.push(fixture.id.clone());
            populate_failure_bundle(&failure, &expected, &actual, &visual_diff)?;
        }
    }

    let status = if failure_ids.is_empty() {
        "passed"
    } else {
        "failed"
    };
    write_summary(
        output_directory,
        "verify",
        status,
        json!({
            "fixture_count": manifest.fixtures.len(),
            "failed_fixtures": failure_ids,
            "baseline_updates_allowed": false
        }),
    )?;
    if status == "failed" {
        bail!("UI verification failed; inspect {}", failures.display());
    }
    Ok(())
}

fn parse_arguments(arguments: Vec<String>) -> Result<UiArguments> {
    let mut arguments = arguments.into_iter();
    let action = arguments.next().ok_or_else(|| {
        anyhow!("ui requires an action: list, render, inspect, audit-text or verify")
    })?;
    let mut output_directory = None;
    let mut fixture = None;
    let mut all = false;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--output-dir" => {
                let value = arguments
                    .next()
                    .ok_or_else(|| anyhow!("--output-dir requires a path"))?;
                output_directory = Some(PathBuf::from(value));
            }
            "--fixture" => {
                fixture = Some(
                    arguments
                        .next()
                        .ok_or_else(|| anyhow!("--fixture requires an id"))?,
                );
            }
            "--all" => all = true,
            unknown => bail!("unknown ui argument {unknown:?}"),
        }
    }
    let output_directory = output_directory
        .ok_or_else(|| anyhow!("ui commands require an explicit --output-dir <path>"))?;
    if fixture.is_some() && all {
        bail!("use either --fixture <id> or --all, not both");
    }
    match action.as_str() {
        "render" | "inspect" if fixture.is_none() => {
            bail!("ui {action} requires --fixture <id>")
        }
        "audit-text" if fixture.is_none() && !all => {
            bail!("ui audit-text requires --fixture <id> or --all")
        }
        "list" | "verify" if fixture.is_some() || all => {
            bail!("ui {action} does not accept --fixture or --all")
        }
        _ => {}
    }
    Ok(UiArguments {
        action,
        output_directory,
        fixture,
        all,
    })
}

fn load_manifest(root: &Path) -> Result<FixtureManifest> {
    let path = root.join(FIXTURE_MANIFEST);
    let manifest: FixtureManifest = serde_json::from_slice(
        &fs::read(&path).with_context(|| format!("read UI fixture manifest {}", path.display()))?,
    )
    .with_context(|| format!("parse UI fixture manifest {}", path.display()))?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

fn validate_manifest(manifest: &FixtureManifest) -> Result<()> {
    if manifest.schema_version != SCHEMA_VERSION {
        bail!(
            "unsupported UI fixture schema {}; expected {SCHEMA_VERSION}",
            manifest.schema_version
        );
    }
    if manifest.fixtures.is_empty() {
        bail!("UI fixture manifest must not be empty");
    }
    let mut ids = BTreeSet::new();
    for fixture in &manifest.fixtures {
        if fixture.id.is_empty()
            || !fixture
                .id
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            bail!("invalid stable UI fixture id {:?}", fixture.id);
        }
        if !ids.insert(&fixture.id) {
            bail!("duplicate UI fixture id {:?}", fixture.id);
        }
        if !(320..=3840).contains(&fixture.viewport.width)
            || !(240..=2160).contains(&fixture.viewport.height)
        {
            bail!("invalid viewport for UI fixture {:?}", fixture.id);
        }
        if !fixture.configuration.font_scale.is_finite()
            || fixture.configuration.validated() != fixture.configuration
        {
            bail!(
                "invalid gallery configuration for UI fixture {:?}",
                fixture.id
            );
        }
        for (label, value) in [
            ("data_fixture", fixture.data_fixture.as_str()),
            ("fonts", fixture.fonts.as_str()),
            ("renderer", fixture.renderer.as_str()),
        ] {
            if value.trim().is_empty() {
                bail!("UI fixture {:?} has empty {label}", fixture.id);
            }
        }
    }
    Ok(())
}

fn validate_evaluation_seed(root: &Path) -> Result<()> {
    let path = root.join("docs/ui-evaluation-seed.json");
    let seed = read_json(&path)?;
    if seed.get("schema_version").and_then(Value::as_u64) != Some(1)
        || seed.get("status").and_then(Value::as_str) != Some("frozen")
    {
        bail!("UI evaluation seed must use schema 1 and remain frozen");
    }
    let dimensions = seed
        .pointer("/scoring/dimensions")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("UI evaluation seed has no scoring dimensions"))?;
    if dimensions.len() != 4
        || seed
            .pointer("/scoring/scale/minimum")
            .and_then(Value::as_i64)
            != Some(0)
        || seed
            .pointer("/scoring/scale/maximum")
            .and_then(Value::as_i64)
            != Some(2)
    {
        bail!("UI evaluation seed must retain its four measurable 0–2 dimensions");
    }
    let tasks = seed
        .get("tasks")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("UI evaluation seed has no tasks"))?;
    if tasks.len() < 5 {
        bail!("UI evaluation seed must retain at least five frozen tasks");
    }
    let mut ids = BTreeSet::new();
    for task in tasks {
        let id = task
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("UI evaluation task has no id"))?;
        if !ids.insert(id) {
            bail!("duplicate UI evaluation task id {id:?}");
        }
        let story = task
            .get("story")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("UI evaluation task {id:?} has no story"))?;
        story
            .parse::<StoryId>()
            .map_err(|error| anyhow!("UI evaluation task {id:?}: {error}"))?;
        for dimension in ["visual_text", "semantics", "interaction", "evidence"] {
            let assertions = task
                .pointer(&format!("/assertions/{dimension}"))
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    anyhow!("UI evaluation task {id:?} has no {dimension} assertions")
                })?;
            if assertions.is_empty() {
                bail!("UI evaluation task {id:?} has empty {dimension} assertions");
            }
        }
    }
    Ok(())
}

fn validate_expected_fixture_set(root: &Path, manifest: &FixtureManifest) -> Result<()> {
    let expected_root = root.join(EXPECTED_ROOT);
    let expected_ids = fs::read_dir(&expected_root)
        .with_context(|| format!("read expected UI snapshots {}", expected_root.display()))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .filter(|file_type| file_type.is_dir())
                .map(|_| entry.file_name().to_string_lossy().into_owned())
        })
        .collect::<BTreeSet<_>>();
    let fixture_ids = manifest
        .fixtures
        .iter()
        .map(|fixture| fixture.id.clone())
        .collect::<BTreeSet<_>>();
    if expected_ids != fixture_ids {
        bail!(
            "expected UI snapshot directories do not match the fixture manifest: expected={expected_ids:?}, fixtures={fixture_ids:?}"
        );
    }
    Ok(())
}

fn selected_fixture<'a>(manifest: &'a FixtureManifest, id: Option<&str>) -> Result<&'a UiFixture> {
    let id = id.ok_or_else(|| anyhow!("a fixture id is required"))?;
    manifest
        .fixtures
        .iter()
        .find(|fixture| fixture.id == id)
        .ok_or_else(|| {
            anyhow!("unknown UI fixture {id:?}; run `cargo xtask ui list --output-dir <path>`")
        })
}

fn list(manifest: &FixtureManifest, output_directory: &Path) -> Result<()> {
    write_json(&output_directory.join("manifest.json"), manifest)?;
    write_summary(
        output_directory,
        "list",
        "passed",
        json!({ "fixtures": manifest.fixtures }),
    )
}

fn audit_text(root: &Path, manifest: &FixtureManifest, arguments: &UiArguments) -> Result<()> {
    let fixtures: Vec<&UiFixture> = if arguments.all {
        manifest.fixtures.iter().collect()
    } else {
        vec![selected_fixture(manifest, arguments.fixture.as_deref())?]
    };
    let mut failed = Vec::new();
    for fixture in &fixtures {
        let output = arguments.output_directory.join(&fixture.id);
        capture(root, fixture, &output, None)?;
        let text = read_json(&output.join("text.json"))?;
        let semantic = read_json(&output.join("semantic.json"))?;
        let text_findings = text
            .get("audit")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("text capture for {:?} has no audit array", fixture.id))?;
        let semantic_findings = semantic
            .get("semantic_audit")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                anyhow!(
                    "semantic capture for {:?} has no semantic_audit array",
                    fixture.id
                )
            })?;
        if !text_findings.is_empty() || !semantic_findings.is_empty() {
            failed.push(fixture.id.clone());
        }
    }
    let status = if failed.is_empty() {
        "passed"
    } else {
        "failed"
    };
    write_summary(
        &arguments.output_directory,
        "audit-text",
        status,
        json!({ "fixture_count": fixtures.len(), "failed_fixtures": failed }),
    )?;
    if status == "failed" {
        bail!("UI text or semantic audit failed")
    }
    Ok(())
}

fn capture(
    root: &Path,
    fixture: &UiFixture,
    output_directory: &Path,
    expected_visual: Option<&Path>,
) -> Result<()> {
    recreate_directory(output_directory)?;
    let request = json!({
        "schema_version": SCHEMA_VERSION,
        "fixture": fixture,
        "output_directory": absolute(output_directory)?,
        "expected_visual": expected_visual.map(absolute).transpose()?,
    });
    let request_path = output_directory.join("request.json");
    write_json(&request_path, &request)?;
    let web_package = root.join("apps/polyorama-gallery/web/pkg/polyorama_gallery.js");
    if !web_package.is_file() {
        bail!("gallery web package is missing; run `cargo xtask build-web` before UI capture");
    }
    let temporary_directory = root.join(".tools/tmp");
    fs::create_dir_all(&temporary_directory)?;
    let status = Command::new("node")
        .arg(root.join("tools/ui-capture.mjs"))
        .arg("--request")
        .arg(absolute(&request_path)?)
        .env("TMPDIR", &temporary_directory)
        .status()
        .context("run deterministic UI capture")?;
    if !status.success() {
        bail!(
            "UI capture failed for {:?}; inspect {}",
            fixture.id,
            output_directory.join("logs").display()
        );
    }
    Ok(())
}

fn populate_failure_bundle(
    failure: &Path,
    expected: &Path,
    actual: &Path,
    visual_diff: &VisualDiff,
) -> Result<()> {
    for side in ["expected", "actual"] {
        fs::create_dir_all(failure.join(side))?;
    }
    for name in ["metadata.json", "semantic.json", "text.json", "visual.png"] {
        fs::copy(expected.join(name), failure.join("expected").join(name))?;
        fs::copy(actual.join(name), failure.join("actual").join(name))?;
    }
    fs::create_dir_all(failure.join("diff"))?;
    for name in ["metadata.json", "semantic.json", "text.json"] {
        let expected_value = read_json(&expected.join(name))?;
        let actual_value = read_json(&actual.join(name))?;
        write_json(
            &failure.join("diff").join(name),
            &json!({
                "equal": expected_value == actual_value,
                "expected": expected_value,
                "actual": actual_value
            }),
        )?;
    }
    fs::copy(
        actual.join("visual-diff.png"),
        failure.join("diff/visual.png"),
    )?;
    write_json(
        &failure.join("diff/visual.json"),
        &json!({
            "dimensions_equal": visual_diff.dimensions_equal,
            "differing_pixels": visual_diff.differing_pixels,
            "total_pixels": visual_diff.total_pixels,
            "pixel_tolerance": 0
        }),
    )?;
    copy_directory(&actual.join("logs"), &failure.join("logs"))?;
    write_json(
        &failure.join("summary.json"),
        &json!({ "schema_version": SCHEMA_VERSION, "status": "failed" }),
    )
}

fn write_missing_baseline_failure(failures: &Path, fixture: &UiFixture) -> Result<()> {
    let failure = failures.join(&fixture.id);
    fs::create_dir_all(failure.join("logs"))?;
    write_json(
        &failure.join("summary.json"),
        &json!({
            "schema_version": SCHEMA_VERSION,
            "status": "failed",
            "reason": "missing_checked_in_baseline",
            "fixture": fixture
        }),
    )?;
    fs::write(
        failure.join("logs/runner.log"),
        "Checked-in expected evidence is missing. Verification never creates or updates baselines.\n",
    )?;
    Ok(())
}

fn write_summary(
    output_directory: &Path,
    command: &str,
    status: &str,
    details: Value,
) -> Result<()> {
    let summary = json!({
        "schema_version": SCHEMA_VERSION,
        "command": command,
        "status": status,
        "details": details
    });
    write_json(&output_directory.join("summary.json"), &summary)?;
    println!("{}", serde_json::to_string(&summary)?);
    Ok(())
}

fn read_json(path: &Path) -> Result<Value> {
    serde_json::from_slice(
        &fs::read(path).with_context(|| format!("read JSON evidence {}", path.display()))?,
    )
    .with_context(|| format!("parse JSON evidence {}", path.display()))
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes).with_context(|| format!("write JSON evidence {}", path.display()))
}

fn recreate_directory(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)
            .with_context(|| format!("remove previous UI evidence {}", path.display()))?;
    }
    fs::create_dir_all(path)
        .with_context(|| format!("create UI evidence directory {}", path.display()))
}

fn copy_directory(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn absolute(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()?.join(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_in_fixture_manifest_is_typed_and_valid() {
        let manifest: FixtureManifest =
            serde_json::from_str(include_str!("../../docs/ui-snapshots/fixtures.json")).unwrap();
        validate_manifest(&manifest).unwrap();
        assert_eq!(manifest.fixtures.len(), 5);
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        validate_evaluation_seed(&root).unwrap();
    }

    #[test]
    fn output_directory_is_mandatory() {
        let error = parse_arguments(vec!["list".into()]).unwrap_err();
        assert!(error.to_string().contains("explicit --output-dir"));
    }

    #[test]
    fn render_requires_a_closed_fixture_selection() {
        let error = parse_arguments(vec!["render".into(), "--output-dir".into(), "out".into()])
            .unwrap_err();
        assert!(error.to_string().contains("requires --fixture"));
    }

    #[test]
    fn automatic_baseline_update_flag_is_rejected() {
        let error = parse_arguments(vec![
            "verify".into(),
            "--output-dir".into(),
            "out".into(),
            "--update".into(),
        ])
        .unwrap_err();
        assert!(error.to_string().contains("unknown ui argument"));
    }

    #[test]
    fn failure_bundle_contains_every_evidence_category() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("xtask-ui-bundle-test-{}", std::process::id()));
        let expected = root.join("expected");
        let actual = root.join("actual");
        let failure = root.join("failure");
        recreate_directory(&root).unwrap();
        fs::create_dir_all(actual.join("logs")).unwrap();
        fs::create_dir_all(&expected).unwrap();
        for name in ["metadata.json", "semantic.json", "text.json"] {
            write_json(&expected.join(name), &json!({ "side": "expected" })).unwrap();
            write_json(&actual.join(name), &json!({ "side": "actual" })).unwrap();
        }
        for path in [
            expected.join("visual.png"),
            actual.join("visual.png"),
            actual.join("visual-diff.png"),
        ] {
            fs::write(path, b"png").unwrap();
        }
        fs::write(actual.join("logs/runner.log"), b"evidence\n").unwrap();

        populate_failure_bundle(
            &failure,
            &expected,
            &actual,
            &VisualDiff {
                dimensions_equal: true,
                differing_pixels: 1,
                total_pixels: 4,
            },
        )
        .unwrap();

        for path in [
            "expected/visual.png",
            "actual/visual.png",
            "diff/visual.png",
            "diff/semantic.json",
            "diff/text.json",
            "logs/runner.log",
            "summary.json",
        ] {
            assert!(failure.join(path).is_file(), "missing {path}");
        }
        fs::remove_dir_all(root).unwrap();
    }
}
