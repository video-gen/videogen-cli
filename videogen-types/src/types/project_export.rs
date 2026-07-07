pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectExport {
    /// Opaque export id matching the original request.
    #[serde(rename = "exportId")]
    #[serde(default)]
    pub export_id: String,
    /// Id of the exported project.
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    pub status: ProjectExportStatus,
    /// Completion progress for the current attempt (0-100). Always `100` when `status` is `succeeded`.
    #[serde(rename = "progressPercentage")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub progress_percentage: f64,
    /// Zero-based index of the current or most recent export attempt.
    #[serde(rename = "attemptIndex")]
    #[serde(default)]
    pub attempt_index: i64,
    /// Signed MP4 download URL. Present when `status` is `succeeded`.
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// Signed thumbnail URL. Present when `status` is `succeeded` and a thumbnail is available.
    #[serde(rename = "thumbnailUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl ProjectExport {
    pub fn builder() -> ProjectExportBuilder {
        <ProjectExportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectExportBuilder {
    export_id: Option<String>,
    project_id: Option<String>,
    status: Option<ProjectExportStatus>,
    progress_percentage: Option<f64>,
    attempt_index: Option<i64>,
    download_url: Option<String>,
    thumbnail_url: Option<String>,
    error: Option<ApiError>,
}

impl ProjectExportBuilder {
    pub fn export_id(mut self, value: impl Into<String>) -> Self {
        self.export_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ProjectExportStatus) -> Self {
        self.status = Some(value);
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

    pub fn download_url(mut self, value: impl Into<String>) -> Self {
        self.download_url = Some(value.into());
        self
    }

    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }

    pub fn error(mut self, value: ApiError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectExport`].
    /// This method will fail if any of the following fields are not set:
    /// - [`export_id`](ProjectExportBuilder::export_id)
    /// - [`project_id`](ProjectExportBuilder::project_id)
    /// - [`status`](ProjectExportBuilder::status)
    /// - [`progress_percentage`](ProjectExportBuilder::progress_percentage)
    /// - [`attempt_index`](ProjectExportBuilder::attempt_index)
    pub fn build(self) -> Result<ProjectExport, BuildError> {
        Ok(ProjectExport {
            export_id: self.export_id.ok_or_else(|| BuildError::missing_field("export_id"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            progress_percentage: self.progress_percentage.ok_or_else(|| BuildError::missing_field("progress_percentage"))?,
            attempt_index: self.attempt_index.ok_or_else(|| BuildError::missing_field("attempt_index"))?,
            download_url: self.download_url,
            thumbnail_url: self.thumbnail_url,
            error: self.error,
        })
    }
}
