pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptToVideoRequest {
    /// The narration script, used verbatim. This exact text is narrated and turned into a video — it is not rewritten or expanded.
    #[serde(default)]
    pub script: String,
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
    pub quality: Option<ScriptToVideoRequestQuality>,
    /// Output language as a BCP-47 code (e.g. `en`, `es`, `fr`). Defaults to English.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Voice id from `GET /v1/resources/tts-voices` (e.g. `vg_voic_...`). A default voice is used when omitted. Any voice may be used here, including voices where `supportsDirectToolExecution` is false.
    #[serde(rename = "voiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// Speech rate multiplier. Defaults to the voice's default speed.
    #[serde(rename = "voiceSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub voice_speed: Option<f64>,
    /// Optional avatar presenter id from `GET /v1/resources/avatar-presenters` (e.g. `vg_pres_...`). When set, the narration is delivered by a talking-head presenter avatar. Pass your `voiceId` to that endpoint to list presenters sorted by best match for the voice. Omit for a standard voiceover with no presenter.
    #[serde(rename = "avatarPresenterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_presenter_id: Option<String>,
    /// Optional file ids of images or videos to feature as b-roll (e.g. `["vg_file_..."]`). Upload files first via `POST /v1/files/upload`. Only image and video files are accepted.
    #[serde(rename = "featuredBRollFileIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_b_roll_file_ids: Option<Vec<String>>,
    /// Optional production notes for the AI that builds the video — visual direction that should not appear in the spoken narration (e.g. on-screen code or text to display, specific b-roll to feature, or scene-by-scene staging). Never spoken; keep the narration itself in `script`.
    #[serde(rename = "workflowAgentContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_agent_context: Option<String>,
    /// Optional edits applied to the project after the video is built, in order. Each action runs asynchronously; the response returns one remix action id per action. Recommended for script-to-video: `ENABLE_CAPTIONS` to show and style captions, `CONVERT_IMAGES_TO_VIDEOS` to animate still images into clips, `ADD_TRANSITIONS` to stamp transitions between sections, and `SET_LOGO` to overlay a logo (this workflow has no native caption-style or logo fields). `EDIT_WITH_AGENT` applies open-ended natural-language edits. See the [Remix actions](/remix-actions) guide.
    #[serde(rename = "remixActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_actions: Option<Vec<RemixAction>>,
}

impl ScriptToVideoRequest {
    pub fn builder() -> ScriptToVideoRequestBuilder {
        <ScriptToVideoRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScriptToVideoRequestBuilder {
    script: Option<String>,
    aspect_ratio: Option<AspectRatio>,
    visual_style: Option<WorkflowVisualStyle>,
    visual_pacing: Option<VisualPacing>,
    quality: Option<ScriptToVideoRequestQuality>,
    language: Option<String>,
    voice_id: Option<String>,
    voice_speed: Option<f64>,
    avatar_presenter_id: Option<String>,
    featured_b_roll_file_ids: Option<Vec<String>>,
    workflow_agent_context: Option<String>,
    remix_actions: Option<Vec<RemixAction>>,
}

impl ScriptToVideoRequestBuilder {
    pub fn script(mut self, value: impl Into<String>) -> Self {
        self.script = Some(value.into());
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

    pub fn quality(mut self, value: ScriptToVideoRequestQuality) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn voice_speed(mut self, value: f64) -> Self {
        self.voice_speed = Some(value);
        self
    }

    pub fn avatar_presenter_id(mut self, value: impl Into<String>) -> Self {
        self.avatar_presenter_id = Some(value.into());
        self
    }

    pub fn featured_b_roll_file_ids(mut self, value: Vec<String>) -> Self {
        self.featured_b_roll_file_ids = Some(value);
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

    /// Consumes the builder and constructs a [`ScriptToVideoRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`script`](ScriptToVideoRequestBuilder::script)
    /// - [`visual_style`](ScriptToVideoRequestBuilder::visual_style)
    pub fn build(self) -> Result<ScriptToVideoRequest, BuildError> {
        Ok(ScriptToVideoRequest {
            script: self.script.ok_or_else(|| BuildError::missing_field("script"))?,
            aspect_ratio: self.aspect_ratio,
            visual_style: self.visual_style.ok_or_else(|| BuildError::missing_field("visual_style"))?,
            visual_pacing: self.visual_pacing,
            quality: self.quality,
            language: self.language,
            voice_id: self.voice_id,
            voice_speed: self.voice_speed,
            avatar_presenter_id: self.avatar_presenter_id,
            featured_b_roll_file_ids: self.featured_b_roll_file_ids,
            workflow_agent_context: self.workflow_agent_context,
            remix_actions: self.remix_actions,
        })
    }
}
