pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AvatarPresenterListResponse {
    #[serde(rename = "avatarPresenters")]
    #[serde(default)]
    pub avatar_presenters: Vec<AvatarPresenter>,
    /// When true, there are more avatar presenters available. Pass `nextCursor` as the `cursor` query param to fetch the next page.
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    /// Opaque cursor to fetch the next page. `null` when `hasMore` is false.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl AvatarPresenterListResponse {
    pub fn builder() -> AvatarPresenterListResponseBuilder {
        <AvatarPresenterListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AvatarPresenterListResponseBuilder {
    avatar_presenters: Option<Vec<AvatarPresenter>>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl AvatarPresenterListResponseBuilder {
    pub fn avatar_presenters(mut self, value: Vec<AvatarPresenter>) -> Self {
        self.avatar_presenters = Some(value);
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

    /// Consumes the builder and constructs a [`AvatarPresenterListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`avatar_presenters`](AvatarPresenterListResponseBuilder::avatar_presenters)
    /// - [`has_more`](AvatarPresenterListResponseBuilder::has_more)
    pub fn build(self) -> Result<AvatarPresenterListResponse, BuildError> {
        Ok(AvatarPresenterListResponse {
            avatar_presenters: self.avatar_presenters.ok_or_else(|| BuildError::missing_field("avatar_presenters"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
        })
    }
}
