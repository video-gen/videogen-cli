pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listEntities
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEntitiesQueryRequest {
    /// When provided, returns only entities of this type. Omit to return all entities.
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<ListEntitiesRequestEntityType>,
    /// Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListEntitiesQueryRequest {
    pub fn builder() -> ListEntitiesQueryRequestBuilder {
        <ListEntitiesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEntitiesQueryRequestBuilder {
    entity_type: Option<ListEntitiesRequestEntityType>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListEntitiesQueryRequestBuilder {
    pub fn entity_type(mut self, value: ListEntitiesRequestEntityType) -> Self {
        self.entity_type = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEntitiesQueryRequest`].
    pub fn build(self) -> Result<ListEntitiesQueryRequest, BuildError> {
        Ok(ListEntitiesQueryRequest {
            entity_type: self.entity_type,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}

