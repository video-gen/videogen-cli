pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GenerateMusicRequest {
    /// A text description of the music to generate. Include genre, mood, instrumentation, and tempo for best results.
    #[serde(default)]
    pub prompt: String,
    /// Number of output results to generate. Defaults to 1.
    #[serde(rename = "numResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// When true, generated files are temporary. Temporary files are guaranteed to be available for 24 hours, after which they may be archived at any time. Temporary files are not analyzed (no description, transcript, or embedding will be generated), so they will not appear in search results. Defaults to false.
    #[serde(rename = "isOutputTemporary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_output_temporary: Option<bool>,
}

impl GenerateMusicRequest {
    pub fn builder() -> GenerateMusicRequestBuilder {
        <GenerateMusicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateMusicRequestBuilder {
    prompt: Option<String>,
    num_results: Option<i64>,
    is_output_temporary: Option<bool>,
}

impl GenerateMusicRequestBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
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

    /// Consumes the builder and constructs a [`GenerateMusicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](GenerateMusicRequestBuilder::prompt)
    pub fn build(self) -> Result<GenerateMusicRequest, BuildError> {
        Ok(GenerateMusicRequest {
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            num_results: self.num_results,
            is_output_temporary: self.is_output_temporary,
        })
    }
}

