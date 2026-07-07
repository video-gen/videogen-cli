pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Workflow type identifier.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkflowType {
    ScriptToVideo,
    VoiceoverToVideo,
    SlideshowToVideo,
    StoryboardToVideo,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkflowType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ScriptToVideo => serializer.serialize_str("SCRIPT_TO_VIDEO"),
            Self::VoiceoverToVideo => serializer.serialize_str("VOICEOVER_TO_VIDEO"),
            Self::SlideshowToVideo => serializer.serialize_str("SLIDESHOW_TO_VIDEO"),
            Self::StoryboardToVideo => serializer.serialize_str("STORYBOARD_TO_VIDEO"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkflowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SCRIPT_TO_VIDEO" => Ok(Self::ScriptToVideo),
            "VOICEOVER_TO_VIDEO" => Ok(Self::VoiceoverToVideo),
            "SLIDESHOW_TO_VIDEO" => Ok(Self::SlideshowToVideo),
            "STORYBOARD_TO_VIDEO" => Ok(Self::StoryboardToVideo),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScriptToVideo => write!(f, "SCRIPT_TO_VIDEO"),
            Self::VoiceoverToVideo => write!(f, "VOICEOVER_TO_VIDEO"),
            Self::SlideshowToVideo => write!(f, "SLIDESHOW_TO_VIDEO"),
            Self::StoryboardToVideo => write!(f, "STORYBOARD_TO_VIDEO"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
