pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A rendition source for a file (e.g. thumbnail, preview, download). Contains a signed URL and metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileSource {
    /// `pending`: asset is still processing or has not been hydrated yet. `ready`: signed URL is available. `failed`: rendition generation failed. `skipped`: rendition does not apply to this file type (e.g. thumbnail for audio).
    pub status: FileSourceStatus,
    /// Signed URL. Present when status is `ready` and file has been recently hydrated. If missing, call the hydrate endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Seconds since epoch (Unix timestamp) when the signed URL expires.
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    /// Rendition width in pixels, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Rendition height in pixels, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// File size in bytes, when known.
    #[serde(rename = "fileBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_bytes: Option<i64>,
}

impl FileSource {
    pub fn builder() -> FileSourceBuilder {
        <FileSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileSourceBuilder {
    status: Option<FileSourceStatus>,
    url: Option<String>,
    expires_at: Option<f64>,
    width: Option<i64>,
    height: Option<i64>,
    file_bytes: Option<i64>,
}

impl FileSourceBuilder {
    pub fn status(mut self, value: FileSourceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: f64) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn file_bytes(mut self, value: i64) -> Self {
        self.file_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](FileSourceBuilder::status)
    pub fn build(self) -> Result<FileSource, BuildError> {
        Ok(FileSource {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            url: self.url,
            expires_at: self.expires_at,
            width: self.width,
            height: self.height,
            file_bytes: self.file_bytes,
        })
    }
}
