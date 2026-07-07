pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Paginated list of API-started workflow runs, most recently created first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowRunListResponse {
    #[serde(rename = "workflowRuns")]
    #[serde(default)]
    pub workflow_runs: Vec<WorkflowRun>,
    /// When true, there are more runs available. Pass `nextCursor` as the `cursor` query param to fetch the next page.
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    /// Opaque cursor to fetch the next page. `null` when `hasMore` is false.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl WorkflowRunListResponse {
    pub fn builder() -> WorkflowRunListResponseBuilder {
        <WorkflowRunListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowRunListResponseBuilder {
    workflow_runs: Option<Vec<WorkflowRun>>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl WorkflowRunListResponseBuilder {
    pub fn workflow_runs(mut self, value: Vec<WorkflowRun>) -> Self {
        self.workflow_runs = Some(value);
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

    /// Consumes the builder and constructs a [`WorkflowRunListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_runs`](WorkflowRunListResponseBuilder::workflow_runs)
    /// - [`has_more`](WorkflowRunListResponseBuilder::has_more)
    pub fn build(self) -> Result<WorkflowRunListResponse, BuildError> {
        Ok(WorkflowRunListResponse {
            workflow_runs: self.workflow_runs.ok_or_else(|| BuildError::missing_field("workflow_runs"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
        })
    }
}
