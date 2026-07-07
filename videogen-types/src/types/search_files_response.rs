pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchFilesResponse {
    #[serde(default)]
    pub results: Vec<SearchFilesResult>,
}

impl SearchFilesResponse {
    pub fn builder() -> SearchFilesResponseBuilder {
        <SearchFilesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchFilesResponseBuilder {
    results: Option<Vec<SearchFilesResult>>,
}

impl SearchFilesResponseBuilder {
    pub fn results(mut self, value: Vec<SearchFilesResult>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchFilesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](SearchFilesResponseBuilder::results)
    pub fn build(self) -> Result<SearchFilesResponse, BuildError> {
        Ok(SearchFilesResponse {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
