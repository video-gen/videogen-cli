pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TtsVoiceListResponse {
    #[serde(rename = "ttsVoices")]
    #[serde(default)]
    pub tts_voices: Vec<TtsVoice>,
    /// When true, there are more voices available. Pass `nextCursor` as the `cursor` query param to fetch the next page.
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    /// Opaque cursor to fetch the next page. `null` when `hasMore` is false.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl TtsVoiceListResponse {
    pub fn builder() -> TtsVoiceListResponseBuilder {
        <TtsVoiceListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TtsVoiceListResponseBuilder {
    tts_voices: Option<Vec<TtsVoice>>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl TtsVoiceListResponseBuilder {
    pub fn tts_voices(mut self, value: Vec<TtsVoice>) -> Self {
        self.tts_voices = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TtsVoiceListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tts_voices`](TtsVoiceListResponseBuilder::tts_voices)
    /// - [`has_more`](TtsVoiceListResponseBuilder::has_more)
    pub fn build(self) -> Result<TtsVoiceListResponse, BuildError> {
        Ok(TtsVoiceListResponse {
            tts_voices: self.tts_voices.ok_or_else(|| BuildError::missing_field("tts_voices"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
        })
    }
}
