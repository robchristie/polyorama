# End-user accessibility integration plan

Status: verified draft candidate held; actual-AT qualification blocked

Baseline revision: `0e725f5a97a6d99a6bc4c961dfc05b4e9252ba1d`

Reorientation budget: 200 lines. Detailed adapter observations, platform
qualification and retained artefacts belong in the accessibility evidence
report or with their executable owner.

## Objective and boundaries

Connect the existing egui/AccessKit semantics to supported platform adapters,
make a representative analytical workflow understandable and operable by
keyboard and assistive technology, enrich bounded GPU-viewport semantics, and
report only the environments exercised with actual assistive technology.

The increment may change application presentation, `polyorama-ui-egui`, test
and qualification tooling, and accessibility documentation. It does not replace
egui, wgpu or AccessKit; create a DOM or retained widget tree; move UI concerns
into core/runtime; materialise complete collections; redesign unrelated UI; or
perform release, deployment or publication work. `Workspace`, document,
session, selection and application `ActionKey` paths remain authoritative.

Ordinary code, tests, documentation and CI changes may land after canonical
verification and exact-head independent review. A claim not exercised with
actual assistive technology, materially unproved acceptance, and any release,
deployment, breaking-compatibility, rights, credential or security-policy
change remains a human-review boundary.

## Exploration and calibration gate

Question: can egui/eframe 0.36.1 expose Polyorama's existing AccessKit tree to
native and browser assistive technologies without a second authoritative UI or
application-state model?

Smallest representative probe:

- one enabled application action and one disabled action with its reason;
- one dock tab and one adjustable splitter;
- one selectable bounded result; and
- one analytical viewport named with active tool, current selection and
  currently available actions.

Evidence owner: `docs/accessibility-integration-report.md` plus focused tests
and retained artefacts that it identifies. Record the exact source/dependency
revision, adapter/platform configuration, accessibility-tree or assistive-
technology observations, route result, and retain/adapt/reject decision.

Exit when one coherent route is selected for each in-scope platform or a
minimal reproduction proves the current upstream browser route cannot provide
the integration. Do not broaden into custom DOM or accessibility frameworks.

## Delivery graph

| Increment | Outcome | Depends on | Status | Evidence |
| --- | --- | --- | --- | --- |
| 0 | Native/browser adapter exploration and representative probe | — | Complete | Accessibility integration report |
| 1 | Selected adapter route, honest availability and update propagation | 0 | Complete | Feature graph, architecture gate, native/Wasm builds and retained browser reproduction |
| 2 | Bounded analytical viewport context and non-pointer workflow actions | 1 | Complete | Semantic parity, dynamic-state and custom-action tests |
| 3 | Focus, dynamic state, virtualisation, dock restoration and physical checks | 1–2 | Complete | Kittest, native/browser harness and deterministic UI evidence |
| 4 | Actual assistive-technology qualification and exact platform matrix | 1–3 | Blocked externally | Current environment has no desktop AT; browser adapter is unavailable in stock eframe 0.36.1 |
| 5 | Canonical verification, report reconciliation, exact-head review and landing | 0–4 | Complete to human boundary | Local and CI `cargo xtask verify`, independent review and draft PR [#28](https://github.com/robchristie/polyorama/pull/28) pass; merge remains held by increment 4 |

## Acceptance proof

- Native adapter enablement and current semantic-update propagation are
  directly tested without unconditional repaint.
- Browser updates are handled by the strongest route supported by this exact
  stack, or a retained minimal upstream blocker identifies where they stop.
- The representative frame exposes one coherent application/dock/pane hierarchy
  with stable identity, state, bounds, actions, focus and disabled explanation.
- The viewport communicates pane/activity, tool, camera link, selection,
  loading/stale/availability and useful coordinate/scale context, with bounded
  keyboard/list/inspector alternatives for spatial operations.
- Behavioural, semantic/AccessKit, physical input, and deterministic visual/text
  axes pass, including dynamic state, virtualisation, duplicate ownership,
  rearrangement and persistence restoration.
- Every platform claim is backed by the specified actual assistive-technology
  workflow; every non-qualified route records the blocker and smallest next
  session required.
- The final exact candidate passes `cargo xtask verify`, receives independent
  exact-head review, and completes the repository's normal pull-request landing
  loop unless a documented human-review boundary remains.

## Current phase

The exploration gate selected eframe's native AccessKit adapter and retained
the stock WebRunner's discarded AccessKit update as the minimal browser
blocker. The platform-independent implementation and canonical verification
are complete, independently reviewed and retained in verified draft PR
[#28](https://github.com/robchristie/polyorama/pull/28). No actual
assistive-technology platform is qualified, so the change is not merged.

## Next action

Run the Linux Orca/AT-SPI2 workflow in the accessibility integration report on
the exact candidate and retain its observations. A human can then decide
whether that evidence is sufficient to remove the hold for the claimed Linux
combination; every other platform row remains independently unqualified.
