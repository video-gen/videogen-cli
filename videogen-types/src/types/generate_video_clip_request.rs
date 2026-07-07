pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GenerateVideoClipRequest {
    /// Text prompt describing the video to generate. Optional when reference media is provided. Describe the video in plain language; any reference media you provide is incorporated automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Optional file ids of reference images (e.g. `["vg_file_..."]`). Upload files first via `POST /v1/files/upload`, then pass the returned ids here. When provided, the images are animated or used as visual guidance for the generated video.
    #[serde(rename = "imageFileIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_file_ids: Option<Vec<String>>,
    /// Optional file ids of reference videos (e.g. `["vg_file_..."]`). Upload files first via `POST /v1/files/upload`, then pass the returned ids here. They are used as motion or style guidance for the generated video.
    #[serde(rename = "videoFileIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_file_ids: Option<Vec<String>>,
    /// Optional file ids of reference audio clips (e.g. `["vg_file_..."]`) used for native lip-sync and soundtrack. Upload files first via `POST /v1/files/upload`, then pass the returned ids here.
    #[serde(rename = "audioFileIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_file_ids: Option<Vec<String>>,
    /// When true, the generated video is guaranteed to include audio. When false, audio may still be present. Defaults to false.
    #[serde(rename = "generateAudio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_audio: Option<bool>,
    /// Desired clip length in seconds. A whole number between 1 and 15. Defaults to 6 when omitted. This endpoint produces a single short clip. For longer, multi-scene, professionally edited videos, use a video workflow such as `POST /v1/workflows/script-to-video`.
    #[serde(rename = "durationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// Aspect ratio for the generated video. Defaults to 16:9 when omitted.
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>,
    /// Video generation quality tier. STANDARD is fastest; HIGH is slowest and highest quality.
    pub quality: GenerateVideoClipRequestQuality,
    #[serde(rename = "watermarkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark_mode: Option<WatermarkMode>,
    /// Number of output results to generate. Defaults to 1.
    #[serde(rename = "numResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// When true, generated files are temporary. Temporary files are guaranteed to be available for 24 hours, after which they may be archived at any time. Temporary files are not analyzed (no description, transcript, or embedding will be generated), so they will not appear in search results. Defaults to false.
    #[serde(rename = "isOutputTemporary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_output_temporary: Option<bool>,
}

impl GenerateVideoClipRequest {
    pub fn builder() -> GenerateVideoClipRequestBuilder {
        <GenerateVideoClipRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateVideoClipRequestBuilder {
    prompt: Option<String>,
    image_file_ids: Option<Vec<String>>,
    video_file_ids: Option<Vec<String>>,
    audio_file_ids: Option<Vec<String>>,
    generate_audio: Option<bool>,
    duration_seconds: Option<i64>,
    aspect_ratio: Option<AspectRatio>,
    quality: Option<GenerateVideoClipRequestQuality>,
    watermark_mode: Option<WatermarkMode>,
    num_results: Option<i64>,
    is_output_temporary: Option<bool>,
}

impl GenerateVideoClipRequestBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn image_file_ids(mut self, value: Vec<String>) -> Self {
        self.image_file_ids = Some(value);
        self
    }

    pub fn video_file_ids(mut self, value: Vec<String>) -> Self {
        self.video_file_ids = Some(value);
        self
    }

    pub fn audio_file_ids(mut self, value: Vec<String>) -> Self {
        self.audio_file_ids = Some(value);
        self
    }

    pub fn generate_audio(mut self, value: bool) -> Self {
        self.generate_audio = Some(value);
        self
    }

    pub fn duration_seconds(mut self, value: i64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn aspect_ratio(mut self, value: AspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn quality(mut self, value: GenerateVideoClipRequestQuality) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn watermark_mode(mut self, value: WatermarkMode) -> Self {
        self.watermark_mode = Some(value);
        self
    }

    pub fn num_results(mut self, value: i64) -> Self {
        self.num_results = Some(value);
        self
    }

    pub fn is_output_temporary(mut self, value: bool) -> Self {
        self.is_output_temporary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GenerateVideoClipRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quality`](GenerateVideoClipRequestBuilder::quality)
    pub fn build(self) -> Result<GenerateVideoClipRequest, BuildError> {
        Ok(GenerateVideoClipRequest {
            prompt: self.prompt,
            image_file_ids: self.image_file_ids,
            video_file_ids: self.video_file_ids,
            audio_file_ids: self.audio_file_ids,
            generate_audio: self.generate_audio,
            duration_seconds: self.duration_seconds,
            aspect_ratio: self.aspect_ratio,
            quality: self.quality.ok_or_else(|| BuildError::missing_field("quality"))?,
            watermark_mode: self.watermark_mode,
            num_results: self.num_results,
            is_output_temporary: self.is_output_temporary,
        })
    }
}

