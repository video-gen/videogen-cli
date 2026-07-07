//! # VideoGen API SDK
//!
//! The official Rust SDK for the VideoGen API.
//!
//! ## Getting Started
//!
//! ```rust
//! use videogen_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         token: Some("<token>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = VideogenClient::new(config).expect("Failed to build client");
//!     client.workflows.script_to_video(&ScriptToVideoRequest {
//!         script: "Staying hydrated keeps your body and mind running at their best. Drinking enough water boosts your energy, focus, and mood. Keep a water bottle nearby and sip throughout the day.".to_string(),
//!         aspect_ratio: None,
//!         visual_style: WorkflowVisualStyle {
//!             r#type: WorkflowVisualStyleType::AiImage,
//!             ai_style: Some("loose watercolor illustration with visible brushstrokes and soft color bleeds".to_string()),
//!             entity_id: None,
//!             restyle_featured_b_roll_with_ai_style: None
//!         },
//!         visual_pacing: Some(VisualPacing::Medium),
//!         quality: Some(ScriptToVideoRequestQuality::High),
//!         language: None,
//!         voice_id: None,
//!         voice_speed: None,
//!         avatar_presenter_id: None,
//!         featured_b_roll_file_ids: None,
//!         workflow_agent_context: None,
//!         remix_actions: Some(vec![RemixAction::EnableCaptions {
//!             data: RemixActionEnableCaptions {
//!                 ..Default::default()
//!             }
//!         }, RemixAction::ConvertImagesToVideos {
//!             data: RemixActionConvertImagesToVideos {
//!                 motion_prompt: Some("slow cinematic push-in".to_string()),
//!                 mute_output_videos: Some(true),
//!                 quality: Some(VideoQuality::High),
//!                 ..Default::default()
//!             }
//!         }])
//!     }, None).await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
