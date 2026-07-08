use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ProjectsClient {
    pub http_client: HttpClient,
}

impl ProjectsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns projects, most recently updated first. By default only API-created projects are included; pass `includeUiProjects=true` to also include dashboard-created projects. Use `selfOnly=true` to restrict results to the calling API key's user; otherwise all matching projects for the team are returned. Cursor-paginated; see the [Pagination](/pagination) guide.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    /// * `cursor` - Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    /// * `self_only` - When true, returns only items created by the API key's owner. When false (default), returns all items accessible to the team.
    /// * `include_ui_projects` - When true, includes dashboard-created projects in addition to API-created projects. When false (default), returns only API-created projects.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_projects(
        &self,
        request: &ListProjectsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/projects",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .bool("selfOnly", request.self_only.clone())
                    .bool("includeUiProjects", request.include_ui_projects.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a simplified view of a project including its title, aspect ratio, status, and URL.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project id (e.g. `vg_proj_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_project(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProjectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}", project_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Starts an export of a project to MP4. Returns immediately with an export id; the file becomes available when the export task completes.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project id (e.g. `vg_proj_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn export_project(
        &self,
        project_id: &str,
        request: &ExportProjectRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExportProjectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/export", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the current status of a project export started via `POST /v1/projects/{projectId}/export`.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project id (e.g. `vg_proj_...`).
    /// * `export_id` - The export id (e.g. `vg_expo_...`) returned by `POST /v1/projects/{projectId}/export`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_project_export(
        &self,
        project_id: &str,
        export_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProjectExport, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/exports/{}", project_id, export_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Applies an ordered list of edits (background music, logo overlay, caption visibility/style) to a project. Each action runs asynchronously as its own remix action; the response returns one remix action id per action in order. Set `saveAsNewProject` to apply the edits to a copy and leave the original untouched. Poll `GET /v1/projects/{projectId}/remix-actions` for status.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project id (e.g. `vg_proj_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn remix_project(
        &self,
        project_id: &str,
        request: &RemixProjectRequest,
        options: Option<RequestOptions>,
    ) -> Result<RemixProjectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/remix", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns every remix action applied to a project (via `POST /v1/projects/{projectId}/remix` or as a post-workflow step), most recent first, with each action's status and progress.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project id (e.g. `vg_proj_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_project_remix_actions(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListRemixActionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/remix-actions", project_id),
                None,
                None,
                options,
            )
            .await
    }
}
