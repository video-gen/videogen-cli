pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result for a single generated file. Present when `status` is `succeeded`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolSuccessResult {
    /// File id for the generated asset.
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    /// File type.
    pub r#type: ToolSuccessResultType,
    /// Hydrated file metadata with signed download URLs. Populated when returned from a webhook or after hydration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<StorageFile>,
}

impl ToolSuccessResult {
    pub fn builder() -> ToolSuccessResultBuilder {
        <ToolSuccessResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolSuccessResultBuilder {
    file_id: Option<String>,
    r#type: Option<ToolSuccessResultType>,
    file: Option<StorageFile>,
}

impl ToolSuccessResultBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ToolSuccessResultType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn file(mut self, value: StorageFile) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolSuccessResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](ToolSuccessResultBuilder::file_id)
    /// - [`r#type`](ToolSuccessResultBuilder::r#type)
    pub fn build(self) -> Result<ToolSuccessResult, BuildError> {
        Ok(ToolSuccessResult {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            file: self.file,
        })
    }
}
