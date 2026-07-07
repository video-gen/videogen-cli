pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GenerateTextRequest {
    /// The instruction or content to generate text from.
    #[serde(default)]
    pub prompt: String,
    /// Optional system instructions that steer the model's role, tone, and constraints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Generation quality tier. `LOW` is fastest and cheapest; `STANDARD` balances quality and cost; `HIGH` is higher quality; `MAX` is highest quality. Defaults to `STANDARD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<GenerateTextRequestQuality>,
    /// Sampling temperature. Higher values produce more varied output. Defaults to the model's default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature: Option<f64>,
    /// Maximum number of tokens to generate. Defaults to 512.
    #[serde(rename = "maxOutputTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,
}

impl GenerateTextRequest {
    pub fn builder() -> GenerateTextRequestBuilder {
        <GenerateTextRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateTextRequestBuilder {
    prompt: Option<String>,
    system: Option<String>,
    quality: Option<GenerateTextRequestQuality>,
    temperature: Option<f64>,
    max_output_tokens: Option<i64>,
}

impl GenerateTextRequestBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn quality(mut self, value: GenerateTextRequestQuality) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn max_output_tokens(mut self, value: i64) -> Self {
        self.max_output_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GenerateTextRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](GenerateTextRequestBuilder::prompt)
    pub fn build(self) -> Result<GenerateTextRequest, BuildError> {
        Ok(GenerateTextRequest {
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            system: self.system,
            quality: self.quality,
            temperature: self.temperature,
            max_output_tokens: self.max_output_tokens,
        })
    }
}

