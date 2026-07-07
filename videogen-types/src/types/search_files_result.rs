pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchFilesResult {
    /// Cosine similarity between the query embedding and the file description embedding. Ranges from 0 (no match) to 1 (identical). Values above 0.7 typically indicate strong relevance.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub similarity: f64,
    pub file: StorageFile,
}

impl SearchFilesResult {
    pub fn builder() -> SearchFilesResultBuilder {
        <SearchFilesResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchFilesResultBuilder {
    similarity: Option<f64>,
    file: Option<StorageFile>,
}

impl SearchFilesResultBuilder {
    pub fn similarity(mut self, value: f64) -> Self {
        self.similarity = Some(value);
        self
    }

    pub fn file(mut self, value: StorageFile) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchFilesResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`similarity`](SearchFilesResultBuilder::similarity)
    /// - [`file`](SearchFilesResultBuilder::file)
    pub fn build(self) -> Result<SearchFilesResult, BuildError> {
        Ok(SearchFilesResult {
            similarity: self.similarity.ok_or_else(|| BuildError::missing_field("similarity"))?,
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}
