# Emuella viewer candidate qualification

The public deterministic native/browser proof passes its frozen preparation,
latency, resource and recovery limits. This is candidate qualification, not a
claim that the complete satellite-image engineering objective has landed.
Representative real-image viewing preparation remains unproved pending the
system campaign's approved output-store and derivative-cleanup decision.

The measured application is exact clean revision
`a40c71f34ed4e6c93a899f01b7731ec263dd0cc2`, tree
`9754fc8876ef73fdb05923be8e6327a2c50124d8`. Codec and protocol use merged
`3afcfabb24282645c3e101ab3495810d28212dfd`; benchmark admission uses merged
`3bdaa98449e2c4b6c0500a1ac9daedb4d61d92bc`. Full `cargo xtask verify` passed at
that application revision. Its canonical software-renderer observations are
separate from the actual NVIDIA qualification below. The additive reference
command was subsequently exercised at clean committed
`e5df4339954be588cd4ec0e959ab7c44df417e17`, without changing viewer execution.

## Reproduction and evidence

Use the [calibration and workload commands](emuella-viewer-calibration.md),
[preparation/service contract](emuella-viewer-service.md) and
[application commands](../apps/emuella-viewer/README.md). Prepare a new output
directory with `tools/viewer-prepare-workload.py`; start the service with the nine
resulting representations; run five native and five browser composed journeys
with the corresponding frozen files under `apps/emuella-viewer/qualification/`.
Recovery has its own workload and threshold identity. Do not apply normal
browser latency requirements to the recovery workload.

The [qualification report](emuella-viewer-evidence/qualified-public/qualification.json)
binds all eleven traces and benchmark admissions, frozen threshold hashes,
input identities and actual build/runtime observations. Its
[manifest](emuella-viewer-evidence/qualified-public/manifest.json) binds retained
file bytes. Large intermediate histories are hash-indexed, with retention limits
explicit; the retained final snapshots and bounded event/checksum records do not
claim a complete event history. The evidence contains only authored public
signals, including the visual captures.

## Observations

Nine parents cover U11/U16 greyscale and RGB8/RGB16. The large parent is
43,008 × 43,008 U11, tiled at 512 with six decompositions and one actual quality
layer. Preparation consumed 7,056 tile callbacks and 3,699,376,128 native sample
bytes in 220.838 seconds, with 5,364 KiB peak RSS. The representation occupies
441,844,578 bytes plus 2,992,977 descriptor bytes. These callback bytes are
synthetic native samples, not original compressed-storage traffic.

Five runs per runtime passed every unchanged frozen admission. The host has a
Ryzen 9 9950X3D, 32 logical CPUs and 91 GiB RAM. Native reported RTX 3090 Vulkan,
NVIDIA driver 610.43.03. Chromium 151.0.7922.34 reported NVIDIA/Ampere and withheld
its device model; the native model is not inferred for the browser.

| Maximum observation | Native | Browser |
|---|---:|---:|
| First primary region resident | 87.39 ms | 190 ms |
| Complete overview and visible gallery | 3,917.18 ms | 15,139.30 ms |
| Detection detail | 128.46 ms | 257.80 ms |
| Gallery completion | 736.18 ms | 1,881.40 ms |
| Warm revisit | 62.11 ms | 369 ms |
| Compressed-cache reconstruction | 3,966.66 ms | 14,764.60 ms |

The complete overview requires 121 primary regions; first-region residency is
not complete-image readiness or display scanout. Five observations provide an
observed maximum, not a population tail estimate. Warm compressed reconstruction
still performs decode and synthesis; its cost makes reduced reconstruction work
a useful next optimisation target.

Recovery passed interrupted reception, retry, connection loss/reconnect, delayed
actual completion rejection, cancellation acknowledgement and eviction. Three
Worker creation events belong to sequential fresh contexts; concurrency remains
one. The separate 1 MiB compressed-cache probe completed the overview and observed
3,654 bin evictions from the demanded parent. Its extra zero-representation-
evictions assertion failed because eight unused catalogue registrations were
also discarded. The [failed assertion and corroboration](emuella-viewer-evidence/qualified-public/same-source-lru/assessment.json)
remain retained; no run was replaced or threshold relaxed.

## Correctness and visual boundary

The [complete-reference report](emuella-viewer-evidence/complete-reference/comparisons.json)
records 18 passing groups: 807 application records, 12,109,360 compared pixels and
zero checksum/dimension/precision mismatches. It decodes complete selected tiles
from the immutable file and crops, bypassing JPP and client cache assembly. This
shares codec algorithms; codec-owned tests separately compare sparse outputs
with an independent full-tile reconstruction path. The reference command's tests
also compare actual sample arrays in 224 cases across four formats, eight windows
and all D6 discard levels, including odd edges and tile crossings.

Native and browser agree for 352 shared retained FNV-1a U16LE records. Browser
has 103 additional records. FNV is a diagnostic comparison, not a cryptographic
image identity or a guarantee against collisions. This evidence supports exact
agreement for the selected native output profile; it does not establish a
universal tolerance for other codecs, representations or browsers.

Opened captures show retained coarse imagery during a held detail response,
small authored bright objects, faint perturbations and crossing lines under
aggressive stretch. Stretch caused no extra compressed-data request or decode.
Visual judgement is limited to these synthetic patterns; satellite object quality
and sensor-specific stretch remain outstanding.

## Implemented and remaining scope

The application implements multiple sources and panes, bounded regional demands,
10,000 logical clustered/scattered detections, shared compressed/decoded/GPU
budgets, parent-based thumbnail/detail reconstruction and actual stateless JPP
cache semantics. Global texture accounting is distinct from physical GPU memory;
GPU completion timing is unavailable. Browser process memory uses sampled RSS
and observed per-process high-water sums with documented shared-page and sampling
limitations. OS/NFS storage cache state was uncontrolled; warm-server and empty
client states are explicit and do not imply cold physical original storage.

The system campaign still needs representative NITF-to-viewing preparation and
real-image visual evidence, final consumer review/landing, and merged-consumer
qualification. The GDAL owner separately records real U11 NITF regional reads;
those do not substitute for a complete viewing-preparation journey here.
A useful next Emuella increment is persistent Part 1 index reuse through the C ABI
for NITF preparation, followed by reducing repeated synthesis on compressed-cache
revisits. Eventual Geometis integration should consume these independent regional
and immutable-identity contracts after the remaining proof is complete.
Geometis implementation inspection, migration, deployment and publication remain
outside this demonstration. Additional JPIP optional features, spatially varying
compression, standalone chips and geospatial reprojection are deferred.
