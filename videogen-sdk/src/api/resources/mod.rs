//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Workflows**
//! - **Projects**
//! - **Tools**
//! - **Files**
//! - **Entities**
//! - **Text**
//! - **Resources**
//! - **Webhooks**
//! - **Account**
//! - **WebhookEvents**

use crate::{ApiError, ClientConfig};

pub mod account;
pub mod entities;
pub mod files;
pub mod projects;
pub mod resources;
pub mod text;
pub mod tools;
pub mod webhook_events;
pub mod webhooks;
pub mod workflows;
pub struct ApiClient {
    pub config: ClientConfig,
    pub workflows: WorkflowsClient,
    pub projects: ProjectsClient,
    pub tools: ToolsClient,
    pub files: FilesClient,
    pub entities: EntitiesClient,
    pub text: TextClient,
    pub resources: ResourcesClient,
    pub webhooks: WebhooksClient,
    pub account: AccountClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            workflows: WorkflowsClient::new(config.clone())?,
            projects: ProjectsClient::new(config.clone())?,
            tools: ToolsClient::new(config.clone())?,
            files: FilesClient::new(config.clone())?,
            entities: EntitiesClient::new(config.clone())?,
            text: TextClient::new(config.clone())?,
            resources: ResourcesClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
            account: AccountClient::new(config.clone())?,
        })
    }
}

pub use account::AccountClient;
pub use entities::EntitiesClient;
pub use files::FilesClient;
pub use projects::ProjectsClient;
pub use resources::ResourcesClient;
pub use text::TextClient;
pub use tools::ToolsClient;
pub use webhook_events::WebhookEventsClient;
pub use webhooks::WebhooksClient;
pub use workflows::WorkflowsClient;
