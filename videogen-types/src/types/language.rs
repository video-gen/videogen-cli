pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A language a project can be translated into.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Language {
    /// The language code to pass to a `TRANSLATE_PROJECT` remix action (e.g. `es`, `fr`, `ja`).
    #[serde(rename = "languageCode")]
    #[serde(default)]
    pub language_code: String,
    /// Human-readable English name of the language (e.g. `Spanish`).
    #[serde(default)]
    pub name: String,
}

impl Language {
    pub fn builder() -> LanguageBuilder {
        <LanguageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguageBuilder {
    language_code: Option<String>,
    name: Option<String>,
}

impl LanguageBuilder {
    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Language`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language_code`](LanguageBuilder::language_code)
    /// - [`name`](LanguageBuilder::name)
    pub fn build(self) -> Result<Language, BuildError> {
        Ok(Language {
            language_code: self.language_code.ok_or_else(|| BuildError::missing_field("language_code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
