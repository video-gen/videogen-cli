use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ResourcesClient {
    pub http_client: HttpClient,
}

impl ResourcesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List available avatar presenters. Pass an `avatarPresenterId` from the response to the avatar video endpoint or to a script/slideshow workflow. Pass a reference `voiceId` to return presenters sorted by best match for that voice. Cursor-paginated; see the [Pagination](/pagination) guide.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    /// * `cursor` - Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    /// * `voice_id` - Optional reference voice id from `GET /v1/resources/tts-voices` (e.g. `vg_voic_...`). When provided, avatar presenters are returned sorted by best match for that voice (best first). Omit to return presenters in the default catalogue order.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_avatar_presenters(
        &self,
        request: &ListAvatarPresentersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AvatarPresenterListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/resources/avatar-presenters",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .string("voiceId", request.voice_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// List available text-to-speech voices. Pass a `voiceId` from the response to the text-to-speech endpoint. Cursor-paginated; see the [Pagination](/pagination) guide.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    /// * `cursor` - Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    /// * `include_deprecated_voices` - When true, includes voices that are deprecated but still callable. Defaults to false.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_tts_voices(
        &self,
        request: &ListTtsVoicesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TtsVoiceListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/resources/tts-voices",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .bool(
                        "includeDeprecatedVoices",
                        request.include_deprecated_voices.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// List the languages a project can be translated into. Pass a `languageCode` from the response to the `TRANSLATE_PROJECT` remix action. Returns the full catalogue in a single response (not paginated).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_languages(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<LanguageListResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/resources/languages", None, None, options)
            .await
    }
}
