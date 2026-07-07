pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetFilesResponse {
    #[serde(default)]
    pub files: Vec<StorageFile>,
    /// When true, there are more files available. Pass `nextCursor` as the `cursor` query param to fetch the next page.
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    /// Opaque cursor to fetch the next page. `null` when `hasMore` is false.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl GetFilesResponse {
    pub fn builder() -> GetFilesResponseBuilder {
        <GetFilesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetFilesResponseBuilder {
    files: Option<Vec<StorageFile>>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl GetFilesResponseBuilder {
    pub fn files(mut self, value: Vec<StorageFile>) -> Self {
        self.files = Some(value);
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

    /// Consumes the builder and constructs a [`GetFilesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](GetFilesResponseBuilder::files)
    /// - [`has_more`](GetFilesResponseBuilder::has_more)
    pub fn build(self) -> Result<GetFilesResponse, BuildError> {
        Ok(GetFilesResponse {
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
        })
    }
}
