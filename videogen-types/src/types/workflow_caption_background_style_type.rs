pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// RECT draws one rectangle behind the whole line; WRAPPED hugs the text; WORD_BY_WORD draws a box per word.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkflowCaptionBackgroundStyleType {
    Rect,
    Wrapped,
    WordByWord,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkflowCaptionBackgroundStyleType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Rect => serializer.serialize_str("RECT"),
            Self::Wrapped => serializer.serialize_str("WRAPPED"),
            Self::WordByWord => serializer.serialize_str("WORD_BY_WORD"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkflowCaptionBackgroundStyleType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "RECT" => Ok(Self::Rect),
            "WRAPPED" => Ok(Self::Wrapped),
            "WORD_BY_WORD" => Ok(Self::WordByWord),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkflowCaptionBackgroundStyleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rect => write!(f, "RECT"),
            Self::Wrapped => write!(f, "WRAPPED"),
            Self::WordByWord => write!(f, "WORD_BY_WORD"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
