# Application composition documentation

Status: active
Next action: Complete the independent public fresh-reader exercise, then reconcile the plan for final exact-head review and landing.

## Outcome and scope

A new consumer can choose the four framework crates, compose a small native
application, route a meaningful action through validated commands and extend a
pane using public documentation and source. Baseline:
`abbe2162ffbbbd2570f4534bb94e2ebf415b898d`.

The supporting groups below are frozen before authoring. Existing README,
working rules, pane/interaction guides and item contracts receive credit.
No API, runtime behaviour, dependency, compiler or browser framework changes
are included. The small example uses project-authored vector content and the
existing Lab package's dependencies; it is separate from Lab application code.
Its native runtime is exercised. WASM compilation does not claim a browser app.

## Frozen coverage map

| Consumer task | Public supporting group and important contracts | Documentation | Example/check | Initial gap |
|---|---|---|---|---|
| P1: choose state and apply an action | `Document`, `Session`, `Workspace`, `ImageIntent`, `validate_intent`, `CommandHistory`; durable/transient ownership, validation may reserve an annotation ID, history does not validate arbitrary commands | Core crate introduction and local item docs; composition guide | Core doctest; minimal consumer | One-line crate introduction; validation side effect and command trust boundary undiscoverable |
| P1: request and complete work | `TileDemand`, `Runtime`, `RuntimeConfig`, `RuntimeInitError`, `RequestToken`, `DecodeRequest`, `DecodeEvent`; complete desired sets, generation/token identity, native waker vs external browser transport, hand-off and residency | Runtime crate introduction and local methods; composition guide | Runtime doctest; existing token/queue regression tests | No consumer lifecycle or target distinction at crate entry |
| P1: present decoded images | `RenderBridge`, `UploadAdmission`, `ScalarRenderer`, `ImageRenderRequest`, `PhysicalViewport`, `DisplaySettings`, `RenderPlan`; bounded ownership transfer, device/queue owner, per-frame maintenance, ordered callback publication | Renderer crate introduction and request/bridge docs; UI callback docs; composition guide | Renderer doctest; existing plan/bridge tests | Request fields and caller ordering lack local explanation |
| P2: run a minimum app | Core state/command group plus `DockBehaviour`, `DockTextContext`, `dock_workspace`, `PanePresenter`; one canonical dock tree, narrow view and output sink, no pane GPU resources | README entrance; composition guide; maintained Cargo example | Native physical interaction smoke with snapshots/capture; native and WASM example compilation | Large applications only |
| P3: vary a pane/presentation | `PanePresenter`, `PresentationContext`, `PresentationScope`, `PresentationObservations`, `ActionKey`, `ActionSpec`, `ActionTarget`, `ActionButtonSpec`; stable identities, pass-local publication, layout/command ownership, repaint after applied output | UI crate introduction; seam/item docs; guide variation | UI doctest; minimal example; independent fresh-reader variation | Pane seam undocumented; pass-local presentation has useful existing contracts to retain |

Transitive exports, analytical feature panes, regional-source implementations,
arbitrary texture import and new docking/scheduling/rendering capabilities are
outside this bounded envelope. Existing regional contracts remain authoritative.

## Delivery checkpoints

| Increment | State | Evidence/next proof |
|---|---|---|
| Source discovery and scope freeze | Complete | Frozen map above; source baseline |
| Public entrances, item contracts and example | Complete | Four crate entries, local supporting contracts, composition guide and maintained minimal consumer |
| Documentation check integration | Complete | Four executed doctests, selected native/WASM compilation and local links; intended broken probes rejected; complete canonical route passed |
| Consumer qualification and delivery | Active | Native baseline and presenter variation, WASM compile and nine rendered pages pass; independent fresh-reader exercise then final review/CI/landing remain |

Detailed qualification belongs in `docs/api-documentation-evidence.md`; generated
HTML stays in ignored output directories; bounded actual logs and observations are retained in the product evidence archive.
