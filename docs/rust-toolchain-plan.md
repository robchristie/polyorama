# Rust 1.98.1 maintenance

Status: active
Next action: qualify the pinned compiler, retained minimum and viewer integration.

## Outcome and scope

Programme package `rust-1.98.1` / P1 selects exact Rust 1.98.1 for Polyorama
development, normal CI and the production browser compiler contract. The
authorised maintenance includes selectors, maintained setup guidance, necessary
compiler repairs and existing native/WASM/browser/viewer acceptance. It preserves
behaviour, public APIs, lints, dependencies, editions and resolver policy.
No release, deployment, package/data publication, protected-resource operation,
machine/default-toolchain change or other-project write is included.

The starting product is `e1f31e11bc3210e763cea1b1cbd5f23c02356db4` (tree
`d654955c046686dc6dbc270c6c42acfb4dcc3f53`), entered through the registered
Polyorama workspace. The programme prerequisite is
[portfolio baseline PR #3](https://github.com/robchristie/portfolio-workspace/pull/3).
[E1's completed owner plan](https://github.com/emuella/emuella-workspace/blob/9cb91a253e38111348b925c52f18debde8671aa4/docs/plans/completed/rust-1-98-1.md)
and [terminal audit](https://github.com/emuella/emuella-workspace/pull/101#issuecomment-5911765234)
establish the upstream migration. E1 requires no API or consumer-lock change;
Polyorama retains codec revision `6586e3d50f95429b242cb2e3535742b002784f2d`.

## Inventory and compatibility

The eleven-member workspace initially has no root or scoped toolchain file.
Normal CI and the production browser guard are the operational 1.97.1 selectors.
This package adds a root rustup pin with Rustfmt, Clippy and the WASM target and
updates those two selectors plus README, browser startup and CI cache guidance.
The root Cargo minimum stays 1.97.1. Nine members inherit it;
`emuella-viewer-source` and `emuella-viewer-tools` retain their omitted declarations.
Historical measurements, qualification reports, archived build identities and
benchmark/provenance references remain attributed to their original compilers.

## Acceptance and sequence

1. Inspect all operational selectors and actual compiler dispatch; use focused
   formatting, xtask tests and native/WASM lint feedback for necessary repairs.
2. Run the existing complete `cargo xtask verify` contract under 1.98.1,
   retaining actual logs and generated verification observations. Historical
   1.97.1 PR #42 evidence does not qualify this compiler.
3. Establish applicable 1.97.1 native/WASM compatibility for changed source
   inputs and the retained lock, then exercise the existing deterministic viewer
   service through native and real browser-worker journeys on the candidate.
4. Complete the plan and programme disposition in the product PR, obtain
   independent exact-head review, then land with CI and post-merge CI. The PR's
   landing comment owns future merge identities and four-state/scratch cleanup.

Dependency order is E1 terminal evidence → P1 compiler/compatibility qualification
→ viewer consumer proof → reviewed product landing → portfolio reconciliation.
Polyorama owns every product write and long-running check/CI watcher.

## Evidence

Verification results and input identities will be retained under
`docs/rust-toolchain-evidence/`; the owning PR supplies review and landing facts.
The package closes only when all acceptance above is attributable to its final
candidate and required post-merge checks pass.
