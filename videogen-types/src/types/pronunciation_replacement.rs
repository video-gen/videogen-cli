pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PronunciationReplacement {
    #[serde(default)]
    pub original: String,
    #[serde(default)]
    pub replacement: String,
}

impl PronunciationReplacement {
    pub fn builder() -> PronunciationReplacementBuilder {
        <PronunciationReplacementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationReplacementBuilder {
    original: Option<String>,
    replacement: Option<String>,
}

impl PronunciationReplacementBuilder {
    pub fn original(mut self, value: impl Into<String>) -> Self {
        self.original = Some(value.into());
        self
    }

    pub fn replacement(mut self, value: impl Into<String>) -> Self {
        self.replacement = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PronunciationReplacement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`original`](PronunciationReplacementBuilder::original)
    /// - [`replacement`](PronunciationReplacementBuilder::replacement)
    pub fn build(self) -> Result<PronunciationReplacement, BuildError> {
        Ok(PronunciationReplacement {
            original: self.original.ok_or_else(|| BuildError::missing_field("original"))?,
            replacement: self.replacement.ok_or_else(|| BuildError::missing_field("replacement"))?,
        })
    }
}
