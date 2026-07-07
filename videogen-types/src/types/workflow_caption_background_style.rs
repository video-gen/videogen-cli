pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Background drawn behind caption text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowCaptionBackgroundStyle {
    /// RECT draws one rectangle behind the whole line; WRAPPED hugs the text; WORD_BY_WORD draws a box per word.
    pub r#type: WorkflowCaptionBackgroundStyleType,
    #[serde(rename = "backgroundColor")]
    #[serde(default)]
    pub background_color: WorkflowRgbColor,
    /// Corner rounding as a proportion of the background height (0 = square corners).
    #[serde(rename = "borderRadiusProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub border_radius_proportion: Option<f64>,
    /// Background opacity from 0 (transparent) to 1 (opaque).
    #[serde(rename = "opacityProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub opacity_proportion: Option<f64>,
}

impl WorkflowCaptionBackgroundStyle {
    pub fn builder() -> WorkflowCaptionBackgroundStyleBuilder {
        <WorkflowCaptionBackgroundStyleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowCaptionBackgroundStyleBuilder {
    r#type: Option<WorkflowCaptionBackgroundStyleType>,
    background_color: Option<WorkflowRgbColor>,
    border_radius_proportion: Option<f64>,
    opacity_proportion: Option<f64>,
}

impl WorkflowCaptionBackgroundStyleBuilder {
    pub fn r#type(mut self, value: WorkflowCaptionBackgroundStyleType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn background_color(mut self, value: WorkflowRgbColor) -> Self {
        self.background_color = Some(value);
        self
    }

    pub fn border_radius_proportion(mut self, value: f64) -> Self {
        self.border_radius_proportion = Some(value);
        self
    }

    pub fn opacity_proportion(mut self, value: f64) -> Self {
        self.opacity_proportion = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowCaptionBackgroundStyle`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](WorkflowCaptionBackgroundStyleBuilder::r#type)
    /// - [`background_color`](WorkflowCaptionBackgroundStyleBuilder::background_color)
    pub fn build(self) -> Result<WorkflowCaptionBackgroundStyle, BuildError> {
        Ok(WorkflowCaptionBackgroundStyle {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            background_color: self.background_color.ok_or_else(|| BuildError::missing_field("background_color"))?,
            border_radius_proportion: self.border_radius_proportion,
            opacity_proportion: self.opacity_proportion,
        })
    }
}
