pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// All webhook event types. Use when creating or listing webhook endpoints.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookEventName {
    ToolExecutionSucceeded,
    ToolExecutionFailed,
    ToolExecutionCancelled,
    WorkflowRunSucceeded,
    WorkflowRunFailed,
    WorkflowRunCancelled,
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
impl Serialize for WebhookEventName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ToolExecutionSucceeded => serializer.serialize_str("tool_execution.succeeded"),
            Self::ToolExecutionFailed => serializer.serialize_str("tool_execution.failed"),
            Self::ToolExecutionCancelled => serializer.serialize_str("tool_execution.cancelled"),
            Self::WorkflowRunSucceeded => serializer.serialize_str("workflow_run.succeeded"),
            Self::WorkflowRunFailed => serializer.serialize_str("workflow_run.failed"),
            Self::WorkflowRunCancelled => serializer.serialize_str("workflow_run.cancelled"),
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

impl<'de> Deserialize<'de> for WebhookEventName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "tool_execution.succeeded" => Ok(Self::ToolExecutionSucceeded),
            "tool_execution.failed" => Ok(Self::ToolExecutionFailed),
            "tool_execution.cancelled" => Ok(Self::ToolExecutionCancelled),
            "workflow_run.succeeded" => Ok(Self::WorkflowRunSucceeded),
            "workflow_run.failed" => Ok(Self::WorkflowRunFailed),
            "workflow_run.cancelled" => Ok(Self::WorkflowRunCancelled),
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

impl fmt::Display for WebhookEventName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolExecutionSucceeded => write!(f, "tool_execution.succeeded"),
            Self::ToolExecutionFailed => write!(f, "tool_execution.failed"),
            Self::ToolExecutionCancelled => write!(f, "tool_execution.cancelled"),
            Self::WorkflowRunSucceeded => write!(f, "workflow_run.succeeded"),
            Self::WorkflowRunFailed => write!(f, "workflow_run.failed"),
            Self::WorkflowRunCancelled => write!(f, "workflow_run.cancelled"),
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
