pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StoryboardToVideoRequest {
    /// Ordered list of scenes. Each scene becomes one section in the final video, in this order.
    #[serde(default)]
    pub scenes: Vec<GenerateStoryboardScene>,
    /// Default generation applied to scenes that don't set their own `generation`. Defaults to AI_IMAGE with no extra style.
    #[serde(rename = "defaultGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_generation: Option<SceneGeneration>,
    /// Default per-scene duration in seconds for scenes that don't set their own `durationSeconds`. Defaults to 5. For AI_VIDEO scenes this must be a whole number between 1 and 15.
    #[serde(rename = "defaultDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_duration_seconds: Option<i64>,
    /// Generation quality tier for every scene. LOW is fastest and cheapest; STANDARD balances quality and cost; HIGH is highest quality. Defaults to STANDARD. LOW is not supported for AI_VIDEO scenes: the request is rejected if any scene is generated as a video at LOW quality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<StoryboardToVideoRequestQuality>,
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>,
    /// Optional storyboard-wide production notes for the AI that builds the video (e.g. recurring characters or props, a consistent setting, or overall staging guidance). Applies across every scene; per-scene direction goes in each scene's `prompt`.
    #[serde(rename = "workflowAgentContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_agent_context: Option<String>,
    /// Optional edits applied to the project after the video is built, in order. Each action runs asynchronously; the response returns one remix action id per action. `ENABLE_CAPTIONS` shows and styles captions, `SET_BACKGROUND_MUSIC` sets a music track, `ADD_TRANSITIONS` stamps transitions between scenes, and `SET_LOGO` overlays a logo. `EDIT_WITH_AGENT` applies open-ended natural-language edits. See the [Remix actions](/remix-actions) guide.
    #[serde(rename = "remixActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_actions: Option<Vec<RemixAction>>,
}

impl StoryboardToVideoRequest {
    pub fn builder() -> StoryboardToVideoRequestBuilder {
        <StoryboardToVideoRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StoryboardToVideoRequestBuilder {
    scenes: Option<Vec<GenerateStoryboardScene>>,
    default_generation: Option<SceneGeneration>,
    default_duration_seconds: Option<i64>,
    quality: Option<StoryboardToVideoRequestQuality>,
    aspect_ratio: Option<AspectRatio>,
    workflow_agent_context: Option<String>,
    remix_actions: Option<Vec<RemixAction>>,
}

impl StoryboardToVideoRequestBuilder {
    pub fn scenes(mut self, value: Vec<GenerateStoryboardScene>) -> Self {
        self.scenes = Some(value);
        self
    }

    pub fn default_generation(mut self, value: SceneGeneration) -> Self {
        self.default_generation = Some(value);
        self
    }

    pub fn default_duration_seconds(mut self, value: i64) -> Self {
        self.default_duration_seconds = Some(value);
        self
    }

    pub fn quality(mut self, value: StoryboardToVideoRequestQuality) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn aspect_ratio(mut self, value: AspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn workflow_agent_context(mut self, value: impl Into<String>) -> Self {
        self.workflow_agent_context = Some(value.into());
        self
    }

    pub fn remix_actions(mut self, value: Vec<RemixAction>) -> Self {
        self.remix_actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StoryboardToVideoRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scenes`](StoryboardToVideoRequestBuilder::scenes)
    pub fn build(self) -> Result<StoryboardToVideoRequest, BuildError> {
        Ok(StoryboardToVideoRequest {
            scenes: self.scenes.ok_or_else(|| BuildError::missing_field("scenes"))?,
            default_generation: self.default_generation,
            default_duration_seconds: self.default_duration_seconds,
            quality: self.quality,
            aspect_ratio: self.aspect_ratio,
            workflow_agent_context: self.workflow_agent_context,
            remix_actions: self.remix_actions,
        })
    }
}
