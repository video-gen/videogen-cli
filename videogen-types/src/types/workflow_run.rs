pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowRun {
    /// Opaque workflow run id.
    #[serde(rename = "workflowRunId")]
    #[serde(default)]
    pub workflow_run_id: String,
    pub status: WorkflowRunStatus,
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
    /// Completion progress for the current attempt (0-100). Always `100` when `status` is `succeeded`.
    #[serde(rename = "progressPercentage")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub progress_percentage: f64,
    /// Zero-based index of the current or most recent execution attempt.
    #[serde(rename = "attemptIndex")]
    #[serde(default)]
    pub attempt_index: i64,
    /// Id of the project created for this workflow run (e.g. `vg_proj_...`).
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    /// URL to view the project in the VideoGen app.
    #[serde(rename = "projectUrl")]
    #[serde(default)]
    pub project_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl WorkflowRun {
    pub fn builder() -> WorkflowRunBuilder {
        <WorkflowRunBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowRunBuilder {
    workflow_run_id: Option<String>,
    status: Option<WorkflowRunStatus>,
    workflow_type: Option<WorkflowType>,
    progress_percentage: Option<f64>,
    attempt_index: Option<i64>,
    project_id: Option<String>,
    project_url: Option<String>,
    error: Option<ApiError>,
}

impl WorkflowRunBuilder {
    pub fn workflow_run_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: WorkflowRunStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn workflow_type(mut self, value: WorkflowType) -> Self {
        self.workflow_type = Some(value);
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

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn project_url(mut self, value: impl Into<String>) -> Self {
        self.project_url = Some(value.into());
        self
    }

    pub fn error(mut self, value: ApiError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowRun`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_run_id`](WorkflowRunBuilder::workflow_run_id)
    /// - [`status`](WorkflowRunBuilder::status)
    /// - [`workflow_type`](WorkflowRunBuilder::workflow_type)
    /// - [`progress_percentage`](WorkflowRunBuilder::progress_percentage)
    /// - [`attempt_index`](WorkflowRunBuilder::attempt_index)
    /// - [`project_id`](WorkflowRunBuilder::project_id)
    /// - [`project_url`](WorkflowRunBuilder::project_url)
    pub fn build(self) -> Result<WorkflowRun, BuildError> {
        Ok(WorkflowRun {
            workflow_run_id: self.workflow_run_id.ok_or_else(|| BuildError::missing_field("workflow_run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            workflow_type: self.workflow_type.ok_or_else(|| BuildError::missing_field("workflow_type"))?,
            progress_percentage: self.progress_percentage.ok_or_else(|| BuildError::missing_field("progress_percentage"))?,
            attempt_index: self.attempt_index.ok_or_else(|| BuildError::missing_field("attempt_index"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            project_url: self.project_url.ok_or_else(|| BuildError::missing_field("project_url"))?,
            error: self.error,
        })
    }
}
