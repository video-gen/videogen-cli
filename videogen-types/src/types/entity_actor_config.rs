pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Read-only voice and presenter summary for an ACTOR entity. Always null for VISUAL_STYLE entities.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityActorConfig {
    /// Display name of the actor's voice when one is configured. Null otherwise.
    #[serde(rename = "voiceDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_display_name: Option<String>,
    /// True when the actor has a configured voice.
    #[serde(rename = "hasVoice")]
    #[serde(default)]
    pub has_voice: bool,
    /// True when the actor has a configured avatar presenter.
    #[serde(rename = "hasAvatarPresenter")]
    #[serde(default)]
    pub has_avatar_presenter: bool,
}

impl EntityActorConfig {
    pub fn builder() -> EntityActorConfigBuilder {
        <EntityActorConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityActorConfigBuilder {
    voice_display_name: Option<String>,
    has_voice: Option<bool>,
    has_avatar_presenter: Option<bool>,
}

impl EntityActorConfigBuilder {
    pub fn voice_display_name(mut self, value: impl Into<String>) -> Self {
        self.voice_display_name = Some(value.into());
        self
    }

    pub fn has_voice(mut self, value: bool) -> Self {
        self.has_voice = Some(value);
        self
    }

    pub fn has_avatar_presenter(mut self, value: bool) -> Self {
        self.has_avatar_presenter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityActorConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_voice`](EntityActorConfigBuilder::has_voice)
    /// - [`has_avatar_presenter`](EntityActorConfigBuilder::has_avatar_presenter)
    pub fn build(self) -> Result<EntityActorConfig, BuildError> {
        Ok(EntityActorConfig {
            voice_display_name: self.voice_display_name,
            has_voice: self.has_voice.ok_or_else(|| BuildError::missing_field("has_voice"))?,
            has_avatar_presenter: self.has_avatar_presenter.ok_or_else(|| BuildError::missing_field("has_avatar_presenter"))?,
        })
    }
}
