# Navigation-item recipe

Status: active
Next action: Complete canonical verification, independent review and authorised landing.

## Outcome and ownership

Polyorama owns one reusable navigation row, scoped observations and deterministic
Gallery stories. Applications own destinations, counts, selected state, intents
and persistence. Bokkie integration and a broader chip/status system are excluded.

## Implementation and UI acceptance contract

The ordinary experience is Home, Tasks, Needs attention, Activity and Settings
beside a matching destination heading. The selected destination is the strongest
navigation cue; counts remain compact secondary information. Compare the shared
analytical action/icon vocabulary, retaining quiet planar chrome, aligned leading
artwork and label starts, restrained badge treatment and clear current state.

- One full-width token hit target; typed ActionTarget/IconId, complete label,
  description, caller selection and Availability. Stable PresentationScope and
  destination keys survive reordered rows, translations, counts and placement.
- Inert, start-aligned, single-line ButtonLabel ellipsis; reserve icon, gaps,
  padding and badge first. Parent provides the measured minimum width. No implicit
  collapsed mode. Caption badge elides once; counts above 99 show 99+ with full
  meaning/count in the row description. Omission and zero are explicit choices.
- Persistent current marker, separate inset pointer-press outline and outer
  keyboard-focus ring, even with equal hover/selection fills. Disabled reason,
  light/dark/high contrast, both densities and 150% text remain observable.
- Independent Tab stops, Enter/Space activation, no selection change on focus.
  Snapshot NavigationItem.selected maps to AccessKit Button aria_current; no
  dock-tab, toggle or second interactive badge. Current clipped geometry and
  omitted hidden nodes; measured label/badge children share one semantic owner.
- Ordinary and adversarial production stories plus a clipped scroll fixture.
  Tests cover pointer/keyboard/disabled, focus vs selection, identity changes,
  current parity, bounded measurements and min geometry, hidden/partial rows.

Calibration question: does one row preserve useful label space and unmistakable
current/focus/press cues across narrow/enlarged reference appearances? Smallest
probe: five production rows plus long-label/large-count and disabled siblings.
Evidence owner: docs/navigation-item-evidence.md and navigation-item-evidence/.
Exit: empty semantic/text audits and accepted ordinary/adversarial rendered
critique; revise once if hierarchy, state distinction or badge balance fails.
Frozen evaluation seeds stay unchanged. Baselines are reviewed separately.

## Delivery increments

| Increment | Status | Evidence |
| --- | --- | --- |
| Production recipe and scoped API | Complete | Navigation regressions and executable consumer example |
| Gallery, consumer documentation and physical evidence | Complete | [UI evidence](navigation-item-evidence.md), physical journeys and 3 inspected baselines |
| Candidate qualification and landing | Pending | cargo xtask verify, independent review and GitHub CI |
