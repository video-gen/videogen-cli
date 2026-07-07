pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Caption styling. Any omitted field falls back to the VideoGen default caption style. Provide an empty object (`{}`) to keep the default style but ensure captions are shown. Pass `null` for the whole `captionStyle` field to hide captions entirely.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowCaptionStyle {
    /// Font family name.
    #[serde(rename = "fontName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_name: Option<String>,
    /// Font size in pixels at 1080p.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub font_size: Option<f64>,
    /// Numeric font weight (400 = regular, 700 = bold).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<i64>,
    #[serde(rename = "textColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<WorkflowRgbColor>,
    #[serde(rename = "textJustification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_justification: Option<WorkflowCaptionStyleTextJustification>,
    /// Vertical position of the caption block in the frame.
    #[serde(rename = "verticalAlignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_alignment: Option<WorkflowCaptionStyleVerticalAlignment>,
    /// Outline color around glyphs, or null for no outline.
    #[serde(rename = "strokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_color: Option<WorkflowRgbColor>,
    /// Outline thickness in pixels.
    #[serde(rename = "strokeWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stroke_weight: Option<f64>,
    /// Background drawn behind the text, or null for no background.
    #[serde(rename = "backgroundStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<WorkflowCaptionBackgroundStyle>,
    /// Color applied to the currently spoken word for karaoke-style highlighting, or null to keep the base text color.
    #[serde(rename = "spokenTextColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoken_text_color: Option<WorkflowRgbColor>,
    /// Outline color applied to the currently spoken word, or null.
    #[serde(rename = "spokenTextStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoken_text_stroke_color: Option<WorkflowRgbColor>,
    /// When true, a word keeps the spoken-text color after it has been spoken instead of reverting.
    #[serde(rename = "persistSpokenTextColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist_spoken_text_color: Option<bool>,
}

impl WorkflowCaptionStyle {
    pub fn builder() -> WorkflowCaptionStyleBuilder {
        <WorkflowCaptionStyleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowCaptionStyleBuilder {
    font_name: Option<String>,
    font_size: Option<f64>,
    font_weight: Option<i64>,
    text_color: Option<WorkflowRgbColor>,
    text_justification: Option<WorkflowCaptionStyleTextJustification>,
    vertical_alignment: Option<WorkflowCaptionStyleVerticalAlignment>,
    stroke_color: Option<WorkflowRgbColor>,
    stroke_weight: Option<f64>,
    background_style: Option<WorkflowCaptionBackgroundStyle>,
    spoken_text_color: Option<WorkflowRgbColor>,
    spoken_text_stroke_color: Option<WorkflowRgbColor>,
    persist_spoken_text_color: Option<bool>,
}

impl WorkflowCaptionStyleBuilder {
    pub fn font_name(mut self, value: impl Into<String>) -> Self {
        self.font_name = Some(value.into());
        self
    }

    pub fn font_size(mut self, value: f64) -> Self {
        self.font_size = Some(value);
        self
    }

    pub fn font_weight(mut self, value: i64) -> Self {
        self.font_weight = Some(value);
        self
    }

    pub fn text_color(mut self, value: WorkflowRgbColor) -> Self {
        self.text_color = Some(value);
        self
    }

    pub fn text_justification(mut self, value: WorkflowCaptionStyleTextJustification) -> Self {
        self.text_justification = Some(value);
        self
    }

    pub fn vertical_alignment(mut self, value: WorkflowCaptionStyleVerticalAlignment) -> Self {
        self.vertical_alignment = Some(value);
        self
    }

    pub fn stroke_color(mut self, value: WorkflowRgbColor) -> Self {
        self.stroke_color = Some(value);
        self
    }

    pub fn stroke_weight(mut self, value: f64) -> Self {
        self.stroke_weight = Some(value);
        self
    }

    pub fn background_style(mut self, value: WorkflowCaptionBackgroundStyle) -> Self {
        self.background_style = Some(value);
        self
    }

    pub fn spoken_text_color(mut self, value: WorkflowRgbColor) -> Self {
        self.spoken_text_color = Some(value);
        self
    }

    pub fn spoken_text_stroke_color(mut self, value: WorkflowRgbColor) -> Self {
        self.spoken_text_stroke_color = Some(value);
        self
    }

    pub fn persist_spoken_text_color(mut self, value: bool) -> Self {
        self.persist_spoken_text_color = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowCaptionStyle`].
    pub fn build(self) -> Result<WorkflowCaptionStyle, BuildError> {
        Ok(WorkflowCaptionStyle {
            font_name: self.font_name,
            font_size: self.font_size,
            font_weight: self.font_weight,
            text_color: self.text_color,
            text_justification: self.text_justification,
            vertical_alignment: self.vertical_alignment,
            stroke_color: self.stroke_color,
            stroke_weight: self.stroke_weight,
            background_style: self.background_style,
            spoken_text_color: self.spoken_text_color,
            spoken_text_stroke_color: self.spoken_text_stroke_color,
            persist_spoken_text_color: self.persist_spoken_text_color,
        })
    }
}
