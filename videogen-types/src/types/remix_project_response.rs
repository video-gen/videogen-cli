pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Returned when remix actions are accepted. Poll `GET /v1/projects/{projectId}/remix-actions` for status.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemixProjectResponse {
    /// Id of the edited project (e.g. `vg_proj_...`; the duplicate when `saveAsNewProject` was true).
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    /// URL to view the project in the VideoGen app.
    #[serde(rename = "projectUrl")]
    #[serde(default)]
    pub project_url: String,
    /// Opaque remix action ids (e.g. `vg_rmix_...`), one per requested action in order.
    #[serde(rename = "remixActionIds")]
    #[serde(default)]
    pub remix_action_ids: Vec<String>,
}

impl RemixProjectResponse {
    pub fn builder() -> RemixProjectResponseBuilder {
        <RemixProjectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemixProjectResponseBuilder {
    project_id: Option<String>,
    project_url: Option<String>,
    remix_action_ids: Option<Vec<String>>,
}

impl RemixProjectResponseBuilder {
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

    /// Consumes the builder and constructs a [`RemixProjectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](RemixProjectResponseBuilder::project_id)
    /// - [`project_url`](RemixProjectResponseBuilder::project_url)
    /// - [`remix_action_ids`](RemixProjectResponseBuilder::remix_action_ids)
    pub fn build(self) -> Result<RemixProjectResponse, BuildError> {
        Ok(RemixProjectResponse {
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            project_url: self.project_url.ok_or_else(|| BuildError::missing_field("project_url"))?,
            remix_action_ids: self.remix_action_ids.ok_or_else(|| BuildError::missing_field("remix_action_ids"))?,
        })
    }
}
