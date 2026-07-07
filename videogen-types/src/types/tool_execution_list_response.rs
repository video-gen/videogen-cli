pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Paginated list of API-started tool executions, most recently created first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolExecutionListResponse {
    #[serde(rename = "toolExecutions")]
    #[serde(default)]
    pub tool_executions: Vec<ExecutedTool>,
    /// When true, there are more executions available. Pass `nextCursor` as the `cursor` query param to fetch the next page.
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    /// Opaque cursor to fetch the next page. `null` when `hasMore` is false.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl ToolExecutionListResponse {
    pub fn builder() -> ToolExecutionListResponseBuilder {
        <ToolExecutionListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolExecutionListResponseBuilder {
    tool_executions: Option<Vec<ExecutedTool>>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl ToolExecutionListResponseBuilder {
    pub fn tool_executions(mut self, value: Vec<ExecutedTool>) -> Self {
        self.tool_executions = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolExecutionListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_executions`](ToolExecutionListResponseBuilder::tool_executions)
    /// - [`has_more`](ToolExecutionListResponseBuilder::has_more)
    pub fn build(self) -> Result<ToolExecutionListResponse, BuildError> {
        Ok(ToolExecutionListResponse {
            tool_executions: self.tool_executions.ok_or_else(|| BuildError::missing_field("tool_executions"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
        })
    }
}
