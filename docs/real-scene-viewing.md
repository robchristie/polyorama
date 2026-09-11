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
