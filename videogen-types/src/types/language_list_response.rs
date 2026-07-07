pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguageListResponse {
    #[serde(default)]
    pub languages: Vec<Language>,
}

impl LanguageListResponse {
    pub fn builder() -> LanguageListResponseBuilder {
        <LanguageListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguageListResponseBuilder {
    languages: Option<Vec<Language>>,
}

impl LanguageListResponseBuilder {
    pub fn languages(mut self, value: Vec<Language>) -> Self {
        self.languages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LanguageListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`languages`](LanguageListResponseBuilder::languages)
    pub fn build(self) -> Result<LanguageListResponse, BuildError> {
        Ok(LanguageListResponse {
            languages: self.languages.ok_or_else(|| BuildError::missing_field("languages"))?,
        })
    }
}
