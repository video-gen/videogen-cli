pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Delivered to your webhook endpoint during the file upload lifecycle. Only sent for files uploaded via the API. The payload always includes a hydrated `file` object with the latest state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileUploadWebhookPayload {
    pub event: FileUploadWebhookEventName,
    /// File id (e.g. `vg_file_...`).
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    /// Seconds since epoch (Unix timestamp) when the event occurred.
    #[serde(rename = "occurredAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub occurred_at: f64,
    /// Hydrated file object with the latest state at the time of the event.
    pub file: StorageFile,
    /// Error details. Present only on `file.upload.failed` and `file.analysis_failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl FileUploadWebhookPayload {
    pub fn builder() -> FileUploadWebhookPayloadBuilder {
        <FileUploadWebhookPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileUploadWebhookPayloadBuilder {
    event: Option<FileUploadWebhookEventName>,
    file_id: Option<String>,
    occurred_at: Option<f64>,
    file: Option<StorageFile>,
    error: Option<ApiError>,
}

impl FileUploadWebhookPayloadBuilder {
    pub fn event(mut self, value: FileUploadWebhookEventName) -> Self {
        self.event = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: f64) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn file(mut self, value: StorageFile) -> Self {
        self.file = Some(value);
        self
    }

    pub fn error(mut self, value: ApiError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileUploadWebhookPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](FileUploadWebhookPayloadBuilder::event)
    /// - [`file_id`](FileUploadWebhookPayloadBuilder::file_id)
    /// - [`occurred_at`](FileUploadWebhookPayloadBuilder::occurred_at)
    /// - [`file`](FileUploadWebhookPayloadBuilder::file)
    pub fn build(self) -> Result<FileUploadWebhookPayload, BuildError> {
        Ok(FileUploadWebhookPayload {
            event: self.event.ok_or_else(|| BuildError::missing_field("event"))?,
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            occurred_at: self.occurred_at.ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            error: self.error,
        })
    }
}
