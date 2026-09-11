# Real-scene viewer calibration

This bounded B increment binds the viewer to merged codec
`dc8ff1f432e132c0fddbbd94d8bcccfd6f5cf7db`. The frozen campaign plan belongs to
the private workspace at checkpoint `1c469f7f21ef1355e47ef33a6a8b2e8ad605aa35`.
The source-coordinate freeze is [the factual JSON record](real-scene-viewing-source-views.json),
committed before compressed output inspection. Calibration remains provisional.

## Source and quality

`tools/viewer-real-scene-source.py` verifies the reviewed notice and prepared
manifest identities, reads original TIFFs in bounded strips, and derives exact
per-band nearest-rank 2nd/98th percentiles excluding nodata. It retains source
range and nodata facts without treating UInt16 storage as sensor precision.
The JSON freeze specifies four 256-square views per selected product and both
full-storage and percentile stretches, at 1:1 and arithmetic 4× reduction.
The reduced numerical comparison uses the same box average on original and
reconstructed full-resolution samples; it does not conflate that display
comparison with DWT-discard reconstruction performance.

The locally opened Mansfield object views contain small aircraft. The initially
labelled Boca PAN object view contains vehicles and their shadows; that factual
classification corrects the label without changing coordinates or thresholds.
Tok's fixed object candidate view contains small bright roadside/yard features;
no aircraft identity is asserted there. Nodata is present at Boca and absent
from the selected complete Mansfield/Tok products.

`tools/viewer-real-scene-prepare.py` invokes the existing native GTiff route on
the complete original source, with PAN band 1, RGB16 MS bands 5,3,2 and RGB8
PS-RGB bands 1,2,3. It records command, binary/library hashes, source identity,
wall time, callback bytes, RSS, encoded payload and every descriptor hash.
No raw input bridge is used. The new explicit `--retain-incomplete true` option
preserves failed preparation output for stores that prohibit deletion; default
preparation retains its existing cleanup behaviour. Failed groups are never reused.

`tools/viewer-real-scene-quality.py` uses bounded `reference-export` windows and
original TIFF bands. Source-domain errors are per-band with storage peaks,
observed ranges and separate nodata/valid subsets. Display errors and clipping
are separate at both stretches/scales; every fixed view and band must meet
RMSE ≤3 and 99th-percentile absolute error ≤12 in U8 display units. This is
same-codec direct-file reconstruction, not independent decoder evidence.
Opened reconstructed images, independent decoder agreement, human acceptance
and analytical/ML suitability are separate observations.

All image payloads, reference samples and captures stay in fresh
`real-scene-viewing-viewer-*` children of the existing approved RarePlanes store.
Each group retains the unchanged notice, attribution and modification/lineage
record. Only scripts, coordinates and numerical evidence belong in Git.

## Application workload

Native `--workload PATH` and the browser harness's optional third argument
install the same bounded JSON action sequence before its first action. The
inherited workload remains the default. The real-scene workload cycles the two
primary product identities while retaining five bookmarks, pan/zoom, clustered
and scattered detections, gallery movement, warm compressed reconstruction,
warm GPU revisits and simultaneous distinct image views.

`viewer-composed-journey.py --workload PATH --catalogue-contract PATH` requires
an exact ordered `sources` array of source SHA-256, bands, width and height.
It then checks complete observed primary-view residency for the full geometry;
the inherited default still requires nine parents and 121 large overview chunks.
The trace hashes the actual workload and retains the catalogue contract. Global
budgets, one worker, virtualisation, phase deadlines, stale-result and recovery
behaviour remain inherited from the [composed contract](emuella-viewer-calibration.md).

Development evaluates exactly 1, 2 and 4 bpp on Mansfield. Five native and five
actual hardware-browser development journeys precede the application threshold
freeze. A validation failure cannot retune the thresholds. If no rate passes
quality, retain an explicit rejection and measure application limitations on a
clearly identified rejected diagnostic representation; do not call that validation.
C remains unselected: entropy, synthesis, requested output and repeated-demand
observations inform the coordinator's later choice, without an optimisation here.

## Measured B result

**Reject the existing HT profile for the frozen real-scene display criteria.**
No common development rate passes PAN16 and RGB16. These are full original
scenes, not substituted crops; only the predeclared quality observations are
regional. The [numerical evidence](real-scene-viewing-evidence.json) binds source,
binaries, preparation, traces, complete references and protected evidence hashes.

| Development product | Rate (bpp) | Preparation wall (s) | Payload + descriptors (bytes) | Worst display RMSE / p99 (U8) | Result |
|---|---:|---:|---:|---:|---|
| Mansfield PAN16 | 1 | 1.866 | 1,694,558 | 15.42 / 41 | Reject |
| Mansfield PAN16 | 2 | 2.517 | 3,364,384 | 8.91 / 23 | Reject |
| Mansfield PAN16 | 4 | 2.867 | 6,703,018 | 2.75 / 7 | Numerical pass |
| Mansfield RGB16 | 1 | 0.314 | 108,761 | 42.43 / 112 | Reject |
| Mansfield RGB16 | 2 | 0.364 | 213,312 | 27.16 / 73 | Reject |
| Mansfield RGB16 | 4 | 0.414 | 422,195 | 18.70 / 50 | Reject |

Preparation peaks were 58,504–69,064 KiB process RSS on development. Complete
Boca PAN/RGB16 preparation at the rejected diagnostic 4 bpp rate took 3.418/0.564 s,
with 7,775,401/528,445 payload-plus-descriptor bytes and 99,136/73,936 KiB RSS.
Complete Tok RGB8 took 8.224 s, 8,491,936 bytes and 98,548 KiB RSS. These are
individual uninstrumented invocations with uncontrolled source cache state,
not statistical performance claims. Original hashing, GTiff callbacks, process
read counters and encoded bytes retain their distinct boundaries in the report.

Five native and five actual hardware-browser development runs completed all
30 phases without workload or global budget failures on the rejected 4 bpp pair.
The five earlier native attempts failed before rendering because a Vulkan loader
was absent from the library path; all remain retained. Adding the existing
Chromium Vulkan loader enabled NVIDIA RTX 3090/Vulkan native rendering. Chromium
151.0.7922.34 reported NVIDIA/Ampere without a device model. No software GPU result
is used as hardware evidence. The exact final native/browser runtime paths and
loader dependencies belong to the retained build/setup identity.

The application ceilings were frozen at `79038ea` after those ten runs and before
reserved scene execution, using no more than 1.25 times each development maximum
and preserving stricter inherited ceilings. Boca native/browser diagnostics meet
them. Tok native/browser complete but fail the frozen detail ceilings:
296.04 > 133.59 ms native and 334.50 > 168.50 ms browser. No rate or ceiling was
retuned. Since development rejected RGB16 at every rate, these reserved executions
are diagnostics, not validation of a selected display configuration.

All 36 independent Kakadu view invocations completed with matching dimensions
and explicit original precision. They differ from the viewer's complete-tile
reference by at most one stored code value; byte-exact independent agreement is
not claimed. Boca RGB16 also changes source nodata-zero samples: fixed nodata subsets show
maximum absolute errors up to 573 storage codes. The source TIFF and its nodata
metadata remain unchanged, but the lossy representation has no separate exact
nodata mask. Exact nodata preservation is therefore not established.

Same-codec direct-file reference checks match all 500 distinct
retained application records over 12,830,508 pixels across five representations.
This does not turn FNV checksums into exhaustive per-pixel comparison evidence.

Recovery observed real interrupted receipt, additional-request retry, reconnection,
cancellation acknowledgement and rejection of a delayed actual completion on
both development and Boca. The initial Boca probe is retained as failed: later
overlapping requests had already supplied the interrupted cache data before
Retry. The adapted probe holds subsequent JPP traffic offline until explicit
Retry and cycles the actual catalogue. All inherited recovery ceilings and the
mandatory actual-eviction condition remain unchanged. Overall recovery still
**fails**: the two-image pressure journeys did not exhaust the 1/4/16 MiB budgets,
so neither representation nor GPU eviction occurred. This is an unsupported
pressure case, not a waived gate or an application crash.

Opened source and reconstructed object views show retained PAN aircraft detail
at 4 bpp and colour distortion/ringing in RGB16, strongest at 1/2 bpp. Opened
actual browser final captures show the complete scene shapes and original Boca
nodata, but the UInt16 full-storage display is very dark. These observations do
not establish exhaustive per-view visual acceptance, human acceptance or
analytical/ML suitability. The numeric failure alone is sufficient for rejection.

## C observations, without implementation

Warm compressed replay repeats exactly 30 native or 24 browser regions, with no
new JPP or descriptor bytes. It repeats all entropy and synthesis work; a warm
GPU revisit adds neither decode nor upload. Native warm replay requests 162,052
output samples while decoding 1,083,652 entropy coefficients and loading 287,470
synthesis coefficients (6.69× and 1.77× output). Browser requests 137,476 samples,
with ratios 5.38× and 1.69×. Legitimate wavelet support is not labelled a defect.

Across the five native runs, worker execution is 73.25–75.90 ms while whole-view
settlement takes 626.82–683.59 ms. Browser worker execution is 65.10–69.30 ms while
settlement takes 400.30–400.70 ms. The dominant measured elapsed boundary is
outside worker execution. Frame-paced serial dispatch/presentation is a concrete
next probe, while repeated reconstruction is also directly evidenced. Separate
entropy and synthesis wall times and physical GPU execution timing are unavailable;
the residual cannot all be attributed to one mechanism from these counters.
No C candidate was selected or implemented by this worker.

## Verification and hand-off

Full `cargo xtask verify` passed: formatting, native/WASM Clippy, workspace tests,
architecture, release native/WASM builds, both browser smoke suites, five frozen
UI fixtures and native lab/gallery smoke. The source report retains the exact
command, successful log hash and earlier environment failures. Canonical
GL/llvmpipe smoke remains separate from B's actual NVIDIA measurements.
An isolated bind mount keeps the canonical logical `.tools` boundary while
retaining captures in the approved persistent store; existing UI path and ownership
checks were not weakened. Generated web package paths may be symlinks to the
registered build store and remain ignored.

The original benchmark owner at `f78c9c4edc2c606a0037b1445753830781726646` assessed
all 22 traces with its actual `journey` CLI. The ten successful development traces
are admitted calibration-only (exit 4 without frozen thresholds). Boca normal
traces are application-qualified (exit 0). Tok normal traces are admitted but
unqualified for detail latency; recovery remains unqualified for actual eviction.
The five initial native startup failures remain separate failed attempts.
All nine immutable representations retained identical payload and descriptor
hashes after viewing.

Preparation binds the installed GDAL 3.13.0 library by SHA-256; no maintained-GDAL
source-revision equivalence is asserted for that installed binary. The GTiff path
does not require the NITF plugin. Timed native binaries are retained separately
from canonical workspace builds, whose feature unification changes binary hashes.
No C implementation, independent review, Git publication, merge or terminal
campaign delivery was performed by this bounded worker.
