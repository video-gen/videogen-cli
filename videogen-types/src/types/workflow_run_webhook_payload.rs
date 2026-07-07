pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Body POSTed to a registered webhook endpoint when a workflow run reaches a terminal state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowRunWebhookPayload {
    pub event: WorkflowRunWebhookEventName,
    /// Opaque workflow run id matching the original request.
    #[serde(rename = "workflowRunId")]
    #[serde(default)]
    pub workflow_run_id: String,
    /// ISO-8601 timestamp at which VideoGen observed the terminal state.
    #[serde(rename = "occurredAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub occurred_at: DateTime<FixedOffset>,
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
    /// Id of the project created for this workflow run.
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    /// URL to view the project in the VideoGen app.
    #[serde(rename = "projectUrl")]
    #[serde(default)]
    pub project_url: String,
    /// Present only on `workflow_run.failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl WorkflowRunWebhookPayload {
    pub fn builder() -> WorkflowRunWebhookPayloadBuilder {
        <WorkflowRunWebhookPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowRunWebhookPayloadBuilder {
    event: Option<WorkflowRunWebhookEventName>,
    workflow_run_id: Option<String>,
    occurred_at: Option<DateTime<FixedOffset>>,
    workflow_type: Option<WorkflowType>,
    project_id: Option<String>,
    project_url: Option<String>,
    error: Option<ApiError>,
}

impl WorkflowRunWebhookPayloadBuilder {
    pub fn event(mut self, value: WorkflowRunWebhookEventName) -> Self {
        self.event = Some(value);
        self
    }

    pub fn workflow_run_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_run_id = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn workflow_type(mut self, value: WorkflowType) -> Self {
        self.workflow_type = Some(value);
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

    /// Consumes the builder and constructs a [`WorkflowRunWebhookPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](WorkflowRunWebhookPayloadBuilder::event)
    /// - [`workflow_run_id`](WorkflowRunWebhookPayloadBuilder::workflow_run_id)
    /// - [`occurred_at`](WorkflowRunWebhookPayloadBuilder::occurred_at)
    /// - [`workflow_type`](WorkflowRunWebhookPayloadBuilder::workflow_type)
    /// - [`project_id`](WorkflowRunWebhookPayloadBuilder::project_id)
    /// - [`project_url`](WorkflowRunWebhookPayloadBuilder::project_url)
    pub fn build(self) -> Result<WorkflowRunWebhookPayload, BuildError> {
        Ok(WorkflowRunWebhookPayload {
            event: self.event.ok_or_else(|| BuildError::missing_field("event"))?,
            workflow_run_id: self.workflow_run_id.ok_or_else(|| BuildError::missing_field("workflow_run_id"))?,
            occurred_at: self.occurred_at.ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            workflow_type: self.workflow_type.ok_or_else(|| BuildError::missing_field("workflow_type"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            project_url: self.project_url.ok_or_else(|| BuildError::missing_field("project_url"))?,
            error: self.error,
        })
    }
}
