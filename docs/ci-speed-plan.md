# CI feedback time

Status: active

## Outcome and acceptance

Reduce feature-delivery CI latency towards a maximum of five minutes for cached
GitHub-hosted runs. Preserve the complete canonical verification surface and the
required `verify` check, including prose-only routing. Implement stable runner
toolchain selection, combined native application/example compilation and
parallel qualification. Profile changes require representative runtime evidence.

## Calibration

The smallest probe compares native build target selection using the pinned
Rust 1.99.0 toolchain, then exercises the parallel verifier on a fresh hosted
runner. The existing cached 1.98.1 run 37574216107 took 20m39s; it attributes
3m24s to workspace native release and another 1m20s to the example release.
Those older measurements guide partitioning but do not qualify Rust 1.99.0.
The successful 1.99.0 baseline is run 37769182622 on
`90c1d0baba78a0c68a488943f858763a36762cf2`; dependency recompilation makes its
30m28s job unsuitable as a warm comparison.

The owning PR retains exact run, cache, compiler and elapsed-span evidence.
Source and assurance invariants belong in verifier regressions; operating
guidance belongs in `docs/ci-cache.md`. Select a candidate only after the full
surface passes. Compare an unchanged fresh-run warm attempt and a representative
source change; report measured limits without turning a timing target into a
test-skipping rule.

## Delivery

| Phase | Owner | Status |
| --- | --- | --- |
| Preserve the canonical check inventory and split executable stages | Verifier | Complete |
| Normalise runner toolchains and give parallel jobs appropriate caches | Workflow | Complete |
| Probe combined native compilation and profile trade-offs | Calibration | Complete for native target selection; retain the current release profile pending warm hosted timing |
| Qualify the complete hosted surface and cached elapsed time | CI | In progress |
| Review the exact candidate, land and verify the merge revision | Owning PR | Pending |

The committed `d2b359e82eab876a313eed6c6985ea4c041a74fa` native-Lab stage passed
the batched release build, both existing native smokes and its interface journey
locally in 89.53s. This screens the partition; it is not hosted timing proof.
Hosted calibration run 37865870558 exposed missing `ripgrep` in the new native
UI setup after the icon interaction check passed. Add it to every UI lane and
retain the runtime-log scan. That failed stage cannot populate its archive;
successful lanes may populate their provisional PR-scoped dependency archives.

Next action: qualify the complete warm workflow after replacing fixed native startup sleeps with bounded window readiness, then repeat it unchanged before final review.

The partially warm repaired run 37866800165 passed checks (3m36s), native Lab
(3m55s), production (3m19s) and UI (2m55s); native-other and Record Desk were
still cold. Existing browser traversal raced slow input frames. Gallery's
failure retained repeated frame observations while the helper continued sending
Tab; Record Desk's failure retained five Tab keydowns followed by Enter on a
different focused node. A bounded local probe found no target-focus assertion
contradicting egui's current focused ID. The selected repair acknowledges each
Tab through a monotonic application diagnostic before another key, preserving
all focus/activation assertions and traversal limits. Both focused physical
browser journeys and the delayed-receipt regressions passed locally. Temporary
probe fields/logging were removed. Keep release profiles unchanged.

The changed-source warm run 37869437618 passed six complete lanes, with a
maximum lane duration of 4m04s. Native Lab failed before its window became
visible: the legacy smoke searched once after a fixed five-second sleep while
its runtime log still showed graphics startup. The selected repair waits up to
30 seconds for the owned application's visible window, fails if its process
exits, and keeps all existing interaction/persistence assertions. Gallery uses
the same helper. Readiness regressions cover delayed visibility, timeout and
application exit.

Run 37870185539 exposed the same queued-Tab condition in the retained icon
workflow's separate fixed-delay traversal. Icon, navigation and Record Desk
now share the receipt-aware helper while retaining their respective 40, 45
and 24 action limits and 60/60/50 ms settling intervals. The helper observes
the final allowed action. The focused icon journey and all caller-bound
regressions passed locally.
