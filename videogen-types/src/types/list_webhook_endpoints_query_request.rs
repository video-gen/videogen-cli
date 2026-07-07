pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listWebhookEndpoints
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWebhookEndpointsQueryRequest {
    /// Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListWebhookEndpointsQueryRequest {
    pub fn builder() -> ListWebhookEndpointsQueryRequestBuilder {
        <ListWebhookEndpointsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhookEndpointsQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListWebhookEndpointsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListWebhookEndpointsQueryRequest`].
    pub fn build(self) -> Result<ListWebhookEndpointsQueryRequest, BuildError> {
        Ok(ListWebhookEndpointsQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}

