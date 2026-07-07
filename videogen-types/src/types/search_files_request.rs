pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchFilesRequest {
    /// Natural-language search query. The text is embedded and compared against file description vectors using cosine similarity.
    #[serde(default)]
    pub query: String,
    /// Number of results to return (1-100). Defaults to 10.
    #[serde(rename = "numResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// When true, only files created by the calling API key's user are returned. When false (default), all files accessible to the team are included.
    #[serde(rename = "selfOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_only: Option<bool>,
}

impl SearchFilesRequest {
    pub fn builder() -> SearchFilesRequestBuilder {
        <SearchFilesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchFilesRequestBuilder {
    query: Option<String>,
    num_results: Option<i64>,
    self_only: Option<bool>,
}

impl SearchFilesRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn num_results(mut self, value: i64) -> Self {
        self.num_results = Some(value);
        self
    }

    pub fn self_only(mut self, value: bool) -> Self {
        self.self_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchFilesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](SearchFilesRequestBuilder::query)
    pub fn build(self) -> Result<SearchFilesRequest, BuildError> {
        Ok(SearchFilesRequest {
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            num_results: self.num_results,
            self_only: self.self_only,
        })
    }
}

