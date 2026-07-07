pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GenerateSoundEffectRequest {
    /// A text description of the sound effect to generate.
    #[serde(default)]
    pub prompt: String,
    /// Desired length of the sound effect in seconds, between 1 and 30. Defaults to about 10 seconds when omitted.
    #[serde(rename = "durationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<f64>,
    #[serde(rename = "promptInfluence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_influence: Option<f64>,
    /// Number of output results to generate. Defaults to 1.
    #[serde(rename = "numResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// When true, generated files are temporary. Temporary files are guaranteed to be available for 24 hours, after which they may be archived at any time. Temporary files are not analyzed (no description, transcript, or embedding will be generated), so they will not appear in search results. Defaults to false.
    #[serde(rename = "isOutputTemporary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_output_temporary: Option<bool>,
}

impl GenerateSoundEffectRequest {
    pub fn builder() -> GenerateSoundEffectRequestBuilder {
        <GenerateSoundEffectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateSoundEffectRequestBuilder {
    prompt: Option<String>,
    duration_seconds: Option<f64>,
    prompt_influence: Option<f64>,
    num_results: Option<i64>,
    is_output_temporary: Option<bool>,
}

impl GenerateSoundEffectRequestBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn duration_seconds(mut self, value: f64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn prompt_influence(mut self, value: f64) -> Self {
        self.prompt_influence = Some(value);
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

    /// Consumes the builder and constructs a [`GenerateSoundEffectRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](GenerateSoundEffectRequestBuilder::prompt)
    pub fn build(self) -> Result<GenerateSoundEffectRequest, BuildError> {
        Ok(GenerateSoundEffectRequest {
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            duration_seconds: self.duration_seconds,
            prompt_influence: self.prompt_influence,
            num_results: self.num_results,
            is_output_temporary: self.is_output_temporary,
        })
    }
}

