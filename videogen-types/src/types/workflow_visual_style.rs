pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Visual treatment for the generated b-roll.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkflowVisualStyle {
    /// STOCK pulls stock footage and images. AI_IMAGE generates a styled image for each section. ENTITY generates images that match a visual-style entity's reference images for a consistent look.
    pub r#type: WorkflowVisualStyleType,
    /// Only applies when type is AI_IMAGE. A free-form description of the look applied to every generated image (e.g. `vintage 1970s film photography, warm grain`). See the AI styles reference for example descriptions of the app's default styles. Required when type is AI_IMAGE.
    #[serde(rename = "aiStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_style: Option<String>,
    /// Only applies when type is ENTITY. The id of a VISUAL_STYLE entity (e.g. `vg_enti_...`) whose reference images guide every generated image. Required when type is ENTITY.
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// Only applies when type is AI_IMAGE. When true, featured b-roll images you provide are re-rendered in the chosen style so they match the generated look (no effect on featured b-roll videos). Defaults to true.
    #[serde(rename = "restyleFeaturedBRollWithAiStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restyle_featured_b_roll_with_ai_style: Option<bool>,
}

impl WorkflowVisualStyle {
    pub fn builder() -> WorkflowVisualStyleBuilder {
        <WorkflowVisualStyleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowVisualStyleBuilder {
    r#type: Option<WorkflowVisualStyleType>,
    ai_style: Option<String>,
    entity_id: Option<String>,
    restyle_featured_b_roll_with_ai_style: Option<bool>,
}

impl WorkflowVisualStyleBuilder {
    pub fn r#type(mut self, value: WorkflowVisualStyleType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn ai_style(mut self, value: impl Into<String>) -> Self {
        self.ai_style = Some(value.into());
        self
    }

    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    pub fn restyle_featured_b_roll_with_ai_style(mut self, value: bool) -> Self {
        self.restyle_featured_b_roll_with_ai_style = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowVisualStyle`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](WorkflowVisualStyleBuilder::r#type)
    pub fn build(self) -> Result<WorkflowVisualStyle, BuildError> {
        Ok(WorkflowVisualStyle {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            ai_style: self.ai_style,
            entity_id: self.entity_id,
            restyle_featured_b_roll_with_ai_style: self.restyle_featured_b_roll_with_ai_style,
        })
    }
}
