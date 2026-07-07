pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Paginated list of projects, most recently updated first. By default only API-created projects are included; pass `includeUiProjects=true` on the request to also include dashboard-created projects.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListProjectsResponse {
    #[serde(default)]
    pub projects: Vec<ProjectResponse>,
    /// When true, there are more projects available. Pass `nextCursor` as the `cursor` query param to fetch the next page.
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    /// Opaque cursor to fetch the next page. `null` when `hasMore` is false.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl ListProjectsResponse {
    pub fn builder() -> ListProjectsResponseBuilder {
        <ListProjectsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProjectsResponseBuilder {
    projects: Option<Vec<ProjectResponse>>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl ListProjectsResponseBuilder {
    pub fn projects(mut self, value: Vec<ProjectResponse>) -> Self {
        self.projects = Some(value);
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

    /// Consumes the builder and constructs a [`ListProjectsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`projects`](ListProjectsResponseBuilder::projects)
    /// - [`has_more`](ListProjectsResponseBuilder::has_more)
    pub fn build(self) -> Result<ListProjectsResponse, BuildError> {
        Ok(ListProjectsResponse {
            projects: self.projects.ok_or_else(|| BuildError::missing_field("projects"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
        })
    }
}
