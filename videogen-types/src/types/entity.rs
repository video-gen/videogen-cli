pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A reusable actor or visual style. Attach its reference images to workflows for consistent characters and looks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Entity {
    /// The entity id (e.g. `vg_enti_...`).
    #[serde(rename = "entityId")]
    #[serde(default)]
    pub entity_id: String,
    /// ACTOR features a consistent character; PRODUCT features a consistent product or object; VISUAL_STYLE guides the look of generated images.
    #[serde(rename = "entityType")]
    pub entity_type: EntityEntityType,
    /// Display name.
    #[serde(default)]
    pub name: String,
    /// Optional description. Empty string when not set.
    #[serde(default)]
    pub description: String,
    /// Voice and presenter summary for ACTOR entities. Null for VISUAL_STYLE entities.
    #[serde(rename = "actorConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_config: Option<EntityActorConfig>,
    /// Reference images attached to the entity.
    #[serde(default)]
    pub references: Vec<EntityReference>,
    /// When the entity was created (ISO 8601).
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When the entity was last updated (ISO 8601).
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl Entity {
    pub fn builder() -> EntityBuilder {
        <EntityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityBuilder {
    entity_id: Option<String>,
    entity_type: Option<EntityEntityType>,
    name: Option<String>,
    description: Option<String>,
    actor_config: Option<EntityActorConfig>,
    references: Option<Vec<EntityReference>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl EntityBuilder {
    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    pub fn entity_type(mut self, value: EntityEntityType) -> Self {
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

    pub fn actor_config(mut self, value: EntityActorConfig) -> Self {
        self.actor_config = Some(value);
        self
    }

    pub fn references(mut self, value: Vec<EntityReference>) -> Self {
        self.references = Some(value);
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

    /// Consumes the builder and constructs a [`Entity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity_id`](EntityBuilder::entity_id)
    /// - [`entity_type`](EntityBuilder::entity_type)
    /// - [`name`](EntityBuilder::name)
    /// - [`description`](EntityBuilder::description)
    /// - [`references`](EntityBuilder::references)
    /// - [`created_at`](EntityBuilder::created_at)
    /// - [`updated_at`](EntityBuilder::updated_at)
    pub fn build(self) -> Result<Entity, BuildError> {
        Ok(Entity {
            entity_id: self.entity_id.ok_or_else(|| BuildError::missing_field("entity_id"))?,
            entity_type: self.entity_type.ok_or_else(|| BuildError::missing_field("entity_type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            actor_config: self.actor_config,
            references: self.references.ok_or_else(|| BuildError::missing_field("references"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
