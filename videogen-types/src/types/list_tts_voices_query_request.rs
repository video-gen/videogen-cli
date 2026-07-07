pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listTtsVoices
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTtsVoicesQueryRequest {
    /// Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// When true, includes voices that are deprecated but still callable. Defaults to false.
    #[serde(rename = "includeDeprecatedVoices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deprecated_voices: Option<bool>,
}

impl ListTtsVoicesQueryRequest {
    pub fn builder() -> ListTtsVoicesQueryRequestBuilder {
        <ListTtsVoicesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTtsVoicesQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
    include_deprecated_voices: Option<bool>,
}

impl ListTtsVoicesQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn include_deprecated_voices(mut self, value: bool) -> Self {
        self.include_deprecated_voices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTtsVoicesQueryRequest`].
    pub fn build(self) -> Result<ListTtsVoicesQueryRequest, BuildError> {
        Ok(ListTtsVoicesQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
            include_deprecated_voices: self.include_deprecated_voices,
        })
    }
}

