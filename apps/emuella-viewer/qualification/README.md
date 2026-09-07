# Frozen local proof limits

The native and browser JSON files freeze the initial composed workload limits
before final qualification. They bind workload
`de91f9293bae59fe30db79feb44e620d28f0c063a0ecf9f91984690c97bc2e02`
and the five retained baseline trace hashes for each runtime. The workload has
one 43008² image, eight additional image identities, 10000 logical detections,
clustered and scattered gallery visits, detail openings and five bookmarks.

The baselines used a Ryzen 9 9950X3D desktop, loopback HTTP and hardware NVIDIA
rendering. Native reported an RTX 3090; the browser reported NVIDIA/Ampere but
withheld its device name. No browser model-specific GPU claim follows. With five
samples, nearest-rank p95 is simply the observed maximum. The 25% allowance
accommodates local scheduling variation and final measurement instrumentation;
it is not a statistical confidence interval or a remote-service target.

| Measurement | Native maximum | Browser maximum |
|---|---:|---:|
| First primary-view region resident | 110 ms | 240 ms |
| Complete overview and visible gallery | 4550 ms | 20290 ms |
| Detection detail completion | 160 ms | 380 ms |
| Visible gallery completion after scroll | 870 ms | 2330 ms |
| Warm revisit completion | 90 ms | 460 ms |
| Compressed-cache reconstruction | 4810 ms | 19470 ms |
| Native process peak RSS | 232 MiB | Not applicable |
| Worker WASM linear memory | Not applicable | 41 MiB |

The first-useful boundary explicitly means a primary-view region. The baseline
export used broader wording, but its dispatch order schedules visible primary
regions at discard 6 before gallery regions at discard 2. Final instrumentation
must test primary residency directly. Residency does not measure display scanout.

Shared caps remain 64 MiB compressed databins, 16 MiB descriptor/index metadata,
16 MiB decoded data, 64 MiB GPU textures and 64 MiB codec workspace, with one
active codec worker. These are global application caps across sources and panes.
Peak process memory includes other allocations; WASM linear memory alone does
not measure the browser process group. Browser process memory, transport
recovery and preparation limits need their own evidence before complete proof
qualification. Adding missing bounds before final qualification must not relax
these existing limits.

The trace admission command checks identities, bounds and event presence. The
integration runner must additionally enforce the actual phase order, regional
correctness, visible scheduling, compressed/GPU reuse and recovery observations.
A passing JSON admission result alone is not an architectural proof. Retain
failed runs and compare final exact-build observations with these unchanged
limits. Source-storage coldness is unknown on the supplied NFS mounts; do not
infer physical server reads from process counters or returned raster pixels.
