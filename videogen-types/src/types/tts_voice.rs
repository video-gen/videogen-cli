pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A text-to-speech voice.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TtsVoice {
    /// Voice id (e.g. `vg_voic_...`). Pass as `voiceId` to `POST /v1/tools/text-to-speech`.
    #[serde(rename = "voiceId")]
    #[serde(default)]
    pub voice_id: String,
    /// Locale tag for the voice (e.g. `en-US`, `es-ES`).
    #[serde(rename = "languageCode")]
    #[serde(default)]
    pub language_code: String,
    /// Human-readable voice name.
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    /// Voice gender.
    #[serde(rename = "displayGender")]
    pub display_gender: TtsVoiceDisplayGender,
    /// Accent (e.g. `american`, `british`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    /// Description of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When false, this voice cannot be used directly with `POST /v1/tools/text-to-speech`. All voices, regardless of this field, can be used in full video generation workflows such as script-to-video.
    #[serde(rename = "supportsDirectToolExecution")]
    #[serde(default)]
    pub supports_direct_tool_execution: bool,
    /// When true, this voice can synthesize text in any language regardless of its `languageCode`. When false, the voice only supports its listed language.
    #[serde(rename = "supportsAllLanguages")]
    #[serde(default)]
    pub supports_all_languages: bool,
    /// When true, this voice is deprecated and may be removed in a future API version. Prefer non-deprecated voices for new integrations.
    #[serde(rename = "isDeprecated")]
    #[serde(default)]
    pub is_deprecated: bool,
}

impl TtsVoice {
    pub fn builder() -> TtsVoiceBuilder {
        <TtsVoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TtsVoiceBuilder {
    voice_id: Option<String>,
    language_code: Option<String>,
    display_name: Option<String>,
    display_gender: Option<TtsVoiceDisplayGender>,
    accent: Option<String>,
    description: Option<String>,
    supports_direct_tool_execution: Option<bool>,
    supports_all_languages: Option<bool>,
    is_deprecated: Option<bool>,
}

impl TtsVoiceBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn display_gender(mut self, value: TtsVoiceDisplayGender) -> Self {
        self.display_gender = Some(value);
        self
    }

    pub fn accent(mut self, value: impl Into<String>) -> Self {
        self.accent = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn supports_direct_tool_execution(mut self, value: bool) -> Self {
        self.supports_direct_tool_execution = Some(value);
        self
    }

    pub fn supports_all_languages(mut self, value: bool) -> Self {
        self.supports_all_languages = Some(value);
        self
    }

    pub fn is_deprecated(mut self, value: bool) -> Self {
        self.is_deprecated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TtsVoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](TtsVoiceBuilder::voice_id)
    /// - [`language_code`](TtsVoiceBuilder::language_code)
    /// - [`display_name`](TtsVoiceBuilder::display_name)
    /// - [`display_gender`](TtsVoiceBuilder::display_gender)
    /// - [`supports_direct_tool_execution`](TtsVoiceBuilder::supports_direct_tool_execution)
    /// - [`supports_all_languages`](TtsVoiceBuilder::supports_all_languages)
    /// - [`is_deprecated`](TtsVoiceBuilder::is_deprecated)
    pub fn build(self) -> Result<TtsVoice, BuildError> {
        Ok(TtsVoice {
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            language_code: self.language_code.ok_or_else(|| BuildError::missing_field("language_code"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            display_gender: self.display_gender.ok_or_else(|| BuildError::missing_field("display_gender"))?,
            accent: self.accent,
            description: self.description,
            supports_direct_tool_execution: self.supports_direct_tool_execution.ok_or_else(|| BuildError::missing_field("supports_direct_tool_execution"))?,
            supports_all_languages: self.supports_all_languages.ok_or_else(|| BuildError::missing_field("supports_all_languages"))?,
            is_deprecated: self.is_deprecated.ok_or_else(|| BuildError::missing_field("is_deprecated"))?,
        })
    }
}
