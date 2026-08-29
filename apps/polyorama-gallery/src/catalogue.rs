use std::{fmt, str::FromStr};

use polyorama_ui_egui::{AppearancePreference, DensityPreference};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StoryId {
    ButtonDefault,
    ButtonDisabled,
    ButtonKeyboardFocus,
    TabsManyLongLabels,
    TabsNarrow,
    SplitterHoverActive,
    ToolbarNarrow,
    PropertyRowLongValue,
    StatusErrorLongMessage,
    VirtualGridLoading,
    VirtualGridPartial,
    ReferenceApplicationShell,
    ReferenceImageToolbarWide,
    ReferenceImageToolbarNarrow,
    ReferenceInspector,
    ReferenceResults,
    ReferenceThumbnails,
    ReferenceDiagnostics,
}

impl StoryId {
    pub const ALL: [Self; 18] = [
        Self::ButtonDefault,
        Self::ButtonDisabled,
        Self::ButtonKeyboardFocus,
        Self::TabsManyLongLabels,
        Self::TabsNarrow,
        Self::SplitterHoverActive,
        Self::ToolbarNarrow,
        Self::PropertyRowLongValue,
        Self::StatusErrorLongMessage,
        Self::VirtualGridLoading,
        Self::VirtualGridPartial,
        Self::ReferenceApplicationShell,
        Self::ReferenceImageToolbarWide,
        Self::ReferenceImageToolbarNarrow,
        Self::ReferenceInspector,
        Self::ReferenceResults,
        Self::ReferenceThumbnails,
        Self::ReferenceDiagnostics,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ButtonDefault => "button/default",
            Self::ButtonDisabled => "button/disabled",
            Self::ButtonKeyboardFocus => "button/keyboard-focus",
            Self::TabsManyLongLabels => "tabs/many-long-labels",
            Self::TabsNarrow => "tabs/narrow",
            Self::SplitterHoverActive => "splitter/hover-active",
            Self::ToolbarNarrow => "toolbar/narrow",
            Self::PropertyRowLongValue => "property-row/long-value",
            Self::StatusErrorLongMessage => "status/error-long-message",
            Self::VirtualGridLoading => "virtual-grid/loading",
            Self::VirtualGridPartial => "virtual-grid/partial",
            Self::ReferenceApplicationShell => "reference/application-shell",
            Self::ReferenceImageToolbarWide => "reference/image-toolbar-wide",
            Self::ReferenceImageToolbarNarrow => "reference/image-toolbar-narrow",
            Self::ReferenceInspector => "reference/inspector",
            Self::ReferenceResults => "reference/results",
            Self::ReferenceThumbnails => "reference/thumbnails",
            Self::ReferenceDiagnostics => "reference/diagnostics",
        }
    }
}

impl fmt::Display for StoryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for StoryId {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|story| story.as_str() == value)
            .ok_or_else(|| format!("unknown Polyorama gallery story {value:?}"))
    }
}

impl Serialize for StoryId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for StoryId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StoryGroup {
    Button,
    Dock,
    Toolbar,
    Property,
    Status,
    VirtualGrid,
    Reference,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionScenario {
    Pointer,
    Keyboard,
    Accessibility,
    Drag,
    Scroll,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct RecommendedViewport {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct StoryDefinition {
    pub id: StoryId,
    pub description: &'static str,
    pub group: StoryGroup,
    pub recommended_viewport: RecommendedViewport,
    pub appearances: &'static [AppearancePreference],
    pub densities: &'static [DensityPreference],
    pub interactions: &'static [InteractionScenario],
}

const APPEARANCES: &[AppearancePreference] =
    &[AppearancePreference::Light, AppearancePreference::Dark];
const DENSITIES: &[DensityPreference] =
    &[DensityPreference::Compact, DensityPreference::Comfortable];
const POINTER: &[InteractionScenario] = &[InteractionScenario::Pointer];
const KEYBOARD: &[InteractionScenario] = &[
    InteractionScenario::Pointer,
    InteractionScenario::Keyboard,
    InteractionScenario::Accessibility,
];
const DRAG: &[InteractionScenario] = &[
    InteractionScenario::Pointer,
    InteractionScenario::Keyboard,
    InteractionScenario::Accessibility,
    InteractionScenario::Drag,
];
const SCROLL: &[InteractionScenario] = &[
    InteractionScenario::Pointer,
    InteractionScenario::Keyboard,
    InteractionScenario::Scroll,
];
const PASSIVE: &[InteractionScenario] = &[];

const fn story(
    id: StoryId,
    description: &'static str,
    group: StoryGroup,
    width: u16,
    height: u16,
    interactions: &'static [InteractionScenario],
) -> StoryDefinition {
    StoryDefinition {
        id,
        description,
        group,
        recommended_viewport: RecommendedViewport { width, height },
        appearances: APPEARANCES,
        densities: DENSITIES,
        interactions,
    }
}

pub static STORIES: [StoryDefinition; 18] = [
    story(
        StoryId::ButtonDefault,
        "Default, primary and selected action states.",
        StoryGroup::Button,
        520,
        180,
        POINTER,
    ),
    story(
        StoryId::ButtonDisabled,
        "Disabled action with complete semantics and unchanged geometry.",
        StoryGroup::Button,
        360,
        160,
        PASSIVE,
    ),
    story(
        StoryId::ButtonKeyboardFocus,
        "Visible keyboard focus on the production action button.",
        StoryGroup::Button,
        360,
        160,
        KEYBOARD,
    ),
    story(
        StoryId::TabsManyLongLabels,
        "Measured long dock tabs with whole-target overflow.",
        StoryGroup::Dock,
        760,
        300,
        DRAG,
    ),
    story(
        StoryId::TabsNarrow,
        "Overflow-only and narrow active-tab dock states.",
        StoryGroup::Dock,
        320,
        300,
        KEYBOARD,
    ),
    story(
        StoryId::SplitterHoverActive,
        "Production splitter hover, drag and keyboard adjustment.",
        StoryGroup::Dock,
        640,
        320,
        DRAG,
    ),
    story(
        StoryId::ToolbarNarrow,
        "Priority-preserving narrow image toolbar.",
        StoryGroup::Toolbar,
        320,
        180,
        KEYBOARD,
    ),
    story(
        StoryId::PropertyRowLongValue,
        "Stacked and aligned property treatment for a long value.",
        StoryGroup::Property,
        360,
        220,
        PASSIVE,
    ),
    story(
        StoryId::StatusErrorLongMessage,
        "Bounded, wrapping diagnostic error treatment.",
        StoryGroup::Status,
        420,
        220,
        PASSIVE,
    ),
    story(
        StoryId::VirtualGridLoading,
        "Bounded loading cells without materialising a collection.",
        StoryGroup::VirtualGrid,
        640,
        360,
        SCROLL,
    ),
    story(
        StoryId::VirtualGridPartial,
        "Mixed resident, loading, empty and error cells.",
        StoryGroup::VirtualGrid,
        640,
        360,
        SCROLL,
    ),
    story(
        StoryId::ReferenceApplicationShell,
        "Application bar and canonical dock shell reference.",
        StoryGroup::Reference,
        960,
        540,
        DRAG,
    ),
    story(
        StoryId::ReferenceImageToolbarWide,
        "Wide analytical image-toolbar reference.",
        StoryGroup::Reference,
        760,
        260,
        KEYBOARD,
    ),
    story(
        StoryId::ReferenceImageToolbarNarrow,
        "Narrow analytical image-toolbar reference.",
        StoryGroup::Reference,
        320,
        260,
        KEYBOARD,
    ),
    story(
        StoryId::ReferenceInspector,
        "Representative inspector with deterministic data.",
        StoryGroup::Reference,
        420,
        500,
        PASSIVE,
    ),
    story(
        StoryId::ReferenceResults,
        "Deterministic result columns and selected row.",
        StoryGroup::Reference,
        760,
        360,
        KEYBOARD,
    ),
    story(
        StoryId::ReferenceThumbnails,
        "Resident and progressive thumbnail states.",
        StoryGroup::Reference,
        760,
        420,
        SCROLL,
    ),
    story(
        StoryId::ReferenceDiagnostics,
        "Long diagnostics labels and large numeric values.",
        StoryGroup::Reference,
        640,
        560,
        PASSIVE,
    ),
];

pub fn story_definition(id: StoryId) -> &'static StoryDefinition {
    STORIES
        .iter()
        .find(|story| story.id == id)
        .expect("every typed story ID is registered")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn registry_is_complete_unique_stable_and_serialisable() {
        assert_eq!(STORIES.len(), StoryId::ALL.len());
        let ids: BTreeSet<_> = STORIES.iter().map(|story| story.id.as_str()).collect();
        assert_eq!(ids.len(), STORIES.len());
        for id in StoryId::ALL {
            let definition = story_definition(id);
            assert!(!definition.description.is_empty());
            assert!(definition.recommended_viewport.width >= 320);
            assert!(definition.recommended_viewport.height >= 160);
            assert_eq!(id.as_str().parse::<StoryId>().unwrap(), id);
            assert_eq!(serde_json::to_value(id).unwrap(), id.as_str());
        }
    }
}
