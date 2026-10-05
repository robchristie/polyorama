#![cfg(not(target_arch = "wasm32"))]

use egui::{Key, Modifiers, Vec2};
use egui_kittest::{Harness, kittest::Queryable};
use record_desk::{app::RecordDeskApp, model::*, store::Store};

fn app_harness(
    width: f32,
    records: Vec<Record>,
) -> (Harness<'static, Option<RecordDeskApp>>, tempfile::TempDir) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("state.json");
    Store::at_path(&path)
        .save(&SavedState::from_desk(
            &Desk::new(records).unwrap(),
            &default_workspace(),
        ))
        .unwrap();
    let mut harness = Harness::builder()
        .with_size(Vec2::new(width, 1000.0))
        .build_ui_state(
            move |ui, state: &mut Option<RecordDeskApp>| {
                if state.is_none() {
                    *state = Some(RecordDeskApp::with_store(ui.ctx(), Store::at_path(&path)));
                    ui.ctx().request_repaint();
                    return;
                }
                state.as_mut().unwrap().present(ui);
            },
            None,
        );
    harness.run();
    (harness, directory)
}

fn records() -> Vec<Record> {
    Desk::default().records()[..2].to_vec()
}

fn replace(h: &mut Harness<'_, Option<RecordDeskApp>>, text: &str) {
    h.get_by_label("Record title").click();
    h.run();
    h.key_press_modifiers(Modifiers::COMMAND, Key::A);
    h.run();
    h.key_press(Key::Backspace);
    h.run();
    h.get_by_label("Record title").type_text(text);
    h.run();
}

#[test]
fn editor_routes_invalid_cancel_apply_and_history_through_the_ui() {
    let (mut h, _dir) = app_harness(1080.0, records());
    let original = h.state().as_ref().unwrap().snapshot().records;
    replace(&mut h, "");
    h.key_press_modifiers(Modifiers::COMMAND, Key::Enter);
    h.run();
    let invalid = h.state().as_ref().unwrap().snapshot();
    assert!(invalid.error);
    assert_eq!(invalid.records, original);
    assert_eq!(invalid.undo_entries, 0);
    h.key_press_modifiers(Modifiers::COMMAND | Modifiers::SHIFT, Key::Enter);
    h.run();
    assert!(!h.state().as_ref().unwrap().snapshot().draft_dirty);
    replace(&mut h, &format!("  {}  ", original[0].title));
    h.key_press_modifiers(Modifiers::COMMAND, Key::Enter);
    h.run();
    let normalised = h.state().as_ref().unwrap().snapshot();
    assert_eq!(normalised.records, original);
    assert_eq!(normalised.undo_entries, 0);
    assert!(!normalised.draft_dirty);
    assert!(normalised.message.contains("No history entry"));
    replace(&mut h, "A committed edit");
    h.get_by_label("Reviewed").click();
    h.run();
    h.key_press_modifiers(Modifiers::COMMAND, Key::Enter);
    h.run();
    let applied = h.state().as_ref().unwrap().snapshot();
    assert_eq!(applied.undo_entries, 1);
    assert_eq!(applied.records[0].title, "A committed edit");
    assert_ne!(applied.records[0].reviewed, original[0].reviewed);
    h.key_press_modifiers(Modifiers::COMMAND, Key::Z);
    h.run();
    assert_eq!(h.state().as_ref().unwrap().snapshot().records, original);
    h.key_press_modifiers(Modifiers::COMMAND | Modifiers::SHIFT, Key::Z);
    h.run();
    assert_eq!(
        h.state().as_ref().unwrap().snapshot().records,
        applied.records
    );
}

#[test]
fn ordinary_narrow_and_empty_observations_have_stable_identity_and_honest_text_coverage() {
    let mut identities = Vec::new();
    for width in [1080.0, 390.0] {
        let (h, _dir) = app_harness(width, records());
        let s = h.state().as_ref().unwrap().snapshot();
        assert!(s.ui.semantic_audit.is_empty(), "{:?}", s.ui.semantic_audit);
        assert!(s.ui.text_audit.is_empty(), "{:?}", s.ui.text_audit);
        let arrange =
            s.ui.nodes
                .iter()
                .find(|n| n.actions.iter().any(|a| a.0 == "record-desk.arrange"))
                .unwrap();
        assert_eq!(arrange.enabled, width >= 640.0);
        assert!(
            s.ui.text_audit_coverage
                .as_ref()
                .unwrap()
                .native_text_controls
                >= 6
        );
        assert!(
            s.ui.text_audit_coverage
                .as_ref()
                .unwrap()
                .measured_components
                > 0
        );
        let row =
            s.ui.nodes
                .iter()
                .find(|n| n.id.0 == "record-desk.record.1001")
                .unwrap();
        identities.push(row.id.clone());
        assert!(matches!(
            row.domain_reference,
            Some(polyorama_ui_egui::DomainReference::External { .. })
        ));
    }
    assert_eq!(identities[0], identities[1]);
    let (h, _dir) = app_harness(390.0, vec![]);
    let s = h.state().as_ref().unwrap().snapshot();
    assert!(s.visible_ids.is_empty());
    assert!(s.selected.is_none());
    assert!(h.query_by_label("Choose a record").is_some());
    assert!(
        h.query_by_label("The saved collection is empty. Move the saved state outside the app, then use Restore to start the synthetic collection.")
            .is_some()
    );
}
