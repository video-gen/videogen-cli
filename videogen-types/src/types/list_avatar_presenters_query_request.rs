pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listAvatarPresenters
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAvatarPresentersQueryRequest {
    /// Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Optional reference voice id from `GET /v1/resources/tts-voices` (e.g. `vg_voic_...`). When provided, avatar presenters are returned sorted by best match for that voice (best first). Omit to return presenters in the default catalogue order.
    #[serde(rename = "voiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

impl ListAvatarPresentersQueryRequest {
    pub fn builder() -> ListAvatarPresentersQueryRequestBuilder {
        <ListAvatarPresentersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAvatarPresentersQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
    voice_id: Option<String>,
}

impl ListAvatarPresentersQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAvatarPresentersQueryRequest`].
    pub fn build(self) -> Result<ListAvatarPresentersQueryRequest, BuildError> {
        Ok(ListAvatarPresentersQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
            voice_id: self.voice_id,
        })
    }
}

