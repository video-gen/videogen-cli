pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// `pending`: asset is still processing or has not been hydrated yet. `ready`: signed URL is available. `failed`: rendition generation failed. `skipped`: rendition does not apply to this file type (e.g. thumbnail for audio).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileSourceStatus {
    Pending,
    Ready,
    Failed,
    Skipped,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FileSourceStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Skipped => serializer.serialize_str("skipped"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FileSourceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "ready" => Ok(Self::Ready),
            "failed" => Ok(Self::Failed),
            "skipped" => Ok(Self::Skipped),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FileSourceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Ready => write!(f, "ready"),
            Self::Failed => write!(f, "failed"),
            Self::Skipped => write!(f, "skipped"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
