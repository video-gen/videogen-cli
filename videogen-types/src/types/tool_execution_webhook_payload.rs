pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Delivered to your webhook endpoint when a tool execution reaches a terminal state. The shape mirrors the `ExecutedTool` response with the addition of `event` and `occurredAt`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolExecutionWebhookPayload {
    pub event: ToolExecutionWebhookEventName,
    /// Execution id matching the original request.
    #[serde(rename = "toolExecutionId")]
    #[serde(default)]
    pub tool_execution_id: String,
    /// Seconds since epoch (Unix timestamp) when the execution reached a terminal state.
    #[serde(rename = "occurredAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub occurred_at: f64,
    /// Tool name (e.g. `GENERATE_IMAGE`, `TEXT_TO_SPEECH`).
    #[serde(rename = "toolType")]
    #[serde(default)]
    pub tool_type: String,
    /// One entry per generated result, each with a hydrated `file`. Present only on `tool_execution.succeeded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ToolSuccessResult>>,
    /// Present only on `tool_execution.failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl ToolExecutionWebhookPayload {
    pub fn builder() -> ToolExecutionWebhookPayloadBuilder {
        <ToolExecutionWebhookPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolExecutionWebhookPayloadBuilder {
    event: Option<ToolExecutionWebhookEventName>,
    tool_execution_id: Option<String>,
    occurred_at: Option<f64>,
    tool_type: Option<String>,
    results: Option<Vec<ToolSuccessResult>>,
    error: Option<ApiError>,
}

impl ToolExecutionWebhookPayloadBuilder {
    pub fn event(mut self, value: ToolExecutionWebhookEventName) -> Self {
        self.event = Some(value);
        self
    }

    pub fn tool_execution_id(mut self, value: impl Into<String>) -> Self {
        self.tool_execution_id = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: f64) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn tool_type(mut self, value: impl Into<String>) -> Self {
        self.tool_type = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<ToolSuccessResult>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn error(mut self, value: ApiError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolExecutionWebhookPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](ToolExecutionWebhookPayloadBuilder::event)
    /// - [`tool_execution_id`](ToolExecutionWebhookPayloadBuilder::tool_execution_id)
    /// - [`occurred_at`](ToolExecutionWebhookPayloadBuilder::occurred_at)
    /// - [`tool_type`](ToolExecutionWebhookPayloadBuilder::tool_type)
    pub fn build(self) -> Result<ToolExecutionWebhookPayload, BuildError> {
        Ok(ToolExecutionWebhookPayload {
            event: self.event.ok_or_else(|| BuildError::missing_field("event"))?,
            tool_execution_id: self.tool_execution_id.ok_or_else(|| BuildError::missing_field("tool_execution_id"))?,
            occurred_at: self.occurred_at.ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            tool_type: self.tool_type.ok_or_else(|| BuildError::missing_field("tool_type"))?,
            results: self.results,
            error: self.error,
        })
    }
}
