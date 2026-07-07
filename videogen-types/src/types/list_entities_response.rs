pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEntitiesResponse {
    #[serde(default)]
    pub entities: Vec<Entity>,
    /// When true, there are more entities available. Pass `nextCursor` as the `cursor` query param to fetch the next page.
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    /// Opaque cursor to fetch the next page. `null` when `hasMore` is false.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl ListEntitiesResponse {
    pub fn builder() -> ListEntitiesResponseBuilder {
        <ListEntitiesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEntitiesResponseBuilder {
    entities: Option<Vec<Entity>>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl ListEntitiesResponseBuilder {
    pub fn entities(mut self, value: Vec<Entity>) -> Self {
        self.entities = Some(value);
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

    /// Consumes the builder and constructs a [`ListEntitiesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entities`](ListEntitiesResponseBuilder::entities)
    /// - [`has_more`](ListEntitiesResponseBuilder::has_more)
    pub fn build(self) -> Result<ListEntitiesResponse, BuildError> {
        Ok(ListEntitiesResponse {
            entities: self.entities.ok_or_else(|| BuildError::missing_field("entities"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
        })
    }
}
