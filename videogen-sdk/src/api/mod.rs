//! API client and types for the VideoGen API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    AccountClient, ApiClient, EntitiesClient, FilesClient, ProjectsClient, ResourcesClient,
    TextClient, ToolsClient, WebhookEventsClient, WebhooksClient, WorkflowsClient,
};

pub use videogen_types::*;
