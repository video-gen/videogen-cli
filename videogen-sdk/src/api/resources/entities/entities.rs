use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EntitiesClient {
    pub http_client: HttpClient,
}

impl EntitiesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the actors and visual styles available to your team, most recently updated first. Cursor-paginated; see the [Pagination](/pagination) guide.
    ///
    /// # Arguments
    ///
    /// * `entity_type` - When provided, returns only entities of this type. Omit to return all entities.
    /// * `limit` - Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination).
    /// * `cursor` - Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_entities(
        &self,
        request: &ListEntitiesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEntitiesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/entities",
                None,
                QueryBuilder::new()
                    .serialize("entityType", request.entity_type.clone())
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new actor or visual style. Attach reference images with `POST /v1/entities/{entityId}/references`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_entity(
        &self,
        request: &CreateEntityRequest,
        options: Option<RequestOptions>,
    ) -> Result<Entity, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/entities",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a single entity by its id, including its reference images.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The entity id (e.g. `vg_enti_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_entity(
        &self,
        entity_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Entity, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/entities/{}", entity_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an entity's name or description. Provide at least one field.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The entity id (e.g. `vg_enti_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_entity(
        &self,
        entity_id: &str,
        request: &UpdateEntityRequest,
        options: Option<RequestOptions>,
    ) -> Result<Entity, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/entities/{}/update", entity_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Archive an entity. Archived entities no longer appear in `GET /v1/entities` and can't be attached to new workflows.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The entity id (e.g. `vg_enti_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn archive_entity(
        &self,
        entity_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<EntityArchiveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/entities/{}/archive", entity_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Attach an image file as a reference for the entity. Upload the image first via `POST /v1/files/upload`. Returns the updated entity.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The entity id (e.g. `vg_enti_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_entity_reference(
        &self,
        entity_id: &str,
        request: &AddEntityReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<Entity, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/entities/{}/references", entity_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Detach a reference image from the entity. Returns the updated entity.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The entity id (e.g. `vg_enti_...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn remove_entity_reference(
        &self,
        entity_id: &str,
        request: &RemoveEntityReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<Entity, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/entities/{}/references/remove", entity_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
