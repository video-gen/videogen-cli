pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportProjectResponse {
    /// Opaque export id (e.g. `vg_expo_...`). Poll `GET /v1/projects/{projectId}/exports/{exportId}` or subscribe to webhooks for completion.
    #[serde(rename = "exportId")]
    #[serde(default)]
    pub export_id: String,
}

impl ExportProjectResponse {
    pub fn builder() -> ExportProjectResponseBuilder {
        <ExportProjectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportProjectResponseBuilder {
    export_id: Option<String>,
}

impl ExportProjectResponseBuilder {
    pub fn export_id(mut self, value: impl Into<String>) -> Self {
        self.export_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExportProjectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`export_id`](ExportProjectResponseBuilder::export_id)
    pub fn build(self) -> Result<ExportProjectResponse, BuildError> {
        Ok(ExportProjectResponse {
            export_id: self.export_id.ok_or_else(|| BuildError::missing_field("export_id"))?,
        })
    }
}
