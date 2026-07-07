pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Editing strategy. `MINOR_EDIT` (default) makes targeted changes while keeping the structure. `RETHINK` re-storyboards the video: it selects which scenes to keep and their order, then edits each in parallel.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemixActionEditWithAgentMode {
    MinorEdit,
    Rethink,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RemixActionEditWithAgentMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MinorEdit => serializer.serialize_str("MINOR_EDIT"),
            Self::Rethink => serializer.serialize_str("RETHINK"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RemixActionEditWithAgentMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "MINOR_EDIT" => Ok(Self::MinorEdit),
            "RETHINK" => Ok(Self::Rethink),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RemixActionEditWithAgentMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MinorEdit => write!(f, "MINOR_EDIT"),
            Self::Rethink => write!(f, "RETHINK"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
