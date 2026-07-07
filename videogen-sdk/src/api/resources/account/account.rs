use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AccountClient {
    pub http_client: HttpClient,
}

impl AccountClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Return the account and team behind the API key making the request. Takes no parameters. Call it as a connection test to confirm a key is valid and to discover the `teamId` and account `email` a key belongs to.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_me(&self, options: Option<RequestOptions>) -> Result<MeResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/me", None, None, options)
            .await
    }
}
