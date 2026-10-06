//! Application-owned bindings; the public framework owns the transport contract.
use super::*;
use std::collections::BTreeMap;

impl RecordDeskApp {
    #[cfg(not(target_arch = "wasm32"))]
    fn new_inspection() -> Inspection {
        Inspection::with_build_identity(
            "Record Desk",
            BuildIdentity {
                version: env!("CARGO_PKG_VERSION").into(),
                source_revision: option_env!("POLYORAMA_BUILD_REVISION").map(str::to_owned),
            },
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn start_inspection(&mut self, context: &egui::Context) {
        let inspection = Self::new_inspection();
        let wake_context = context.clone();
        match inspection.start_native(move || wake_context.request_repaint()) {
            Ok(Some(host)) => {
                inspection.install_completion_hook(context);
                self.inspection = Some(inspection);
                self._inspection_host = Some(host);
            }
            Ok(None) => {}
            Err(error) => self.failure(format!("Automation could not start: {error}")),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn enable_inspection(&mut self, instance: &str) -> Result<(), String> {
        if self.inspection.is_none() {
            let service = Inspection::with_instance(
                "Record Desk",
                BuildIdentity {
                    version: env!("CARGO_PKG_VERSION").into(),
                    source_revision: option_env!("POLYORAMA_BUILD_REVISION").map(str::to_owned),
                },
                instance,
            )
            .map_err(|error| error.to_string())?;
            service.install_completion_hook(&self.inspection_context);
            self.inspection = Some(service);
            self.inspection_context.request_repaint();
        }
        Ok(())
    }

    fn inspection_bindings(&self, snapshot: &UiSnapshot) -> Vec<InspectionBinding<Action>> {
        Action::ALL
            .into_iter()
            .flat_map(|action| {
                let invocable = matches!(
                    action,
                    Action::Apply
                        | Action::Cancel
                        | Action::Undo
                        | Action::Redo
                        | Action::Search
                        | Action::ResetFilters
                        | Action::Arrange
                );
                // Guard assumptions that affect meaning, not focus or repaint counters.
                let meaning = match action {
                    Action::Apply
                    | Action::Cancel
                    | Action::Undo
                    | Action::Redo
                    | Action::ResetFilters => self.inspection_revision.to_string(),
                    Action::Arrange => match &self.workspace.root {
                        polyorama_core::DockNode::Split { axis, .. } => format!("{axis:?}"),
                        _ => "unsplit".into(),
                    },
                    _ => "stable".into(),
                };
                let binding = InspectionBinding::new(
                    ActionTarget::application(action),
                    self.availability(action),
                    invocable,
                    meaning,
                );
                let nodes: Vec<_> = snapshot.by_action(action).collect();
                if nodes.is_empty() {
                    vec![binding]
                } else {
                    nodes
                        .into_iter()
                        .map(|node| {
                            let mut bound = binding.clone().with_id(node.id.clone());
                            if let Some(domain) = &node.domain_reference {
                                bound = bound.with_domain(domain.clone());
                            }
                            bound
                        })
                        .collect()
                }
            })
            .collect()
    }

    pub(super) fn drain_inspection(&mut self, context: &egui::Context) {
        let Some(service) = self.inspection.clone() else {
            return;
        };
        while service.drain(&self.inspection_bindings(&self.snapshot.ui), |target| {
            self.action(target.action)
                .map_err(InspectionError::validation)?;
            context.request_repaint();
            Ok(())
        }) {}
    }

    #[cfg(target_arch = "wasm32")]
    pub fn inspection_request(&mut self, json: &str) -> Result<String, String> {
        let service = self.inspection.clone().ok_or("Automation is disabled")?;
        if json.len() > INSPECTION_REQUEST_BYTES {
            return Ok(service.handle_json(json));
        }
        let request: InspectionRequest = match serde_json::from_str(json) {
            Ok(request) => request,
            Err(_) => return Ok(service.handle_json(json)),
        };
        let context = self.inspection_context.clone();
        let response = service.dispatch(
            request,
            &self.inspection_bindings(&self.snapshot.ui),
            |target| {
                self.action(target.action)
                    .map_err(InspectionError::validation)?;
                context.request_repaint();
                Ok(())
            },
        );
        serde_json::to_string(&response).map_err(|e| e.to_string())
    }

    pub(super) fn publish_inspection(&self, context: &egui::Context, snapshot: &UiSnapshot) {
        let Some(service) = &self.inspection else {
            return;
        };
        if context.will_discard() {
            return;
        }
        let facts = BTreeMap::from([
            ("undo_entries".into(), (self.desk.undo_len() as u64).into()),
            ("redo_entries".into(), (self.desk.redo_len() as u64).into()),
            ("draft_dirty".into(), self.desk.is_dirty().into()),
            ("status".into(), self.message.clone().into()),
            ("error".into(), self.error.into()),
            (
                "selected_record".into(),
                self.desk
                    .selected()
                    .map_or(String::new(), |id| id.0.to_string())
                    .into(),
            ),
            (
                "draft_title".into(),
                self.desk
                    .draft()
                    .map_or(String::new(), |r| r.title.clone())
                    .into(),
            ),
            (
                "layout_axis".into(),
                match &self.workspace.root {
                    DockNode::Split { axis, .. } => format!("{axis:?}"),
                    _ => "unsplit".into(),
                }
                .into(),
            ),
        ]);
        let metadata = CollectionMetadata {
            collections: vec![CollectionObservation {
                id: "records".into(),
                total: Some(self.desk.records().len() as u64),
                observed: snapshot.by_role(UiRole::ResultRow).count(),
                offset: None,
                virtualised: false,
                complete: snapshot.by_role(UiRole::ResultRow).count() == self.desk.records().len(),
            }],
            ..Default::default()
        };
        if let Err(error) = service.stage_completed(
            context,
            snapshot.clone(),
            &self.inspection_bindings(snapshot),
            metadata,
            facts,
        ) {
            eprintln!("Automation publication failed: {error:?}");
        }
    }
}
