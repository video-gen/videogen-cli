use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FilesClient {
    pub http_client: HttpClient,
}

impl FilesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List files in your account, including generated assets and uploads. Files are returned most recently updated first. Cursor-paginated; see the [Pagination](/pagination) guide.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    /// * `cursor` - Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_files(
        &self,
        request: &GetFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/files",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Semantic vector search over your files. Embeds the query text and returns the closest matching files ranked by cosine similarity. Only files with indexed descriptions are searchable.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn search_files(
        &self,
        request: &SearchFilesRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/files/search",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve metadata for a single file by its id.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file id (e.g. `vg_file_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_file(
        &self,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<StorageFile, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/files/{}", file_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Create a new file and receive a pre-signed upload URL. PUT the file bytes to the returned URL, then poll `GET /v1/files/{fileId}` until the file is ready.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_file_upload(
        &self,
        request: &CreateFileUploadRequest,
        options: Option<RequestOptions>,
    ) -> Result<FileUploadResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/files/upload",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate fresh signed URLs for all available renditions of a file. Call this when source URLs are missing or expired. Returns the full file object with populated `thumbnailSource`, `previewSource`, and `downloadSource`.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file id (e.g. `vg_file_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn hydrate_file(
        &self,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<StorageFile, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/files/{}/hydrate", file_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Archive a file by setting its archived timestamp. Archived files are excluded from list results. Returns the updated file object.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file id (e.g. `vg_file_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn archive_file(
        &self,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<StorageFile, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/files/{}/archive", file_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Enable public preview for a file. Works for any file type. Copies the file to a permanent public URL (`staticPublicPreviewSource`) and, for video and audio, registers a public embed playback id (`publicPlaybackId`) for use with `@videogen/player`. If streaming playback is still processing, the endpoint polls briefly and background processing finishes creating the embed playback id. Returns the updated file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file id (e.g. `vg_file_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn enable_public_preview(
        &self,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<StorageFile, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/files/{}/enable-public-preview", file_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Disable public preview for a file. Removes the permanent public URL copy and revokes unauthenticated embed streaming access. Authenticated signed URLs remain functional. Returns the updated file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file id (e.g. `vg_file_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn disable_public_preview(
        &self,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<StorageFile, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/files/{}/disable-public-preview", file_id),
                None,
                None,
                options,
            )
            .await
    }
}
