use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WorkflowsClient {
    pub http_client: HttpClient,
}

impl WorkflowsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a project and generates a narrated video from a prompt or script. Returns immediately with a workflow run id; poll or subscribe to webhooks for completion.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn script_to_video(
        &self,
        request: &ScriptToVideoRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartWorkflowRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/script-to-video",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Legacy alias for `POST /v1/workflows/script-to-video`. Use that endpoint instead.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_visuals_narrations_and_captions_to_script(
        &self,
        request: &ScriptToVideoRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartWorkflowRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/add-visuals-narrations-and-captions-to-script",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a project from an uploaded voiceover file and generates a video with matching b-roll. Upload the voiceover via the files API first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn voiceover_to_video(
        &self,
        request: &VoiceoverToVideoRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartWorkflowRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/voiceover-to-video",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Legacy alias for `POST /v1/workflows/voiceover-to-video`. Use that endpoint instead.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_visuals_and_captions_to_voiceover(
        &self,
        request: &VoiceoverToVideoRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartWorkflowRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/add-visuals-and-captions-to-voiceover",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a project from an uploaded PDF or PowerPoint file and generates an AI-narrated video walking through each slide. Upload the file via `POST /v1/files/upload` first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn slideshow_to_video(
        &self,
        request: &SlideshowToVideoRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartWorkflowRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/slideshow-to-video",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Legacy alias for `POST /v1/workflows/slideshow-to-video`. Use that endpoint instead.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_narration_transitions_and_captions_to_slideshow(
        &self,
        request: &SlideshowToVideoRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartWorkflowRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/add-narration-transitions-and-captions-to-slideshow",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a project from an ordered list of scenes and generates one section per scene. Each scene is generated from its prompt as either a still image or a video clip; the scenes are then assembled into a single video. Returns immediately with a workflow run id; poll or subscribe to webhooks for completion.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn storyboard_to_video(
        &self,
        request: &StoryboardToVideoRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartWorkflowRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/storyboard-to-video",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Legacy alias for `POST /v1/workflows/storyboard-to-video`. Use that endpoint instead.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn generate_scenes_from_storyboard(
        &self,
        request: &StoryboardToVideoRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartWorkflowRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/generate-scenes-from-storyboard",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List workflow runs started via the API, most recently created first. Use `selfOnly=true` to restrict results to the calling API key's user; otherwise all runs for the team are returned. Cursor-paginated; see the [Pagination](/pagination) guide.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    /// * `cursor` - Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    /// * `self_only` - When true, returns only items created by the API key's owner. When false (default), returns all items accessible to the team.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_workflow_runs(
        &self,
        request: &ListWorkflowRunsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowRunListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workflows/runs",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .bool("selfOnly", request.self_only.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_workflow_run(
        &self,
        workflow_run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowRun, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/runs/{}", workflow_run_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn cancel_workflow_run(
        &self,
        workflow_run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<StartWorkflowRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/runs/{}/cancel", workflow_run_id),
                None,
                None,
                options,
            )
            .await
    }
}
