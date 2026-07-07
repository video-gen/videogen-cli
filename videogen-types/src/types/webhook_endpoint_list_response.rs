pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhookEndpointListResponse {
    #[serde(default)]
    pub endpoints: Vec<WebhookEndpoint>,
    /// When true, there are more endpoints available. Pass `nextCursor` as the `cursor` query param to fetch the next page.
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    /// Opaque cursor to fetch the next page. `null` when `hasMore` is false.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl WebhookEndpointListResponse {
    pub fn builder() -> WebhookEndpointListResponseBuilder {
        <WebhookEndpointListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookEndpointListResponseBuilder {
    endpoints: Option<Vec<WebhookEndpoint>>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl WebhookEndpointListResponseBuilder {
    pub fn endpoints(mut self, value: Vec<WebhookEndpoint>) -> Self {
        self.endpoints = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookEndpointListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`endpoints`](WebhookEndpointListResponseBuilder::endpoints)
    /// - [`has_more`](WebhookEndpointListResponseBuilder::has_more)
    pub fn build(self) -> Result<WebhookEndpointListResponse, BuildError> {
        Ok(WebhookEndpointListResponse {
            endpoints: self.endpoints.ok_or_else(|| BuildError::missing_field("endpoints"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
        })
    }
}
