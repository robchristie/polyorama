# Compact status chip

Status: active
Next action: Complete canonical verification, independent review and authorised landing of the selected candidate.

## Outcome and boundaries

Applications own workflow states, transitions, labels, localisation and explicit
semantic tones. Polyorama owns one compact informational recipe, its measured
text, geometry, theme treatment and observations. Preserve wrapping status_badge,
PresentationContext::badge and navigation badge contracts. Bokkie integration,
filter/selection/removal controls and live announcements are separate work.

## Implementation and UI acceptance contract

Representative task: scan task names with quiet adjacent statuses (Active,
Scheduled, Needs review), then read the selected task heading with the same
recipe. Names and heading lead; chips remain secondary information, clearly
smaller than action targets. Reference: analytical production status badge and
navigation badge geometry; keep planar surfaces and restrained boundaries.

- One scoped call and public low-level recipe. Borrowed full label, StatusTone,
  optional decorative IconId/explanation, typed bounded width and TextInteraction.
  No label inference or framework workflow enum; stable logical scope/key.
- Content-sized single-line Status text, start aligned and vertically centred;
  explicit measured ellipsis. Reserve token padding, icon and gap first. Minimum
  useful label slot retains measured ellipsis; smaller parents receive a minimum
  allocation and must wrap/scroll/relocate, never implicit icon-only paint.
- Default selectable via present_measured_text and supported egui drag/copy;
  explicit inert mode in interactive rows. Visual chip and selection bounds are
  distinct; no button, toggle, click action or live region.
- One accessible text owner with full label/explanation even when elided.
  Stable current identity, correct parent/domain, matching clipped snapshot and
  AccessKit geometry, hidden omission, current-pass observations and honest text
  audit coverage. Decorative icon/chrome have no accessible owner.
- Opaque quiet surface resolved from ApplicationTheme tokens so ordinary,
  hover and selected parents cannot lower text contrast. Standard >=4.5:1 and
  high contrast >=7:1. Existing tone vocabulary, complete label and optional
  meaningful artwork retain interpretation independent of colour.
- Ordinary task-list/detail, tone/icon/parent-surface and narrow/localised/scroll
  production Gallery stories. Light/dark/high contrast, both densities and 150%
  fonts; partial/full clips and status updates. Test allocation, overflow, icon
  reservation, colours, identity, parity, selection/copy and inert row input.

Calibration question: does the bounded chip remain secondary, legible and
balanced beside task names and headings, including narrow/enlarged text?
Smallest probe: task/detail story plus narrow localisation and parent surfaces.
Evidence owner: docs/status-chip-evidence.md and status-chip-evidence/.
Exit: focused semantic/text proof and accepted two-pass ordinary/adversarial
critique; at most one visual refinement and re-critique. Frozen evaluation
fixtures/assertions stay unchanged. Baseline acceptance is separate.

## Delivery increments

| Increment | Status | Evidence |
| --- | --- | --- |
| Production recipe and scoped API | Complete | Nine focused regressions and compiled consumer mapping |
| Gallery, documentation and physical evidence | Complete | [UI evidence](status-chip-evidence.md), physical journeys and three inspected baselines |
| Final qualification and landing | Pending | cargo xtask verify, independent exact-head review, required CI |
