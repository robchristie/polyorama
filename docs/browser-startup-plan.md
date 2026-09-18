# Browser startup efficiency

Status: active

Starting revision: `dc7d3d6a89de0f070eade59bf7da7522de8b57f8` (actual clean checkout).
Owner: current implementation task, branch `codex/browser-startup-efficiency`.
Scope: this repository's Lab, Gallery, viewer, build/hosting tools and consumer recipe.
No consumer repositories, codecs, datasets or release/deployment operations change.

## Acceptance and dependency order

1. Define finite host/application/worker milestones and capture an instrumented,
   otherwise unoptimised baseline. Validate usable workspace through real input.
2. Add deterministic pinned post-bindgen packaging, compression and versioned
   static delivery; retain deterministic no-store smoke servers.
3. Compare unchanged, size and speed candidates using identical workload and
   delivery. Audit test-only exports; retain gating only for measured benefit.
4. Select at most one structural optimisation if traces justify it; otherwise
   document deferral. Preserve native, rendering, worker and codec contracts.
5. Verify final packages, failures, root/subpath, input, native/WASM and canonical
   checks. Independently review exact head, land, check post-merge CI and clean up.

## Calibration

Question: which build/delivery changes reduce usable-workspace and first-content
latency without buying an unaccepted regression in runtime or first image?
Smallest probe: Lab default 1440×900 workspace, Gallery default shell, existing
viewer deterministic fixture. Lab is primary; five fresh-process cold/repeat
pairs per delivery profile at baseline/final; smaller candidate screens.
Profiles: unthrottled local and fixed 10 Mbit/s down/up, 40 ms request latency.
Evidence owner: `docs/browser-startup-evidence/`, raw machine-readable samples;
builds/logs/screen images in ignored `.tools/runtime/browser-startup/`.
Exit: establish baseline variability and declare numeric gates before judging
candidates. Select one coherent final configuration or reject unsupported changes.

## State

Baseline captured: five Lab/Gallery cold/repeat pairs per profile. Raw samples
and predeclared gates are in `browser-startup-evidence/`. Compression-only and
Oz/O3 screens completed against identical baseline WASM. Lab test-export gating
rejected at 0.884% Brotli saving (<1% declared gate) and reverted.
Oz selected provisionally: ~3.2% smaller Brotli Lab WASM; O3's further <0.1%
compressed saving does not justify its slower observed viewer cold screen.
Viewer output hashes/resource bounds matched across 176 decoded-region
observations per variant. No structural change justified: construction is ~1ms;
remaining dominant local work lies in framework/GPU startup outside this scope.
Viewer worker assets reuse the cached package; no duplicate-transfer claim.

Next action: build final isolated production artifact, run five-pair finalist and failure/subpath checks, then canonical verification and exact-head landing.
