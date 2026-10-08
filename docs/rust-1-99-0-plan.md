# Rust 1.99.0 maintenance

Status: active
Next action: Qualify the committed compiler selection through the complete owner contract and existing deterministic viewer journeys.

## Outcome and scope

Portfolio package `rust-1.99.0` / P1 selects exact Rust 1.99.0 for active
Polyorama development, builds and normal CI. The root toolchain, production
browser guard and maintained application/consumer setup guidance move together.
The Rust 1.97.1 workspace and independent Record Desk minimums, both Cargo
lockfiles, editions, resolver, warning policy, architecture and admission/resource
limits remain. Necessary compiler repairs are included only when demonstrated.
Historical plans, measurements and compiler identities retain their attribution.
No deployment, release, package/data publication, protected-resource operation,
default-toolchain change, optional-tool update or unrelated product work is included.

The product starts at `8b2bb966d91191efdb47ff16c4cb71797bd2fea8` through
its canonical workspace. The immutable
[portfolio checkpoint](https://github.com/robchristie/portfolio-workspace/blob/f5417357b169b3cd037a340d459e2553503a409e/docs/campaigns/rust-1.99.0.md)
defines the authorised package. E1 is terminal through its
[completed plan](https://github.com/emuella/emuella-workspace/blob/f3b9e56a3ec83d32dc808c52596f02f419df1f70/docs/plans/completed/rust-1-99-0.md)
and [terminal receipt](https://github.com/emuella/emuella-workspace/pull/103#issuecomment-6052599947).
There is no demonstrated dependency/API change: the existing codec pin
`6586e3d50f95429b242cb2e3535742b002784f2d` remains.

## Acceptance and sequence

1. Inspect active selectors, actual compiler dispatch and applicable maintained
   qualification guidance; commit one coherent compiler candidate.
2. Run `cargo xtask verify` with actual Rust 1.99.0, including native/WASM lint,
   tests, release builds, production packaging, snapshots, native and browser
   smokes and independent Record Desk consumer workflows. Old-compiler binaries
   and receipts do not qualify this compiler.
3. Reproduce the previous public deterministic input bytes from their retained
   recipe and hash-check their immutable identities; run the nine-input native
   journey and unchanged frozen six-input browser pressure gate. Preserve every
   existing admission/cancellation/eviction predicate and qualification limit.
4. Open and exercise this worktree's compatible private HTTPS preview under the
   installed launcher, retaining readiness separately from actual browser proof;
   stop only the owned generation.
5. Reconcile the verified owner state, obtain independent review of the exact
   candidate, pass public CI, then squash-merge and await actual post-merge CI.
   The owning PR retains review, merge identities and four-state/scratch cleanup.

The [previous compiler evidence](rust-toolchain-evidence/README.md) supplies
applicable fixture recipes and environment knowledge. Its Rust 1.98.1 binaries
and acceptance results are historical. Existing representative/performance and
protected-image plans retain their original limits and incomplete outcomes;
this package makes only the bounded public functional claim.
