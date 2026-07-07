use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List configured webhook endpoints for your account. Cursor-paginated; see the [Pagination](/pagination) guide.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    /// * `cursor` - Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_webhook_endpoints(
        &self,
        request: &ListWebhookEndpointsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhookEndpointListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/webhooks/endpoints",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Register a new webhook endpoint to receive `tool_execution.*`, `workflow_run.*`, and `file.*` events. The signing secret is only returned in this response. Store it securely.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_webhook_endpoint(
        &self,
        request: &CreateWebhookEndpointRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhookEndpoint, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhooks/endpoints",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a webhook endpoint. It will stop receiving events immediately.
    ///
    /// # Arguments
    ///
    /// * `endpoint_id` - The webhook endpoint id returned by `POST /v1/webhooks/endpoints`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn delete_webhook_endpoint(
        &self,
        endpoint_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/webhooks/endpoints/{}", endpoint_id),
                None,
                None,
                options,
            )
            .await
    }
}
