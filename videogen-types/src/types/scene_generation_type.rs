pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// AI_IMAGE generates a still image. AI_VIDEO generates a video clip.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneGenerationType {
    AiImage,
    AiVideo,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SceneGenerationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AiImage => serializer.serialize_str("AI_IMAGE"),
            Self::AiVideo => serializer.serialize_str("AI_VIDEO"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SceneGenerationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "AI_IMAGE" => Ok(Self::AiImage),
            "AI_VIDEO" => Ok(Self::AiVideo),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SceneGenerationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AiImage => write!(f, "AI_IMAGE"),
            Self::AiVideo => write!(f, "AI_VIDEO"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
