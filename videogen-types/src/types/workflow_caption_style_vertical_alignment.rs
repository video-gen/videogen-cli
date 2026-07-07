pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Vertical position of the caption block in the frame.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkflowCaptionStyleVerticalAlignment {
    Top,
    Middle,
    Bottom,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkflowCaptionStyleVerticalAlignment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Top => serializer.serialize_str("TOP"),
            Self::Middle => serializer.serialize_str("MIDDLE"),
            Self::Bottom => serializer.serialize_str("BOTTOM"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkflowCaptionStyleVerticalAlignment {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TOP" => Ok(Self::Top),
            "MIDDLE" => Ok(Self::Middle),
            "BOTTOM" => Ok(Self::Bottom),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkflowCaptionStyleVerticalAlignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Top => write!(f, "TOP"),
            Self::Middle => write!(f, "MIDDLE"),
            Self::Bottom => write!(f, "BOTTOM"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
