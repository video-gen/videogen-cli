pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemoveEntityReferenceRequest {
    /// The file id (e.g. `vg_file_...`) of the reference to remove.
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
}

impl RemoveEntityReferenceRequest {
    pub fn builder() -> RemoveEntityReferenceRequestBuilder {
        <RemoveEntityReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveEntityReferenceRequestBuilder {
    file_id: Option<String>,
}

impl RemoveEntityReferenceRequestBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RemoveEntityReferenceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](RemoveEntityReferenceRequestBuilder::file_id)
    pub fn build(self) -> Result<RemoveEntityReferenceRequest, BuildError> {
        Ok(RemoveEntityReferenceRequest {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}

