pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Webhook event types for tool execution lifecycle.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolExecutionWebhookEventName {
    ToolExecutionSucceeded,
    ToolExecutionFailed,
    ToolExecutionCancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolExecutionWebhookEventName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ToolExecutionSucceeded => serializer.serialize_str("tool_execution.succeeded"),
            Self::ToolExecutionFailed => serializer.serialize_str("tool_execution.failed"),
            Self::ToolExecutionCancelled => serializer.serialize_str("tool_execution.cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolExecutionWebhookEventName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "tool_execution.succeeded" => Ok(Self::ToolExecutionSucceeded),
            "tool_execution.failed" => Ok(Self::ToolExecutionFailed),
            "tool_execution.cancelled" => Ok(Self::ToolExecutionCancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolExecutionWebhookEventName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolExecutionSucceeded => write!(f, "tool_execution.succeeded"),
            Self::ToolExecutionFailed => write!(f, "tool_execution.failed"),
            Self::ToolExecutionCancelled => write!(f, "tool_execution.cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
