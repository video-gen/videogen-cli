pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextToSpeechRequest {
    #[serde(rename = "ttsText")]
    #[serde(default)]
    pub tts_text: String,
    /// Voice id from `GET /v1/resources/tts-voices`. Only voices with `supportsDirectToolExecution` set to true are accepted.
    #[serde(rename = "voiceId")]
    #[serde(default)]
    pub voice_id: String,
    /// ISO-639-1 language hint for pronunciation (e.g. `en`, `es`, `zh`).
    #[serde(rename = "speechLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_language_code: Option<String>,
    #[serde(rename = "pronunciationReplacements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_replacements: Option<Vec<PronunciationReplacement>>,
    /// When true, automatically expands numbers, symbols, acronyms, and other non-word tokens into their spoken forms before synthesis so the voice pronounces them correctly (e.g. `$100` → `one hundred dollars`, `NASA` → `nasa`, `3rd` → `third`). Defaults to false when omitted.
    #[serde(rename = "autoExpandPronunciationReplacements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_expand_pronunciation_replacements: Option<bool>,
    /// Speech rate multiplier. Defaults to the voice's default speed.
    #[serde(rename = "voiceSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub voice_speed: Option<f64>,
    /// Number of output results to generate. Defaults to 1.
    #[serde(rename = "numResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// When true, generated files are temporary. Temporary files are guaranteed to be available for 24 hours, after which they may be archived at any time. Temporary files are not analyzed (no description, transcript, or embedding will be generated), so they will not appear in search results. Defaults to false.
    #[serde(rename = "isOutputTemporary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_output_temporary: Option<bool>,
}

impl TextToSpeechRequest {
    pub fn builder() -> TextToSpeechRequestBuilder {
        <TextToSpeechRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToSpeechRequestBuilder {
    tts_text: Option<String>,
    voice_id: Option<String>,
    speech_language_code: Option<String>,
    pronunciation_replacements: Option<Vec<PronunciationReplacement>>,
    auto_expand_pronunciation_replacements: Option<bool>,
    voice_speed: Option<f64>,
    num_results: Option<i64>,
    is_output_temporary: Option<bool>,
}

impl TextToSpeechRequestBuilder {
    pub fn tts_text(mut self, value: impl Into<String>) -> Self {
        self.tts_text = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn speech_language_code(mut self, value: impl Into<String>) -> Self {
        self.speech_language_code = Some(value.into());
        self
    }

    pub fn pronunciation_replacements(mut self, value: Vec<PronunciationReplacement>) -> Self {
        self.pronunciation_replacements = Some(value);
        self
    }

    pub fn auto_expand_pronunciation_replacements(mut self, value: bool) -> Self {
        self.auto_expand_pronunciation_replacements = Some(value);
        self
    }

    pub fn voice_speed(mut self, value: f64) -> Self {
        self.voice_speed = Some(value);
        self
    }

    pub fn num_results(mut self, value: i64) -> Self {
        self.num_results = Some(value);
        self
    }

    pub fn is_output_temporary(mut self, value: bool) -> Self {
        self.is_output_temporary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToSpeechRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tts_text`](TextToSpeechRequestBuilder::tts_text)
    /// - [`voice_id`](TextToSpeechRequestBuilder::voice_id)
    pub fn build(self) -> Result<TextToSpeechRequest, BuildError> {
        Ok(TextToSpeechRequest {
            tts_text: self.tts_text.ok_or_else(|| BuildError::missing_field("tts_text"))?,
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            speech_language_code: self.speech_language_code,
            pronunciation_replacements: self.pronunciation_replacements,
            auto_expand_pronunciation_replacements: self.auto_expand_pronunciation_replacements,
            voice_speed: self.voice_speed,
            num_results: self.num_results,
            is_output_temporary: self.is_output_temporary,
        })
    }
}

