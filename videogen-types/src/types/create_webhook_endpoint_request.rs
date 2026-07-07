pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookEndpointRequest {
    /// HTTPS URL that will receive webhook POST requests.
    #[serde(default)]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub events: Vec<WebhookEventName>,
}

impl CreateWebhookEndpointRequest {
    pub fn builder() -> CreateWebhookEndpointRequestBuilder {
        <CreateWebhookEndpointRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookEndpointRequestBuilder {
    url: Option<String>,
    description: Option<String>,
    events: Option<Vec<WebhookEventName>>,
}

impl CreateWebhookEndpointRequestBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<WebhookEventName>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookEndpointRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CreateWebhookEndpointRequestBuilder::url)
    /// - [`events`](CreateWebhookEndpointRequestBuilder::events)
    pub fn build(self) -> Result<CreateWebhookEndpointRequest, BuildError> {
        Ok(CreateWebhookEndpointRequest {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            description: self.description,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
        })
    }
}

