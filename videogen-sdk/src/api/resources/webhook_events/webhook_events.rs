use crate::{ApiError, ClientConfig, HttpClient};

pub struct WebhookEventsClient {
    pub http_client: HttpClient,
}

impl WebhookEventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }
}
