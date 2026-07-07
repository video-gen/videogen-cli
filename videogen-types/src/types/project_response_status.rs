pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// High-level project status.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectResponseStatus {
    Generating,
    Ready,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProjectResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Generating => serializer.serialize_str("generating"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "generating" => Ok(Self::Generating),
            "ready" => Ok(Self::Ready),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProjectResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generating => write!(f, "generating"),
            Self::Ready => write!(f, "ready"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
