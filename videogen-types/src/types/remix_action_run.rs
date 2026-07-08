pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemixActionRun {
    /// Opaque remix action id (e.g. `vg_rmix_...`).
    #[serde(rename = "remixActionId")]
    #[serde(default)]
    pub remix_action_id: String,
    pub r#type: RemixActionType,
    pub status: RemixActionStatus,
    /// Id of the project this remix action edits (e.g. `vg_proj_...`).
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    /// URL to view the project in the VideoGen app.
    #[serde(rename = "projectUrl")]
    #[serde(default)]
    pub project_url: String,
    /// Completion progress for the current attempt (0-100). Always `100` when `status` is `succeeded`.
    #[serde(rename = "progressPercentage")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub progress_percentage: f64,
    /// Zero-based index of the current or most recent execution attempt.
    #[serde(rename = "attemptIndex")]
    #[serde(default)]
    pub attempt_index: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl RemixActionRun {
    pub fn builder() -> RemixActionRunBuilder {
        <RemixActionRunBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemixActionRunBuilder {
    remix_action_id: Option<String>,
    r#type: Option<RemixActionType>,
    status: Option<RemixActionStatus>,
    project_id: Option<String>,
    project_url: Option<String>,
    progress_percentage: Option<f64>,
    attempt_index: Option<i64>,
    error: Option<ApiError>,
}

impl RemixActionRunBuilder {
    pub fn remix_action_id(mut self, value: impl Into<String>) -> Self {
        self.remix_action_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: RemixActionType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: RemixActionStatus) -> Self {
        self.status = Some(value);
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

    pub fn progress_percentage(mut self, value: f64) -> Self {
        self.progress_percentage = Some(value);
        self
    }

    pub fn attempt_index(mut self, value: i64) -> Self {
        self.attempt_index = Some(value);
        self
    }

    pub fn error(mut self, value: ApiError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemixActionRun`].
    /// This method will fail if any of the following fields are not set:
    /// - [`remix_action_id`](RemixActionRunBuilder::remix_action_id)
    /// - [`r#type`](RemixActionRunBuilder::r#type)
    /// - [`status`](RemixActionRunBuilder::status)
    /// - [`project_id`](RemixActionRunBuilder::project_id)
    /// - [`project_url`](RemixActionRunBuilder::project_url)
    /// - [`progress_percentage`](RemixActionRunBuilder::progress_percentage)
    /// - [`attempt_index`](RemixActionRunBuilder::attempt_index)
    pub fn build(self) -> Result<RemixActionRun, BuildError> {
        Ok(RemixActionRun {
            remix_action_id: self.remix_action_id.ok_or_else(|| BuildError::missing_field("remix_action_id"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            project_url: self.project_url.ok_or_else(|| BuildError::missing_field("project_url"))?,
            progress_percentage: self.progress_percentage.ok_or_else(|| BuildError::missing_field("progress_percentage"))?,
            attempt_index: self.attempt_index.ok_or_else(|| BuildError::missing_field("attempt_index"))?,
            error: self.error,
        })
    }
}
