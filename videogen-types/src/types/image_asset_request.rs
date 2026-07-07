pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImageAssetRequest {
    /// File id of the source image (e.g. `vg_file_...`). Upload a file first via `POST /v1/files/upload`, then pass the returned id here.
    #[serde(rename = "imageStorageFileId")]
    #[serde(default)]
    pub image_storage_file_id: String,
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

impl ImageAssetRequest {
    pub fn builder() -> ImageAssetRequestBuilder {
        <ImageAssetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageAssetRequestBuilder {
    image_storage_file_id: Option<String>,
    watermark_mode: Option<WatermarkMode>,
    num_results: Option<i64>,
    is_output_temporary: Option<bool>,
}

impl ImageAssetRequestBuilder {
    pub fn image_storage_file_id(mut self, value: impl Into<String>) -> Self {
        self.image_storage_file_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ImageAssetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`image_storage_file_id`](ImageAssetRequestBuilder::image_storage_file_id)
    pub fn build(self) -> Result<ImageAssetRequest, BuildError> {
        Ok(ImageAssetRequest {
            image_storage_file_id: self.image_storage_file_id.ok_or_else(|| BuildError::missing_field("image_storage_file_id"))?,
            watermark_mode: self.watermark_mode,
            num_results: self.num_results,
            is_output_temporary: self.is_output_temporary,
        })
    }
}
