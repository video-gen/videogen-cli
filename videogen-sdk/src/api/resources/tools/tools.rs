use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ToolsClient {
    pub http_client: HttpClient,
}

impl ToolsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Generate an image from a text prompt, optionally guided by one or more reference images. When reference images are provided, the prompt describes the desired transformation. VideoGen automatically routes each request to the most effective state-of-the-art image model for your prompt, reference images, and quality tier, so you don't pick a model.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn generate_image(
        &self,
        request: &GenerateImageRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/generate-image",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate a single short video clip (up to 15 seconds) from a text prompt, optionally guided by reference images, videos, and audio. At least one of `prompt`, `imageFileIds`, `videoFileIds`, or `audioFileIds` must be provided. VideoGen automatically routes each request to the most effective state-of-the-art video model for your inputs and settings, so you don't pick a model. This endpoint returns one standalone clip. For longer, higher-quality, professionally edited videos with narration, captions, music, and multiple scenes, use a video workflow such as [Script to video](/workflows) (`POST /v1/workflows/script-to-video`) instead.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn generate_video_clip(
        &self,
        request: &GenerateVideoClipRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/generate-video-clip",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Convert text into a spoken audio file. Only voices with `supportsDirectToolExecution` set to true can be used. Optionally choose a voice, language, speed, and pronunciation overrides.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn text_to_speech(
        &self,
        request: &TextToSpeechRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/text-to-speech",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate a sound effect from a text description. Optionally control the duration and prompt influence. VideoGen automatically routes each request to the most effective state-of-the-art sound effect model for your prompt and settings, so you don't pick a model.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn generate_sound_effect(
        &self,
        request: &GenerateSoundEffectRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/generate-sound-effect",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate an instrumental music track from a text description. The returned track is approximately 30 seconds long. VideoGen automatically routes each request to the most effective state-of-the-art music model for your prompt, so you don't pick a model.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn generate_music(
        &self,
        request: &GenerateMusicRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/generate-music",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate a talking-head avatar video by pairing a presenter with an audio file, typically from a prior text-to-speech result.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn generate_avatar(
        &self,
        request: &GenerateAvatarRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/generate-avatar",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Convert any raster image into a scalable vector graphic (SVG). The output traces the shapes and colors of the input image.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn vectorize_image(
        &self,
        request: &ImageAssetRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/vectorize-image",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove the background from an image, returning a transparent-background PNG of the foreground subject.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn remove_image_background(
        &self,
        request: &ImageAssetRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/remove-image-background",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove the background from a video, producing a transparent-background video of the foreground subject.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn remove_video_background(
        &self,
        request: &VideoAssetRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/remove-video-background",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Increase the resolution of an image while preserving detail and sharpness.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn upscale_image(
        &self,
        request: &ImageAssetRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/upscale-image",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Increase the resolution of a video while preserving detail and sharpness.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn upscale_video(
        &self,
        request: &VideoAssetRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/upscale-video",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Turn a still image into a short video clip with a 3D parallax motion effect, simulating camera movement through the scene.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn image3d_effect(
        &self,
        request: &ImageAssetRequest,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/tools/image-3d-effect",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Request cancellation of a running tool execution. The execution transitions to `cancelled` if it has not already completed.
    ///
    /// # Arguments
    ///
    /// * `tool_execution_id` - The tool execution id returned when the tool was started.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cancel_tool_execution(
        &self,
        tool_execution_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<StartToolExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/tools/executions/{}/cancel", tool_execution_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List tool executions started via the API, most recently created first. Use `selfOnly=true` to restrict results to the calling API key's user; otherwise all executions for the team are returned. Cursor-paginated; see the [Pagination](/pagination) guide.
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
    pub async fn list_tool_executions(
        &self,
        request: &ListToolExecutionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ToolExecutionListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/tools/executions",
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

    /// Retrieve the current status and result of a tool execution. Poll this endpoint until `status` is `succeeded`, `failed`, or `cancelled`.
    ///
    /// # Arguments
    ///
    /// * `tool_execution_id` - The tool execution id returned when the tool was started.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_tool_execution_info(
        &self,
        tool_execution_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ExecutedTool, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/tools/executions/{}", tool_execution_id),
                None,
                None,
                options,
            )
            .await
    }
}
