pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Returned when a workflow run is accepted. Poll `GET /v1/workflows/runs/{workflowRunId}` or subscribe to webhooks for completion.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StartWorkflowRunResponse {
    /// Opaque workflow run id (e.g. `vg_work_...`).
    #[serde(rename = "workflowRunId")]
    #[serde(default)]
    pub workflow_run_id: String,
    /// Id of the project created for this workflow run (e.g. `vg_proj_...`).
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    /// URL to view the project in the VideoGen app.
    #[serde(rename = "projectUrl")]
    #[serde(default)]
    pub project_url: String,
    /// Opaque remix action ids (e.g. `vg_rmix_...`), one per `remixActions` entry in request order. Empty when no remix actions were requested. Each runs after the video is built; poll `GET /v1/projects/{projectId}/remix-actions`.
    #[serde(rename = "remixActionIds")]
    #[serde(default)]
    pub remix_action_ids: Vec<String>,
}

impl StartWorkflowRunResponse {
    pub fn builder() -> StartWorkflowRunResponseBuilder {
        <StartWorkflowRunResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartWorkflowRunResponseBuilder {
    workflow_run_id: Option<String>,
    project_id: Option<String>,
    project_url: Option<String>,
    remix_action_ids: Option<Vec<String>>,
}

impl StartWorkflowRunResponseBuilder {
    pub fn workflow_run_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_run_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn project_url(mut self, value: impl Into<String>) -> Self {
        self.project_url = Some(value.into());
        self
    }

    pub fn remix_action_ids(mut self, value: Vec<String>) -> Self {
        self.remix_action_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StartWorkflowRunResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_run_id`](StartWorkflowRunResponseBuilder::workflow_run_id)
    /// - [`project_id`](StartWorkflowRunResponseBuilder::project_id)
    /// - [`project_url`](StartWorkflowRunResponseBuilder::project_url)
    /// - [`remix_action_ids`](StartWorkflowRunResponseBuilder::remix_action_ids)
    pub fn build(self) -> Result<StartWorkflowRunResponse, BuildError> {
        Ok(StartWorkflowRunResponse {
            workflow_run_id: self.workflow_run_id.ok_or_else(|| BuildError::missing_field("workflow_run_id"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            project_url: self.project_url.ok_or_else(|| BuildError::missing_field("project_url"))?,
            remix_action_ids: self.remix_action_ids.ok_or_else(|| BuildError::missing_field("remix_action_ids"))?,
        })
    }
}
