pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How a scene is generated. Tagged by `type`: AI_IMAGE produces a still image from the prompt; AI_VIDEO produces a video clip from the prompt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SceneGeneration {
    /// AI_IMAGE generates a still image. AI_VIDEO generates a video clip.
    pub r#type: SceneGenerationType,
    /// A free-form description of the look, appended to the scene prompt (e.g. `loose watercolor illustration with visible brushstrokes`). See the AI styles reference for example descriptions of the app's default styles. No extra style is applied when omitted.
    #[serde(rename = "aiStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_style: Option<String>,
}

impl SceneGeneration {
    pub fn builder() -> SceneGenerationBuilder {
        <SceneGenerationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SceneGenerationBuilder {
    r#type: Option<SceneGenerationType>,
    ai_style: Option<String>,
}

impl SceneGenerationBuilder {
    pub fn r#type(mut self, value: SceneGenerationType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn ai_style(mut self, value: impl Into<String>) -> Self {
        self.ai_style = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SceneGeneration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](SceneGenerationBuilder::r#type)
    pub fn build(self) -> Result<SceneGeneration, BuildError> {
        Ok(SceneGeneration {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            ai_style: self.ai_style,
        })
    }
}
