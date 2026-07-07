pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GenerateImageRequest {
    /// Text prompt describing the image to generate. When reference images are provided, the prompt describes the desired transformation.
    #[serde(default)]
    pub prompt: String,
    /// Optional file ids of reference images (e.g. `["vg_file_..."]`). Upload files first via `POST /v1/files/upload`, then pass the returned ids here. Maximum 4 images. When provided, the model uses these as guidance for generation.
    #[serde(rename = "imageFileIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_file_ids: Option<Vec<String>>,
    /// Aspect ratio for the generated image. Defaults to 16:9 when omitted.
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>,
    /// Image generation quality tier. LOW is fastest; HIGH is slowest and highest quality.
    pub quality: GenerateImageRequestQuality,
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

impl GenerateImageRequest {
    pub fn builder() -> GenerateImageRequestBuilder {
        <GenerateImageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateImageRequestBuilder {
    prompt: Option<String>,
    image_file_ids: Option<Vec<String>>,
    aspect_ratio: Option<AspectRatio>,
    quality: Option<GenerateImageRequestQuality>,
    watermark_mode: Option<WatermarkMode>,
    num_results: Option<i64>,
    is_output_temporary: Option<bool>,
}

impl GenerateImageRequestBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn image_file_ids(mut self, value: Vec<String>) -> Self {
        self.image_file_ids = Some(value);
        self
    }

    pub fn aspect_ratio(mut self, value: AspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn quality(mut self, value: GenerateImageRequestQuality) -> Self {
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

    /// Consumes the builder and constructs a [`GenerateImageRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](GenerateImageRequestBuilder::prompt)
    /// - [`quality`](GenerateImageRequestBuilder::quality)
    pub fn build(self) -> Result<GenerateImageRequest, BuildError> {
        Ok(GenerateImageRequest {
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            image_file_ids: self.image_file_ids,
            aspect_ratio: self.aspect_ratio,
            quality: self.quality.ok_or_else(|| BuildError::missing_field("quality"))?,
            watermark_mode: self.watermark_mode,
            num_results: self.num_results,
            is_output_temporary: self.is_output_temporary,
        })
    }
}

