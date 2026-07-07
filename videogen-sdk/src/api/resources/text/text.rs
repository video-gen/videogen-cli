use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TextClient {
    pub http_client: HttpClient,
}

impl TextClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Generate text from a prompt using a general-purpose language model. Choose a quality tier with `quality` (`LOW`, `STANDARD`, `HIGH`, or `MAX`). Synchronous: the response includes the generated text. Useful for drafting scripts, titles, descriptions, and other short copy before generating a video.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn generate_text(
        &self,
        request: &GenerateTextRequest,
        options: Option<RequestOptions>,
    ) -> Result<GenerateTextResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/text/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
