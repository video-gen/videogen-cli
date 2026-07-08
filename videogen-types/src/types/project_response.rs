pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Simplified project metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProjectResponse {
    /// Opaque project id (e.g. `vg_proj_...`).
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub title: String,
    #[serde(rename = "aspectRatio")]
    #[serde(default)]
    pub aspect_ratio: AspectRatio,
    /// High-level project status.
    pub status: ProjectResponseStatus,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(rename = "projectUrl")]
    #[serde(default)]
    pub project_url: String,
}

impl ProjectResponse {
    pub fn builder() -> ProjectResponseBuilder {
        <ProjectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectResponseBuilder {
    project_id: Option<String>,
    title: Option<String>,
    aspect_ratio: Option<AspectRatio>,
    status: Option<ProjectResponseStatus>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    project_url: Option<String>,
}

impl ProjectResponseBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn aspect_ratio(mut self, value: AspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn status(mut self, value: ProjectResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn project_url(mut self, value: impl Into<String>) -> Self {
        self.project_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProjectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](ProjectResponseBuilder::project_id)
    /// - [`title`](ProjectResponseBuilder::title)
    /// - [`aspect_ratio`](ProjectResponseBuilder::aspect_ratio)
    /// - [`status`](ProjectResponseBuilder::status)
    /// - [`created_at`](ProjectResponseBuilder::created_at)
    /// - [`updated_at`](ProjectResponseBuilder::updated_at)
    /// - [`project_url`](ProjectResponseBuilder::project_url)
    pub fn build(self) -> Result<ProjectResponse, BuildError> {
        Ok(ProjectResponse {
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            aspect_ratio: self.aspect_ratio.ok_or_else(|| BuildError::missing_field("aspect_ratio"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            project_url: self.project_url.ok_or_else(|| BuildError::missing_field("project_url"))?,
        })
    }
}
