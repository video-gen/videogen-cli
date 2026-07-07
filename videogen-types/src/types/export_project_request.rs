pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportProjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ExportProjectQuality>,
}

impl ExportProjectRequest {
    pub fn builder() -> ExportProjectRequestBuilder {
        <ExportProjectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportProjectRequestBuilder {
    quality: Option<ExportProjectQuality>,
}

impl ExportProjectRequestBuilder {
    pub fn quality(mut self, value: ExportProjectQuality) -> Self {
        self.quality = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportProjectRequest`].
    pub fn build(self) -> Result<ExportProjectRequest, BuildError> {
        Ok(ExportProjectRequest {
            quality: self.quality,
        })
    }
}

