pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single scene in the storyboard. Becomes one section in the final video.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GenerateStoryboardScene {
    /// What to generate for this scene.
    #[serde(default)]
    pub prompt: String,
    /// Optional section name for this scene. Defaults to a numbered name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional per-scene generation override. Falls back to the request-level `defaultGeneration` when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<SceneGeneration>,
    /// Optional per-scene duration in seconds. Falls back to `defaultDurationSeconds` when omitted. For AI_VIDEO scenes this must be a whole number between 1 and 15.
    #[serde(rename = "durationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// Optional id of an ACTOR entity (e.g. `vg_enti_...`) to feature in this scene. The entity's reference images are added so the same character appears in the generated image or video.
    #[serde(rename = "actorEntityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_entity_id: Option<String>,
    /// Optional id of a VISUAL_STYLE entity (e.g. `vg_enti_...`) whose reference images guide the look of this scene.
    #[serde(rename = "visualStyleEntityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_style_entity_id: Option<String>,
    /// Optional voiceover script for this scene. When provided, text-to-speech audio is generated from this text, then combined with the actor entity's reference image to produce a talking-head avatar video. The TTS voice is sourced from the actor entity's voice configuration.
    #[serde(rename = "voiceoverScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voiceover_script: Option<String>,
}

impl GenerateStoryboardScene {
    pub fn builder() -> GenerateStoryboardSceneBuilder {
        <GenerateStoryboardSceneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateStoryboardSceneBuilder {
    prompt: Option<String>,
    title: Option<String>,
    generation: Option<SceneGeneration>,
    duration_seconds: Option<i64>,
    actor_entity_id: Option<String>,
    visual_style_entity_id: Option<String>,
    voiceover_script: Option<String>,
}

impl GenerateStoryboardSceneBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn generation(mut self, value: SceneGeneration) -> Self {
        self.generation = Some(value);
        self
    }

    pub fn duration_seconds(mut self, value: i64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn actor_entity_id(mut self, value: impl Into<String>) -> Self {
        self.actor_entity_id = Some(value.into());
        self
    }

    pub fn visual_style_entity_id(mut self, value: impl Into<String>) -> Self {
        self.visual_style_entity_id = Some(value.into());
        self
    }

    pub fn voiceover_script(mut self, value: impl Into<String>) -> Self {
        self.voiceover_script = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GenerateStoryboardScene`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](GenerateStoryboardSceneBuilder::prompt)
    pub fn build(self) -> Result<GenerateStoryboardScene, BuildError> {
        Ok(GenerateStoryboardScene {
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            title: self.title,
            generation: self.generation,
            duration_seconds: self.duration_seconds,
            actor_entity_id: self.actor_entity_id,
            visual_style_entity_id: self.visual_style_entity_id,
            voiceover_script: self.voiceover_script,
        })
    }
}
