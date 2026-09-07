//! Thin, pass-local access to the existing presentation recipes.

use std::hash::Hash;

use egui::{Response, Ui};

use crate::{
    ActionButtonIdentity, ActionButtonSpec, ActionKey, DesignTokens, DomainReference,
    NativeTextControlKind, SemanticUiId, TextAuditCoverage, TextInteraction, TextLayoutObservation,
    TextOverflow, TextRole, UiNode, action_button_with_identity,
    action_semantic_node_with_identity, record_native_text_control, text_audit_coverage,
};

/// Logical hierarchy, independent of egui's layout hierarchy. Keys must describe
/// stable controls or domain objects, never row positions, labels or counters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresentationScope(egui::Id);

impl PresentationScope {
    pub fn new(key: impl Hash + std::fmt::Debug) -> Self {
        Self(egui::Id::new(("polyorama.presentation", key)))
    }

    pub fn child(self, key: impl Hash + std::fmt::Debug) -> Self {
        Self(self.0.with(key))
    }

    pub fn instance(self, key: impl Hash + std::fmt::Debug) -> PresentationId {
        PresentationId(self.0.with(key))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresentationId(egui::Id);

impl PresentationId {
    pub fn egui_id(self) -> egui::Id {
        self.0
    }

    pub fn semantic_id(self) -> SemanticUiId {
        SemanticUiId::new(format!("presentation.{:016x}", self.0.value()))
    }

    pub fn text_instance(self) -> u64 {
        // Snapshot IDs must round-trip through JavaScript without rounding.
        self.0.value() & ((1_u64 << 53) - 1)
    }

    pub fn action_identity(self) -> ActionButtonIdentity {
        ActionButtonIdentity {
            widget_id: self.egui_id(),
            semantic_id: self.semantic_id(),
            text_instance: self.text_instance(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ContentTextSpec {
    pub role: TextRole,
    pub overflow: TextOverflow,
    pub max_lines: u8,
    pub interaction: TextInteraction,
}

/// The source and justification for presentation outside the adapter's recipes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawPresentation {
    pub id: SemanticUiId,
    pub reason: &'static str,
}

/// Published inside the same pass. Keep the latest publication per viewport;
/// replace it on a repeated layout pass, rather than extending prior results.
/// Coverage's measured count covers this context's retained layouts; attempted,
/// failed and native counts use the shared viewport-pass inventory. Recompute
/// once over merged layouts at the viewport boundary; never sum partial coverage.
pub struct PresentationObservations {
    pub viewport: egui::ViewportId,
    pub pass: u64,
    pub text_layouts: Vec<TextLayoutObservation>,
    pub semantic_nodes: Vec<UiNode>,
    pub coverage: TextAuditCoverage,
    pub raw_presentations: Vec<RawPresentation>,
}

/// Owns only resolved appearance and current observations. The caller retains
/// explicit egui layout, state, command delivery and collection virtualisation.
/// Construct inside each pass and finish before leaving it. Reuse in another
/// context, viewport or pass is rejected, including publication after a retry.
pub struct PresentationContext {
    context: egui::Context,
    viewport: egui::ViewportId,
    pass: u64,
    tokens: DesignTokens,
    font_scale: f32,
    scope: PresentationScope,
    parent: SemanticUiId,
    domain_reference: Option<DomainReference>,
    text_layouts: Vec<TextLayoutObservation>,
    semantic_nodes: Vec<UiNode>,
    raw_presentations: Vec<RawPresentation>,
}

impl PresentationContext {
    pub fn new(
        ui: &mut Ui,
        tokens: DesignTokens,
        font_scale: f32,
        scope: PresentationScope,
        parent: SemanticUiId,
    ) -> Self {
        Self {
            context: ui.ctx().clone(),
            viewport: ui.ctx().viewport_id(),
            pass: ui.ctx().cumulative_pass_nr(),
            tokens,
            font_scale,
            scope,
            parent,
            domain_reference: None,
            text_layouts: Vec::new(),
            semantic_nodes: Vec::new(),
            raw_presentations: Vec::new(),
        }
    }

    /// Domain metadata is separate from capability routing. Include the stable
    /// domain ID in the logical scope when controls repeat for several objects.
    pub fn with_domain_reference(mut self, reference: DomainReference) -> Self {
        self.domain_reference = Some(reference);
        self
    }

    fn check_pass(&self, ui: &Ui) {
        assert!(
            self.context == *ui.ctx(),
            "presentation context changed egui context"
        );
        assert_eq!(
            self.viewport,
            ui.ctx().viewport_id(),
            "presentation context changed viewport"
        );
        assert_eq!(
            self.pass,
            ui.ctx().cumulative_pass_nr(),
            "presentation context outlived its pass"
        );
    }

    pub fn action<A: ActionKey>(
        &mut self,
        ui: &mut Ui,
        key: impl Hash + std::fmt::Debug,
        spec: ActionButtonSpec<A>,
    ) -> Response {
        self.check_pass(ui);
        let identity = self.scope.instance(key).action_identity();
        let target = spec.target;
        let state = spec.state;
        let availability = spec.availability.clone();
        let response = action_button_with_identity(
            ui,
            spec,
            &identity,
            &self.tokens,
            self.font_scale,
            &mut self.text_layouts,
        );
        let mut node = action_semantic_node_with_identity(
            &response,
            target,
            &availability,
            state,
            self.parent.clone(),
            &identity,
        );
        if self.domain_reference.is_some() {
            node.domain_reference = self.domain_reference.clone();
        }
        self.semantic_nodes.push(node);
        response
    }

    pub fn heading(
        &mut self,
        ui: &mut Ui,
        key: impl Hash + std::fmt::Debug,
        text: &str,
    ) -> Response {
        self.check_pass(ui);
        let identity = self.scope.instance(key);
        let response = crate::pane_content::scoped_section_heading(
            ui,
            identity.text_instance(),
            text,
            &self.tokens,
            self.font_scale,
            &mut self.text_layouts,
            identity.egui_id(),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_author_id(identity.semantic_id().0);
        });
        response
    }

    pub fn content(
        &mut self,
        ui: &mut Ui,
        key: impl Hash + std::fmt::Debug,
        text: &str,
        spec: ContentTextSpec,
    ) -> Response {
        self.check_pass(ui);
        let identity = self.scope.instance(key);
        let response = crate::pane_content::scoped_content_label(
            ui,
            identity.text_instance(),
            text,
            spec,
            &self.tokens,
            self.font_scale,
            &mut self.text_layouts,
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_author_id(identity.semantic_id().0);
        });
        response
    }

    /// A native control remains explicitly unmeasured in the text denominator.
    pub fn native<R>(
        &mut self,
        ui: &mut Ui,
        kind: NativeTextControlKind,
        render: impl FnOnce(&mut Ui) -> (Response, R),
    ) -> (Response, R) {
        self.check_pass(ui);
        let result = render(ui);
        record_native_text_control(&result.0, kind);
        result
    }

    /// Annotate raw presentation, not ordinary layout such as rows or scrolling.
    /// The closure still owns any native-control recording or measured evidence.
    pub fn raw<R>(
        &mut self,
        ui: &mut Ui,
        key: impl Hash + std::fmt::Debug,
        reason: &'static str,
        render: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        self.check_pass(ui);
        assert!(
            !reason.trim().is_empty(),
            "raw presentation requires a reason"
        );
        self.raw_presentations.push(RawPresentation {
            id: self.scope.instance(key).semantic_id(),
            reason,
        });
        render(ui)
    }

    /// Geometry filters may remove clipped successes, but never failed attempts.
    pub fn retain_visible_text(
        &mut self,
        ui: &mut Ui,
        mut visible: impl FnMut(&TextLayoutObservation) -> bool,
    ) {
        self.check_pass(ui);
        self.text_layouts
            .retain(|item| item.layout_error.is_some() || visible(item));
    }

    pub fn finish(self, ui: &mut Ui) -> PresentationObservations {
        self.check_pass(ui);
        let coverage = text_audit_coverage(ui.ctx(), &self.text_layouts);
        PresentationObservations {
            viewport: self.viewport,
            pass: self.pass,
            text_layouts: self.text_layouts,
            semantic_nodes: self.semantic_nodes,
            coverage,
            raw_presentations: self.raw_presentations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ActionButtonState, ActionEmphasis, ActionTarget, Availability, DensityVariant,
        ThemeVariant, UiRole, UiSnapshot, audit_accesskit, audit_text_layouts,
        test_actions::TestAction,
    };

    fn tokens() -> DesignTokens {
        DesignTokens::resolve(ThemeVariant::Dark, DensityVariant::Comfortable)
    }

    fn action() -> ActionButtonSpec<TestAction> {
        ActionButtonSpec {
            target: ActionTarget::application(TestAction::Undo),
            availability: Availability::Enabled,
            state: ActionButtonState::Momentary,
            emphasis: ActionEmphasis::Normal,
            compact: false,
        }
    }

    fn content(max_lines: u8) -> ContentTextSpec {
        ContentTextSpec {
            role: TextRole::Body,
            overflow: TextOverflow::Wrap,
            max_lines,
            interaction: TextInteraction::Selectable,
        }
    }

    #[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
    enum Control {
        HeaderUndo,
        FooterUndo,
    }

    #[test]
    fn repeated_capabilities_keep_identity_across_reordering_and_incidental_layout() {
        let context = egui::Context::default();
        crate::install_typography_fonts(&context);
        context.enable_accesskit();
        let root = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(600.0, 300.0));
        let scope = PresentationScope::new("detail").child("obligation-17");
        let domain = DomainReference::External {
            namespace: "test.obligation".into(),
            id: "obligation-17".into(),
        };
        let mut previous = Vec::new();
        for (layout, controls) in [
            ("desktop", [Control::HeaderUndo, Control::FooterUndo]),
            ("narrow", [Control::FooterUndo, Control::HeaderUndo]),
        ] {
            let mut published = None;
            let mut output = context.run_ui(
                egui::RawInput {
                    screen_rect: Some(root),
                    ..Default::default()
                },
                |ui| {
                    let mut presentation =
                        PresentationContext::new(ui, tokens(), 1.0, scope, SemanticUiId::root())
                            .with_domain_reference(domain.clone());
                    ui.push_id(layout, |ui| {
                        ui.horizontal(|ui| {
                            for control in controls {
                                let response = presentation.action(ui, control, action());
                                assert_eq!(response.id, scope.instance(control).egui_id());
                            }
                        });
                    });
                    published = Some(presentation.finish(ui));
                },
            );
            let published = published.unwrap();
            assert_eq!(published.semantic_nodes.len(), 2);
            assert_ne!(
                published.semantic_nodes[0].id,
                published.semantic_nodes[1].id
            );
            assert_ne!(
                published.text_layouts[0].component_id,
                published.text_layouts[1].component_id
            );
            assert!(
                published
                    .semantic_nodes
                    .iter()
                    .all(|node| node.domain_reference == Some(domain.clone()))
            );
            let ids: Vec<_> = published
                .semantic_nodes
                .iter()
                .map(|node| node.id.clone())
                .collect();
            if !previous.is_empty() {
                assert_eq!(ids, previous.into_iter().rev().collect::<Vec<_>>());
            }
            previous = ids;
            let mut snapshot = UiSnapshot::default();
            snapshot.nodes.push(UiNode::container(
                SemanticUiId::root(),
                None,
                UiRole::Application,
                root.into(),
            ));
            snapshot.nodes.extend(published.semantic_nodes);
            let update = output.platform_output.accesskit_update.take().unwrap();
            assert!(audit_accesskit(&snapshot, &update).is_empty());
            output.textures_delta.clear();
        }
        assert_ne!(
            scope.instance(Control::HeaderUndo),
            PresentationScope::new("detail")
                .child("obligation-18")
                .instance(Control::HeaderUndo),
        );
    }

    #[test]
    fn text_responses_and_author_ids_survive_layout_changes_without_changing_paint() {
        let mut paints = Vec::new();
        let scope = PresentationScope::new("text-identity");
        for scoped in [false, true] {
            let context = egui::Context::default();
            crate::install_typography_fonts(&context);
            context.enable_accesskit();
            let mut previous_ids = None;
            for layout in ["wide", "narrow"] {
                let mut output = context.run_ui(Default::default(), |ui| {
                    ui.push_id(layout, |ui| {
                        if scoped {
                            let mut presentation = PresentationContext::new(
                                ui,
                                tokens(),
                                1.0,
                                scope,
                                SemanticUiId::root(),
                            );
                            let heading = presentation.heading(ui, "heading", "Current obligation");
                            let content = presentation.content(
                                ui,
                                "content",
                                "Supporting evidence",
                                content(2),
                            );
                            let ids = (heading.id, content.id);
                            if let Some(previous) = previous_ids {
                                assert_eq!(ids, previous);
                            }
                            previous_ids = Some(ids);
                            let _ = presentation.finish(ui);
                        } else {
                            crate::section_heading(
                                ui,
                                scope.instance("heading").text_instance(),
                                "Current obligation",
                                &tokens(),
                                1.0,
                                &mut Vec::new(),
                            );
                            crate::measured_content_label(
                                ui,
                                scope.instance("content").text_instance(),
                                "Supporting evidence",
                                TextRole::Body,
                                TextOverflow::Wrap,
                                2,
                                TextInteraction::Selectable,
                                &tokens(),
                                1.0,
                                &mut Vec::new(),
                            );
                        }
                    });
                });
                if scoped {
                    let update = output.platform_output.accesskit_update.take().unwrap();
                    for key in ["heading", "content"] {
                        let author_id = scope.instance(key).semantic_id().0;
                        assert_eq!(
                            update
                                .nodes
                                .iter()
                                .filter(|(_, node)| node.author_id() == Some(author_id.as_str()))
                                .count(),
                            1
                        );
                    }
                }
                output.textures_delta.clear();
                paints.push(output.shapes);
            }
        }
        assert_eq!(paints[0], paints[2]);
        assert_eq!(paints[1], paints[3]);
    }

    #[test]
    fn geometry_filter_preserves_failed_attempts_and_native_controls_stay_unmeasured() {
        let context = egui::Context::default();
        crate::install_typography_fonts(&context);
        let mut output = context.run_ui(Default::default(), |ui| {
            let mut presentation = PresentationContext::new(
                ui,
                tokens(),
                1.0,
                PresentationScope::new("coverage"),
                SemanticUiId::root(),
            );
            presentation.heading(ui, "title", "Current obligation");
            presentation.content(ui, "valid", "Available evidence", content(2));
            presentation.content(ui, "invalid", "Bad request", content(24));
            presentation.native(ui, NativeTextControlKind::Button, |ui| {
                (ui.button("Native"), ())
            });
            presentation.raw(ui, "reader", "Native selectable evidence reader", |ui| {
                ui.label("Unmeasured evidence");
            });
            presentation.retain_visible_text(ui, |_| false);
            let publication = presentation.finish(ui);
            assert_eq!(publication.text_layouts.len(), 1);
            assert_eq!(publication.coverage.attempted_components, 3);
            assert_eq!(publication.coverage.successful_components, 2);
            assert_eq!(publication.coverage.failed_components, 1);
            assert_eq!(publication.coverage.native_text_controls, 1);
            assert_eq!(publication.coverage.observed_native_controls, 0);
            assert_eq!(publication.raw_presentations.len(), 1);
            assert!(!audit_text_layouts(&publication.text_layouts).is_empty());
        });
        output.textures_delta.clear();
    }

    #[test]
    fn multiple_contexts_merge_layouts_and_recompute_coverage_without_summing_inventories() {
        let context = egui::Context::default();
        crate::install_typography_fonts(&context);
        context
            .run_ui(Default::default(), |ui| {
                let mut first = PresentationContext::new(
                    ui,
                    tokens(),
                    1.0,
                    PresentationScope::new("first"),
                    SemanticUiId::root(),
                );
                first.content(ui, "text", "First region", content(1));
                first.native(ui, NativeTextControlKind::Button, |ui| {
                    (ui.button("Native"), ())
                });
                let mut first = first.finish(ui);
                let mut second = PresentationContext::new(
                    ui,
                    tokens(),
                    1.0,
                    PresentationScope::new("second"),
                    SemanticUiId::root(),
                );
                second.content(ui, "text", "Second region", content(1));
                let second = second.finish(ui);
                assert_eq!(second.coverage.measured_components, 1);
                assert_eq!(second.coverage.attempted_components, 2);
                assert_eq!(second.coverage.native_text_controls, 1);
                first.text_layouts.extend(second.text_layouts);
                let combined = text_audit_coverage(ui.ctx(), &first.text_layouts);
                assert_eq!(combined.measured_components, 2);
                assert_eq!(combined.attempted_components, 2);
                assert_eq!(combined.native_text_controls, 1);
            })
            .textures_delta
            .clear();
    }

    #[test]
    fn repeated_passes_replace_publication_and_reset_the_inventory() {
        let context = egui::Context::default();
        crate::install_typography_fonts(&context);
        let mut publication = None;
        let mut pass_count = 0;
        let mut output = context.run_ui(Default::default(), |ui| {
            pass_count += 1;
            let mut presentation = PresentationContext::new(
                ui,
                tokens(),
                1.0,
                PresentationScope::new("retry"),
                SemanticUiId::root(),
            );
            if pass_count == 1 {
                presentation.content(ui, "provisional", "First pass", content(24));
                presentation.native(ui, NativeTextControlKind::Button, |ui| {
                    (ui.button("Provisional"), ())
                });
                ui.ctx().request_discard("exercise a repeated layout pass");
            } else {
                presentation.content(ui, "final", "Final pass", content(2));
            }
            publication = Some(presentation.finish(ui));
        });
        output.textures_delta.clear();
        assert_eq!(pass_count, 2);
        let publication = publication.unwrap();
        assert_eq!(publication.pass, 1);
        assert_eq!(publication.text_layouts.len(), 1);
        assert_eq!(publication.coverage.attempted_components, 1);
        assert_eq!(publication.coverage.failed_components, 0);
        assert_eq!(publication.coverage.native_text_controls, 0);
    }

    #[test]
    fn viewport_inventories_and_publications_are_independent() {
        let context = egui::Context::default();
        crate::install_typography_fonts(&context);
        let secondary = egui::ViewportId::from_hash_of("secondary");
        for (viewport, count) in [(egui::ViewportId::ROOT, 2), (secondary, 1)] {
            let mut output = context.run_ui(
                egui::RawInput {
                    viewport_id: viewport,
                    viewports: [(viewport, egui::ViewportInfo::default())]
                        .into_iter()
                        .collect(),
                    ..Default::default()
                },
                |ui| {
                    assert_eq!(text_audit_coverage(ui.ctx(), &[]).attempted_components, 0);
                    let mut presentation = PresentationContext::new(
                        ui,
                        tokens(),
                        1.0,
                        PresentationScope::new("viewport"),
                        SemanticUiId::root(),
                    );
                    presentation.content(ui, "title", "Viewport", content(1));
                    if count == 2 {
                        presentation.content(ui, "detail", "Root detail", content(1));
                    }
                    let publication = presentation.finish(ui);
                    assert_eq!(publication.viewport, viewport);
                    assert_eq!(publication.coverage.attempted_components, count);
                },
            );
            output.textures_delta.clear();
        }
    }

    #[test]
    fn publishing_a_context_from_an_earlier_pass_is_rejected() {
        let context = egui::Context::default();
        let mut stale = None;
        context
            .run_ui(Default::default(), |ui| {
                stale = Some(PresentationContext::new(
                    ui,
                    tokens(),
                    1.0,
                    PresentationScope::new("stale"),
                    SemanticUiId::root(),
                ));
            })
            .textures_delta
            .clear();
        context
            .run_ui(Default::default(), |ui| {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    stale.take().unwrap().finish(ui)
                }));
                assert!(result.is_err());
            })
            .textures_delta
            .clear();
    }

    #[test]
    fn scoped_actions_preserve_recipe_geometry_and_painted_text() {
        let mut rendered = Vec::new();
        for scoped in [false, true] {
            let context = egui::Context::default();
            crate::install_typography_fonts(&context);
            let mut bounds = None;
            let mut output = context.run_ui(Default::default(), |ui| {
                bounds = Some(if scoped {
                    let mut presentation = PresentationContext::new(
                        ui,
                        tokens(),
                        1.0,
                        PresentationScope::new("equivalence"),
                        SemanticUiId::root(),
                    );
                    presentation.action(ui, "undo", action()).rect
                } else {
                    crate::action_button(ui, action(), &tokens(), 1.0, &mut Vec::new()).rect
                });
            });
            output.textures_delta.clear();
            rendered.push((bounds, output.shapes));
        }
        assert_eq!(rendered[0], rendered[1]);
    }
}
