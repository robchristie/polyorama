# Rust 1.99.0 qualification observations

These observations own portfolio package P1. Actual executable source is clean
commit `830eb7d7334f3b857437afd5fc3f158f545cd463`, tree
`61d868d5e2178dae1d34ea59c50007fe2baacd75`, based on
`8b2bb966d91191efdb47ff16c4cb71797bd2fea8`. Later plan/evidence-only commits
preserve the executable inputs. Production manifests retain their actual build
revision; they do not relabel an earlier build. [PR #54](https://github.com/robchristie/polyorama/pull/54)
owns exact-head review, applicable public CI, merge and post-merge CI.

[observations.tar.gz](observations.tar.gz) retains 306 actual receipts, complete
logs, public fixture recipes/hashes, native/browser snapshots, canonical
structured observations and selected opened captures. [archive-index.json](archive-index.json)
binds each file's size/SHA-256 and the archive digest. Extract into an inspection
directory with `tar -xzf docs/rust-1-99-0-evidence/observations.tar.gz -C <directory>`.
No fixture payload, executable, browser profile, cookie or protected input is
published. Temporary wrappers describe this host's installed bundled-library
process environment; they are observations, not a new maintained runner.

| Requirement | Actual result | Full retained evidence |
|---|---|---|
| Actual compiler | rustc/Cargo 1.99.0; rustc `b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`, Linux x86-64; four Cargo jobs, Node 25.8.2, npm 11.11.1, bindgen 0.2.127 | [canonical receipt](canonical-headful.json), `canonical-headful.log` |
| Complete owner contract | `cargo xtask verify`, exit 0, 334.615 seconds; native/WASM lint/tests/releases, architecture/API docs, startup, 14 UI snapshots and native/browser physical workflows, including independent Record Desk | `canonical-headful.log`, `canonical-generated/` |
| Exact production guard | Actual Rust 1.98.1 rejected before staging by `RUSTUP_TOOLCHAIN=1.98.1 target/debug/xtask build-browser-production`; the xtask binary is the new candidate. Actual 1.99.0 production packaging and response/startup tests pass | `guard-negative.log`, `production-manifest.json`, canonical log |
| Immutable public inputs | All nine reproduced manifests/payloads/descriptors, 754 immutable files, match the prior retained hashes exactly. Preparation timing/RSS receipts are newly measured | [comparison](fixture-comparison.json), `fixtures/generation.json` |
| Native viewer | Nine original mixed grey/RGB identities; all 30 stages settle, desired = ready, no in-flight work/errors; Mesa llvmpipe/OpenGL | `repaired-native-viewer/identity.json`, `native.json`, `native.json.stages.json`, `native.log` |
| Unchanged full browser gate | Frozen six-image cohort retains original indices 0,1,3,5,7,8 and immutable identities; 16 ordinary states, one actual Worker, rendered regions, stretch reuse/bookmarks/comparison; pressure gate: 62 GPU evictions, four representation evictions, two aborts, one interrupted actual JPP transfer | `pressure-cohort.json`, `viewer-browser/identity.json`, `viewer-browser/browser/viewer-browser.json`, `viewer-browser-pressure.json`, complete logs |
| Private preview | Exact run/readiness and all nine fetched asset hashes/MIME/no-store matched; Chrome 154.0.8037.95, Apple Metal-3, secure context, opened 1440×900 useful pixels; physical 70×40 drag moved both linked camera centres from (65536,65536) to (47616,55296) | [bounded receipt](private-preview-receipt.json), `mac-lab-before.png`, `mac-lab-panned.png` |

The constrained final browser has 388 completions, no errors, GPU peak
16,777,216 bytes (16 MiB cap), decoded peak 3,145,728 bytes (4 MiB cap), 104
intercepted actual JPP responses and one interrupted transfer. The actual
browser adapter is SwiftShader. These functional public fixture results do not
establish target-GPU performance, the representative 43,008-wide workload,
protected real-image qualification, codec quality or unrestricted browser/input
support. Existing limits, failed historical results and stopping rules remain.

## Failed observations and bounded environment repair

The first launcher attempt failed before canonical execution because the
bundled-bin mount hid `/usr/bin/env`. Subsequent existing CLI-installer mocks
required ordinary Bash/cat/chmod within their restricted PATH. Own sysroot links
restore those installed tools; no product source or test predicate changed.
The failed logs/receipts remain in the archive.

The first full run passed production startup and 14 snapshots, then Record
Desk typing produced keydowns without input events. The existing browser smoke
documents real fontconfig on minimal bundled hosts. Supplying that same repository
font configuration in the task wrapper makes the focused unchanged complete
workflow and the subsequent complete canonical command pass. The earlier failure,
input events and opened failure capture remain under `record-desk-font-failure/`.
The successful full run uses this recorded environment, with no host tool install
or default-toolchain change. Its warm elapsed time is not a performance comparison
with the initial new-compiler rebuild.

Linux access to the private preview returned its expected audience-policy 403.
The approved Mac route proves current assets, rendering, linked pan and Results.
Its optional Thumbnails check sampled a pane-body predicate after a fixed 300 ms
and failed; the observer did not retain sufficient dispatch/after-state evidence
to classify a product regression. Diagnostics/reload steps were not performed.
A focused fresh owned generation was prepared, but its observer failed before
creating a target or dispatching input: the owned SSH forward had timed out,
and one ordinary reconnection also timed out. These failures remain separate
from the passed maintained browser contracts. No uncertain input was replayed,
profile copied, audience policy changed or TLS exception added. Both owned preview
generations report `observed-inactive`; the original task page was absent, prior
page targets remained, and the forward PID/listener were absent. Raw private
bindings and observation files stay local; the bounded receipt retains their
SHA-256 identities. No complete Mac pilot is claimed.

## Compatibility and dependency disposition

No semantic compiler repair was needed. Both Rust 1.97.1 minimum declarations,
editions/resolver, member inheritance/omissions, warning/architecture policy and
all admission/cancellation/eviction limits remain. Both Cargo locks are unchanged:
root SHA-256 `8659266333ffacc69d40ab7f31336801acb800ff05cf1375c09a7bef6a3aeb75`,
Record Desk `894168d9eb788ac1fd7f7313001d0b4c8fe42c695dcb4935e70cb6c39569422a`.
The existing codec `6586e3d50f95429b242cb2e3535742b002784f2d` remains locked;
[E1 terminal evidence](https://github.com/emuella/emuella-workspace/pull/103#issuecomment-6052599947)
requires no consumer/API refresh. Old-compiler observations supply fixture and
compatibility knowledge, not Rust 1.99.0 build acceptance. No deployment,
release, package/data publication or protected-resource operation is included.
