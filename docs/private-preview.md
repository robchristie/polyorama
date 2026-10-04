# Analytical Workspace Lab private development preview

The root `.dev-preview.toml` identifies this application as `polyorama-lab` and
launches it through an installed gateway's version 1 inherited-socket contract.
The identifier follows that contract's lower-case, maximum-31-character grammar.
Python 3.11 or later is sufficient for the
adapter and its verification tests. The gateway owns the listening allocation,
HTTPS origin and preview lifecycle; `tools/dev-preview.py` adopts the supplied
`DEV_PREVIEW_LISTEN_FD` directly, without closing it and binding another server.
Host configuration and gateway installation are separate operator work.

From the canonical worktree root, build the development browser assets first:

```sh
cargo xtask build-web
dev-preview up --json
dev-preview status --json
dev-preview logs
```

Open the HTTPS `url` returned by `up` in a WebGPU-capable browser. Keep its exact
worktree and run identity with the pilot evidence. `/_dev_preview/ready` reports
the supplied project, canonical worktree, run ID and origin only while all nine
required Lab assets are readable, non-empty regular files; WASM assets must have
the expected WASM header. Startup refuses missing assets and identity mismatch.
Readiness proves the adapter and assets are available; browser rendering and
interaction still need direct qualification.

The adapter serves only the fixed Lab HTML, CSS, bootstrap, startup helper,
worker and generated JS/WASM files from `apps/analytical-workspace-lab/web`.
It rejects symlinks in asset paths, traversal, directory listings and other
files. It publishes no repository root, evidence directory, Gallery, viewer or
data service. Every response uses `Cache-Control: no-store`; WASM responses use
`application/wasm`. Existing relative imports keep the worker and its package
on the same preview origin.

Development uses browser reloads. After a Rust edit, run `cargo xtask build-web`
again and reload. After an HTML, CSS or JS edit, reload directly. The adapter
reads files on each request; launch revision and dirty state reported by the
gateway remain launch-time snapshots. Restart the preview after changing the
manifest or adapter:

```sh
dev-preview down --json
dev-preview up --json
```

The Lab has no application API, WebSocket or hot-module-replacement service.
There is no extra server or HMR port. Normal loopback development and the
[production packaging and cache policies](browser-startup.md) keep their
existing behaviour.

## Bounded browser pilot

Use a freshly built, identified worktree and retain the `up`/`status` JSON,
source commit and dirty observation, build command/tool versions and hashes of
the served assets. Browser qualification should record the browser version,
WebGPU backend and tested environment alongside screenshots and observations:

1. Open the exact returned HTTPS origin. Confirm that the workspace renders,
   required JS/WASM responses succeed with the documented MIME/cache headers,
   and the relative tile worker completes work.
2. Pan Primary View and observe Linked Detail; open Results, Thumbnails and
   Diagnostics. Record visible image content and successful worker activity,
   rather than treating a generic HTTP 200 as application readiness.
3. Make a small reversible HTML/CSS change in the Lab web root, reload and
   observe it. Restore the file and reload again. Record both observations and
   preserve unrelated edits.
4. Stop this worktree's preview with `dev-preview down --json`. Confirm the
   recorded run is inactive and its URL no longer serves the Lab. Retain the
   cleanup result; stopping this preview must leave other runs alone.

The focused adapter regression command is:

```sh
python3 -m unittest discover -s tools/tests -p test_dev_preview.py
```

It launches the real adapter with an inherited descriptor while the simulated
publisher retains its original socket, checks that a competing bind fails
before/during/after the child run, validates readiness identity and assets, and
checks missing assets, invalid descriptors and path/symlink rejection. The
canonical `cargo xtask verify` includes these tests and the existing full
native/browser verification surface. A focused pass does not substitute for
that command or the browser pilot.
