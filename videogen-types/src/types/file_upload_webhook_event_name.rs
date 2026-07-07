pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Webhook event types for file upload lifecycle. Only fired for files uploaded via the API (not the VideoGen UI).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileUploadWebhookEventName {
    FileUploadCompleted,
    FileUploadFailed,
    FilePlaybackReady,
    FileDownloadReady,
    FileAnalysisCompleted,
    FileAnalysisFailed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FileUploadWebhookEventName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FileUploadCompleted => serializer.serialize_str("file.upload.completed"),
            Self::FileUploadFailed => serializer.serialize_str("file.upload.failed"),
            Self::FilePlaybackReady => serializer.serialize_str("file.playback_ready"),
            Self::FileDownloadReady => serializer.serialize_str("file.download_ready"),
            Self::FileAnalysisCompleted => serializer.serialize_str("file.analysis_completed"),
            Self::FileAnalysisFailed => serializer.serialize_str("file.analysis_failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FileUploadWebhookEventName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "file.upload.completed" => Ok(Self::FileUploadCompleted),
            "file.upload.failed" => Ok(Self::FileUploadFailed),
            "file.playback_ready" => Ok(Self::FilePlaybackReady),
            "file.download_ready" => Ok(Self::FileDownloadReady),
            "file.analysis_completed" => Ok(Self::FileAnalysisCompleted),
            "file.analysis_failed" => Ok(Self::FileAnalysisFailed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FileUploadWebhookEventName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileUploadCompleted => write!(f, "file.upload.completed"),
            Self::FileUploadFailed => write!(f, "file.upload.failed"),
            Self::FilePlaybackReady => write!(f, "file.playback_ready"),
            Self::FileDownloadReady => write!(f, "file.download_ready"),
            Self::FileAnalysisCompleted => write!(f, "file.analysis_completed"),
            Self::FileAnalysisFailed => write!(f, "file.analysis_failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
