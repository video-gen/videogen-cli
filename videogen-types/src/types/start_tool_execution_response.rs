pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Returned when a tool execution is started. Use `toolExecutionId` to poll for results or cancel.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StartToolExecutionResponse {
    /// Execution id (e.g. `vg_tool_...`).
    #[serde(rename = "toolExecutionId")]
    #[serde(default)]
    pub tool_execution_id: String,
}

impl StartToolExecutionResponse {
    pub fn builder() -> StartToolExecutionResponseBuilder {
        <StartToolExecutionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartToolExecutionResponseBuilder {
    tool_execution_id: Option<String>,
}

impl StartToolExecutionResponseBuilder {
    pub fn tool_execution_id(mut self, value: impl Into<String>) -> Self {
        self.tool_execution_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StartToolExecutionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_execution_id`](StartToolExecutionResponseBuilder::tool_execution_id)
    pub fn build(self) -> Result<StartToolExecutionResponse, BuildError> {
        Ok(StartToolExecutionResponse {
            tool_execution_id: self.tool_execution_id.ok_or_else(|| BuildError::missing_field("tool_execution_id"))?,
        })
    }
}
