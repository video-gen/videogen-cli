pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityArchiveResponse {
    /// The id of the archived entity.
    #[serde(rename = "entityId")]
    #[serde(default)]
    pub entity_id: String,
    /// Always true on success.
    #[serde(default)]
    pub archived: bool,
}

impl EntityArchiveResponse {
    pub fn builder() -> EntityArchiveResponseBuilder {
        <EntityArchiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityArchiveResponseBuilder {
    entity_id: Option<String>,
    archived: Option<bool>,
}

impl EntityArchiveResponseBuilder {
    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityArchiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity_id`](EntityArchiveResponseBuilder::entity_id)
    /// - [`archived`](EntityArchiveResponseBuilder::archived)
    pub fn build(self) -> Result<EntityArchiveResponse, BuildError> {
        Ok(EntityArchiveResponse {
            entity_id: self.entity_id.ok_or_else(|| BuildError::missing_field("entity_id"))?,
            archived: self.archived.ok_or_else(|| BuildError::missing_field("archived"))?,
        })
    }
}
