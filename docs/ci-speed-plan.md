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

Next action: repair hosted prerequisites, then compare an unchanged fresh-run warm attempt and a changed-source warm run against the complete five-minute workflow target.
