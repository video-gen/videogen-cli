pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SlideshowToVideoRequest {
    /// Opaque file id of an uploaded PDF or PowerPoint file (e.g. `vg_file_...`). Upload the file first via `POST /v1/files/upload`.
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    /// Optional per-slide narration, in slide order, applied by index: each slide uses its matching entry, and an empty string makes that slide silent. If you provide fewer entries than slides, the remaining slides are silent; extra entries are ignored. Omit this field entirely to narrate each slide from its speaker notes in the uploaded file. To guarantee no narration on any slide, pass an empty array.
    #[serde(rename = "slideScripts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slide_scripts: Option<Vec<String>>,
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>,
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
    /// Caption styling. Omit to use the default style with captions shown. Pass an object to override individual style fields (any omitted field uses the default). Pass `null` to hide captions entirely.
    #[serde(rename = "captionStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_style: Option<WorkflowCaptionStyle>,
    /// Optional file id of an uploaded logo image to overlay on the video (e.g. `vg_file_...`). Upload the image first via `POST /v1/files/upload`. Only image files are accepted.
    #[serde(rename = "logoFileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_file_id: Option<String>,
    /// Optional edits applied to the project after the video is built, in order. Each action runs asynchronously; the response returns one remix action id per action. Captions and a logo are set with the `captionStyle` and `logoFileId` request fields above; recommended remix actions here are `CONVERT_IMAGES_TO_VIDEOS` to animate still images into clips, `ADD_TRANSITIONS` to stamp transitions between sections and assets, and `EDIT_WITH_AGENT` for open-ended natural-language edits. See the [Remix actions](/remix-actions) guide.
    #[serde(rename = "remixActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_actions: Option<Vec<RemixAction>>,
}

impl SlideshowToVideoRequest {
    pub fn builder() -> SlideshowToVideoRequestBuilder {
        <SlideshowToVideoRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlideshowToVideoRequestBuilder {
    file_id: Option<String>,
    slide_scripts: Option<Vec<String>>,
    aspect_ratio: Option<AspectRatio>,
    language: Option<String>,
    voice_id: Option<String>,
    voice_speed: Option<f64>,
    avatar_presenter_id: Option<String>,
    caption_style: Option<WorkflowCaptionStyle>,
    logo_file_id: Option<String>,
    remix_actions: Option<Vec<RemixAction>>,
}

impl SlideshowToVideoRequestBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn slide_scripts(mut self, value: Vec<String>) -> Self {
        self.slide_scripts = Some(value);
        self
    }

    pub fn aspect_ratio(mut self, value: AspectRatio) -> Self {
        self.aspect_ratio = Some(value);
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

    pub fn caption_style(mut self, value: WorkflowCaptionStyle) -> Self {
        self.caption_style = Some(value);
        self
    }

    pub fn logo_file_id(mut self, value: impl Into<String>) -> Self {
        self.logo_file_id = Some(value.into());
        self
    }

    pub fn remix_actions(mut self, value: Vec<RemixAction>) -> Self {
        self.remix_actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SlideshowToVideoRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](SlideshowToVideoRequestBuilder::file_id)
    pub fn build(self) -> Result<SlideshowToVideoRequest, BuildError> {
        Ok(SlideshowToVideoRequest {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            slide_scripts: self.slide_scripts,
            aspect_ratio: self.aspect_ratio,
            language: self.language,
            voice_id: self.voice_id,
            voice_speed: self.voice_speed,
            avatar_presenter_id: self.avatar_presenter_id,
            caption_style: self.caption_style,
            logo_file_id: self.logo_file_id,
            remix_actions: self.remix_actions,
        })
    }
}
