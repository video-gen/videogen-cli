pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddEntityReferenceRequest {
    /// The file id (e.g. `vg_file_...`) of an image to attach as a reference.
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    /// Optional description of the reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When true, make this the entity's primary reference (used for its thumbnail).
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl AddEntityReferenceRequest {
    pub fn builder() -> AddEntityReferenceRequestBuilder {
        <AddEntityReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddEntityReferenceRequestBuilder {
    file_id: Option<String>,
    description: Option<String>,
    is_default: Option<bool>,
}

impl AddEntityReferenceRequestBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddEntityReferenceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](AddEntityReferenceRequestBuilder::file_id)
    pub fn build(self) -> Result<AddEntityReferenceRequest, BuildError> {
        Ok(AddEntityReferenceRequest {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            description: self.description,
            is_default: self.is_default,
        })
    }
}

