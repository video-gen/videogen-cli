pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFileUploadRequest {
    /// The type of file to upload. Optional; when omitted, the type is inferred after upload processing completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CreateFileUploadRequestType>,
    /// Display name for the uploaded file.
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    /// When true, the file is temporary. Temporary files are guaranteed to be available for 24 hours, after which they may be archived at any time. Temporary files are not analyzed (no description, transcript, or embedding will be generated), so they will not appear in search results. Defaults to false.
    #[serde(rename = "isTemporary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_temporary: Option<bool>,
}

impl CreateFileUploadRequest {
    pub fn builder() -> CreateFileUploadRequestBuilder {
        <CreateFileUploadRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFileUploadRequestBuilder {
    r#type: Option<CreateFileUploadRequestType>,
    display_name: Option<String>,
    is_temporary: Option<bool>,
}

impl CreateFileUploadRequestBuilder {
    pub fn r#type(mut self, value: CreateFileUploadRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn is_temporary(mut self, value: bool) -> Self {
        self.is_temporary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateFileUploadRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](CreateFileUploadRequestBuilder::display_name)
    pub fn build(self) -> Result<CreateFileUploadRequest, BuildError> {
        Ok(CreateFileUploadRequest {
            r#type: self.r#type,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            is_temporary: self.is_temporary,
        })
    }
}

