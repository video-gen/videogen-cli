pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listToolExecutions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListToolExecutionsQueryRequest {
    /// Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// When true, returns only items created by the API key's owner. When false (default), returns all items accessible to the team.
    #[serde(rename = "selfOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_only: Option<bool>,
}

impl ListToolExecutionsQueryRequest {
    pub fn builder() -> ListToolExecutionsQueryRequestBuilder {
        <ListToolExecutionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListToolExecutionsQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
    self_only: Option<bool>,
}

impl ListToolExecutionsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn self_only(mut self, value: bool) -> Self {
        self.self_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListToolExecutionsQueryRequest`].
    pub fn build(self) -> Result<ListToolExecutionsQueryRequest, BuildError> {
        Ok(ListToolExecutionsQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
            self_only: self.self_only,
        })
    }
}

