pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateEntityRequest {
    /// ACTOR features a consistent character; PRODUCT features a consistent product or object; VISUAL_STYLE guides the look of generated images.
    #[serde(rename = "entityType")]
    pub entity_type: CreateEntityRequestEntityType,
    /// Display name.
    #[serde(default)]
    pub name: String,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateEntityRequest {
    pub fn builder() -> CreateEntityRequestBuilder {
        <CreateEntityRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEntityRequestBuilder {
    entity_type: Option<CreateEntityRequestEntityType>,
    name: Option<String>,
    description: Option<String>,
}

impl CreateEntityRequestBuilder {
    pub fn entity_type(mut self, value: CreateEntityRequestEntityType) -> Self {
        self.entity_type = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateEntityRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity_type`](CreateEntityRequestBuilder::entity_type)
    /// - [`name`](CreateEntityRequestBuilder::name)
    pub fn build(self) -> Result<CreateEntityRequest, BuildError> {
        Ok(CreateEntityRequest {
            entity_type: self.entity_type.ok_or_else(|| BuildError::missing_field("entity_type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}

