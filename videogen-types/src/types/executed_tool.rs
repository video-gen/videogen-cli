pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecutedTool {
    /// Execution id matching the original request.
    #[serde(rename = "toolExecutionId")]
    #[serde(default)]
    pub tool_execution_id: String,
    pub status: ExecutedToolStatus,
    /// Tool name (e.g. `GENERATE_IMAGE`, `TEXT_TO_SPEECH`).
    #[serde(rename = "toolType")]
    #[serde(default)]
    pub tool_type: String,
    /// Completion progress for the current attempt (0-100). Always `100` when `status` is `succeeded`.
    #[serde(rename = "progressPercentage")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub progress_percentage: f64,
    /// Zero-based index of the current or most recent execution attempt.
    #[serde(rename = "attemptIndex")]
    #[serde(default)]
    pub attempt_index: i64,
    /// One entry per generated result. Present when `status` is `succeeded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ToolSuccessResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl ExecutedTool {
    pub fn builder() -> ExecutedToolBuilder {
        <ExecutedToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutedToolBuilder {
    tool_execution_id: Option<String>,
    status: Option<ExecutedToolStatus>,
    tool_type: Option<String>,
    progress_percentage: Option<f64>,
    attempt_index: Option<i64>,
    results: Option<Vec<ToolSuccessResult>>,
    error: Option<ApiError>,
}

impl ExecutedToolBuilder {
    pub fn tool_execution_id(mut self, value: impl Into<String>) -> Self {
        self.tool_execution_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ExecutedToolStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tool_type(mut self, value: impl Into<String>) -> Self {
        self.tool_type = Some(value.into());
        self
    }

    pub fn progress_percentage(mut self, value: f64) -> Self {
        self.progress_percentage = Some(value);
        self
    }

    pub fn attempt_index(mut self, value: i64) -> Self {
        self.attempt_index = Some(value);
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

    /// Consumes the builder and constructs a [`ExecutedTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_execution_id`](ExecutedToolBuilder::tool_execution_id)
    /// - [`status`](ExecutedToolBuilder::status)
    /// - [`tool_type`](ExecutedToolBuilder::tool_type)
    /// - [`progress_percentage`](ExecutedToolBuilder::progress_percentage)
    /// - [`attempt_index`](ExecutedToolBuilder::attempt_index)
    pub fn build(self) -> Result<ExecutedTool, BuildError> {
        Ok(ExecutedTool {
            tool_execution_id: self.tool_execution_id.ok_or_else(|| BuildError::missing_field("tool_execution_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            tool_type: self.tool_type.ok_or_else(|| BuildError::missing_field("tool_type"))?,
            progress_percentage: self.progress_percentage.ok_or_else(|| BuildError::missing_field("progress_percentage"))?,
            attempt_index: self.attempt_index.ok_or_else(|| BuildError::missing_field("attempt_index"))?,
            results: self.results,
            error: self.error,
        })
    }
}
