pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Lifecycle events emitted for workflow runs started via the developer API.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkflowRunWebhookEventName {
    WorkflowRunSucceeded,
    WorkflowRunFailed,
    WorkflowRunCancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkflowRunWebhookEventName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WorkflowRunSucceeded => serializer.serialize_str("workflow_run.succeeded"),
            Self::WorkflowRunFailed => serializer.serialize_str("workflow_run.failed"),
            Self::WorkflowRunCancelled => serializer.serialize_str("workflow_run.cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkflowRunWebhookEventName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "workflow_run.succeeded" => Ok(Self::WorkflowRunSucceeded),
            "workflow_run.failed" => Ok(Self::WorkflowRunFailed),
            "workflow_run.cancelled" => Ok(Self::WorkflowRunCancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkflowRunWebhookEventName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkflowRunSucceeded => write!(f, "workflow_run.succeeded"),
            Self::WorkflowRunFailed => write!(f, "workflow_run.failed"),
            Self::WorkflowRunCancelled => write!(f, "workflow_run.cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
