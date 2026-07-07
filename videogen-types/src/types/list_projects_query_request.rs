pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listProjects
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListProjectsQueryRequest {
    /// Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// When true, returns only items created by the API key's owner. When false (default), returns all items accessible to the team.
    #[serde(rename = "selfOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_only: Option<bool>,
    /// When true, includes dashboard-created projects in addition to API-created projects. When false (default), returns only API-created projects.
    #[serde(rename = "includeUiProjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_ui_projects: Option<bool>,
}

impl ListProjectsQueryRequest {
    pub fn builder() -> ListProjectsQueryRequestBuilder {
        <ListProjectsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProjectsQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
    self_only: Option<bool>,
    include_ui_projects: Option<bool>,
}

impl ListProjectsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn self_only(mut self, value: bool) -> Self {
        self.self_only = Some(value);
        self
    }

    pub fn include_ui_projects(mut self, value: bool) -> Self {
        self.include_ui_projects = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProjectsQueryRequest`].
    pub fn build(self) -> Result<ListProjectsQueryRequest, BuildError> {
        Ok(ListProjectsQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
            self_only: self.self_only,
            include_ui_projects: self.include_ui_projects,
        })
    }
}

