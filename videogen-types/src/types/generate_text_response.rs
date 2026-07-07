pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GenerateTextResponse {
    /// The generated text.
    #[serde(default)]
    pub text: String,
}

impl GenerateTextResponse {
    pub fn builder() -> GenerateTextResponseBuilder {
        <GenerateTextResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateTextResponseBuilder {
    text: Option<String>,
}

impl GenerateTextResponseBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GenerateTextResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](GenerateTextResponseBuilder::text)
    pub fn build(self) -> Result<GenerateTextResponse, BuildError> {
        Ok(GenerateTextResponse {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
