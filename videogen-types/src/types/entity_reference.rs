pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A file attached to an entity as a reference image.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityReference {
    /// The reference image file id (e.g. `vg_file_...`). Hydrate it via `GET /v1/files/{fileId}` to fetch a viewable URL.
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    /// Optional description of the reference. Empty string when not set.
    #[serde(default)]
    pub description: String,
    /// When true, this is the entity's primary reference (used for its thumbnail).
    #[serde(rename = "isDefault")]
    #[serde(default)]
    pub is_default: bool,
}

impl EntityReference {
    pub fn builder() -> EntityReferenceBuilder {
        <EntityReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityReferenceBuilder {
    file_id: Option<String>,
    description: Option<String>,
    is_default: Option<bool>,
}

impl EntityReferenceBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](EntityReferenceBuilder::file_id)
    /// - [`description`](EntityReferenceBuilder::description)
    /// - [`is_default`](EntityReferenceBuilder::is_default)
    pub fn build(self) -> Result<EntityReference, BuildError> {
        Ok(EntityReference {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            is_default: self.is_default.ok_or_else(|| BuildError::missing_field("is_default"))?,
        })
    }
}
