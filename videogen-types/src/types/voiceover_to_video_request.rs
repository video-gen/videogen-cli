pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceoverToVideoRequest {
    /// Opaque file id of an uploaded voiceover audio file (e.g. `vg_file_...`). Upload the file first via `POST /v1/files/upload`.
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>,
    #[serde(rename = "visualStyle")]
    pub visual_style: WorkflowVisualStyle,
    #[serde(rename = "visualPacing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_pacing: Option<VisualPacing>,
    /// Image generation quality tier for AI-generated visuals. LOW is fastest and cheapest; STANDARD balances quality and cost; HIGH is highest quality. Only applies when `visualStyle.type` is AI_IMAGE or ENTITY; STOCK pulls existing footage and is unaffected. Defaults to STANDARD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<VoiceoverToVideoRequestQuality>,
    /// Output language as a BCP-47 code (e.g. `en`, `es`, `fr`). Defaults to English.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Caption styling. Omit to use the default style with captions shown. Pass an object to override individual style fields (any omitted field uses the default). Pass `null` to hide captions entirely.
    #[serde(rename = "captionStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_style: Option<WorkflowCaptionStyle>,
    /// Optional file id of an uploaded logo image to overlay on the video (e.g. `vg_file_...`). Upload the image first via `POST /v1/files/upload`. Only image files are accepted.
    #[serde(rename = "logoFileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_file_id: Option<String>,
    /// Optional production notes for the AI that builds the video — visual direction for how to illustrate the voiceover (e.g. on-screen code or text to display, specific b-roll to feature, or scene-by-scene staging). Never spoken; does not change the uploaded voiceover audio or its transcript.
    #[serde(rename = "workflowAgentContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_agent_context: Option<String>,
    /// Optional edits applied to the project after the video is built, in order. Each action runs asynchronously; the response returns one remix action id per action. Captions and a logo are set with the `captionStyle` and `logoFileId` request fields above; recommended remix actions here are `CONVERT_IMAGES_TO_VIDEOS` to animate still images into clips, `ADD_TRANSITIONS` to stamp transitions between sections and assets, and `EDIT_WITH_AGENT` for open-ended natural-language edits. See the [Remix actions](/remix-actions) guide.
    #[serde(rename = "remixActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_actions: Option<Vec<RemixAction>>,
}

impl VoiceoverToVideoRequest {
    pub fn builder() -> VoiceoverToVideoRequestBuilder {
        <VoiceoverToVideoRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceoverToVideoRequestBuilder {
    file_id: Option<String>,
    aspect_ratio: Option<AspectRatio>,
    visual_style: Option<WorkflowVisualStyle>,
    visual_pacing: Option<VisualPacing>,
    quality: Option<VoiceoverToVideoRequestQuality>,
    language: Option<String>,
    caption_style: Option<WorkflowCaptionStyle>,
    logo_file_id: Option<String>,
    workflow_agent_context: Option<String>,
    remix_actions: Option<Vec<RemixAction>>,
}

impl VoiceoverToVideoRequestBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn aspect_ratio(mut self, value: AspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn visual_style(mut self, value: WorkflowVisualStyle) -> Self {
        self.visual_style = Some(value);
        self
    }

    pub fn visual_pacing(mut self, value: VisualPacing) -> Self {
        self.visual_pacing = Some(value);
        self
    }

    pub fn quality(mut self, value: VoiceoverToVideoRequestQuality) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn caption_style(mut self, value: WorkflowCaptionStyle) -> Self {
        self.caption_style = Some(value);
        self
    }

    pub fn logo_file_id(mut self, value: impl Into<String>) -> Self {
        self.logo_file_id = Some(value.into());
        self
    }

    pub fn workflow_agent_context(mut self, value: impl Into<String>) -> Self {
        self.workflow_agent_context = Some(value.into());
        self
    }

    pub fn remix_actions(mut self, value: Vec<RemixAction>) -> Self {
        self.remix_actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceoverToVideoRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](VoiceoverToVideoRequestBuilder::file_id)
    /// - [`visual_style`](VoiceoverToVideoRequestBuilder::visual_style)
    pub fn build(self) -> Result<VoiceoverToVideoRequest, BuildError> {
        Ok(VoiceoverToVideoRequest {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            aspect_ratio: self.aspect_ratio,
            visual_style: self.visual_style.ok_or_else(|| BuildError::missing_field("visual_style"))?,
            visual_pacing: self.visual_pacing,
            quality: self.quality,
            language: self.language,
            caption_style: self.caption_style,
            logo_file_id: self.logo_file_id,
            workflow_agent_context: self.workflow_agent_context,
            remix_actions: self.remix_actions,
        })
    }
}
