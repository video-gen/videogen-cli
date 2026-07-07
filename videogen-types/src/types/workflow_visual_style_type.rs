pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// STOCK pulls stock footage and images. AI_IMAGE generates a styled image for each section. ENTITY generates images that match a visual-style entity's reference images for a consistent look.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkflowVisualStyleType {
    Stock,
    AiImage,
    Entity,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkflowVisualStyleType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Stock => serializer.serialize_str("STOCK"),
            Self::AiImage => serializer.serialize_str("AI_IMAGE"),
            Self::Entity => serializer.serialize_str("ENTITY"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkflowVisualStyleType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "STOCK" => Ok(Self::Stock),
            "AI_IMAGE" => Ok(Self::AiImage),
            "ENTITY" => Ok(Self::Entity),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkflowVisualStyleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stock => write!(f, "STOCK"),
            Self::AiImage => write!(f, "AI_IMAGE"),
            Self::Entity => write!(f, "ENTITY"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
