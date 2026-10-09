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
| Preserve the canonical check inventory and split executable stages | Verifier | In progress |
| Normalise runner toolchains and give parallel jobs appropriate caches | Workflow | Pending |
| Probe combined native compilation and profile trade-offs | Calibration | Pending |
| Qualify the complete hosted surface and cached elapsed time | CI | Pending |
| Review the exact candidate, land and verify the merge revision | Owning PR | Pending |

Next action: implement and regression-test the shared verification stages, then measure their hosted critical path.
