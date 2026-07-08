pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhookEndpoint {
    /// Webhook endpoint id (e.g. `ep_...`).
    #[serde(rename = "endpointId")]
    #[serde(default)]
    pub endpoint_id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub events: Vec<WebhookEventName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Seconds since epoch (Unix timestamp) when the endpoint was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    /// HMAC secret for verifying [Standard Webhooks](https://www.standardwebhooks.com/) signatures. Only returned once on create; store it securely.
    #[serde(rename = "signingSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_secret: Option<String>,
    /// Last four characters of the signing secret, for display purposes.
    #[serde(rename = "signingSecretLast4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_secret_last4: Option<String>,
}

impl WebhookEndpoint {
    pub fn builder() -> WebhookEndpointBuilder {
        <WebhookEndpointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookEndpointBuilder {
    endpoint_id: Option<String>,
    url: Option<String>,
    events: Option<Vec<WebhookEventName>>,
    description: Option<String>,
    created_at: Option<f64>,
    signing_secret: Option<String>,
    signing_secret_last4: Option<String>,
}

impl WebhookEndpointBuilder {
    pub fn endpoint_id(mut self, value: impl Into<String>) -> Self {
        self.endpoint_id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<WebhookEventName>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn signing_secret(mut self, value: impl Into<String>) -> Self {
        self.signing_secret = Some(value.into());
        self
    }

    pub fn signing_secret_last4(mut self, value: impl Into<String>) -> Self {
        self.signing_secret_last4 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookEndpoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`endpoint_id`](WebhookEndpointBuilder::endpoint_id)
    /// - [`url`](WebhookEndpointBuilder::url)
    /// - [`events`](WebhookEndpointBuilder::events)
    /// - [`created_at`](WebhookEndpointBuilder::created_at)
    pub fn build(self) -> Result<WebhookEndpoint, BuildError> {
        Ok(WebhookEndpoint {
            endpoint_id: self.endpoint_id.ok_or_else(|| BuildError::missing_field("endpoint_id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            description: self.description,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            signing_secret: self.signing_secret,
            signing_secret_last4: self.signing_secret_last4,
        })
    }
}
