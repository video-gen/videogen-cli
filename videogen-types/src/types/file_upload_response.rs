pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileUploadResponse {
    /// The file id to use in subsequent API calls (e.g. `vg_file_...`).
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    /// Pre-signed URL. PUT the raw file bytes to this URL to complete the upload.
    #[serde(rename = "uploadUrl")]
    #[serde(default)]
    pub upload_url: String,
}

impl FileUploadResponse {
    pub fn builder() -> FileUploadResponseBuilder {
        <FileUploadResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileUploadResponseBuilder {
    file_id: Option<String>,
    upload_url: Option<String>,
}

impl FileUploadResponseBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn upload_url(mut self, value: impl Into<String>) -> Self {
        self.upload_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FileUploadResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](FileUploadResponseBuilder::file_id)
    /// - [`upload_url`](FileUploadResponseBuilder::upload_url)
    pub fn build(self) -> Result<FileUploadResponse, BuildError> {
        Ok(FileUploadResponse {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            upload_url: self.upload_url.ok_or_else(|| BuildError::missing_field("upload_url"))?,
        })
    }
}
