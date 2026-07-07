pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Generation quality tier for every scene. LOW is fastest and cheapest; STANDARD balances quality and cost; HIGH is highest quality. Defaults to STANDARD. LOW is not supported for AI_VIDEO scenes: the request is rejected if any scene is generated as a video at LOW quality.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StoryboardToVideoRequestQuality {
    Low,
    Standard,
    High,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StoryboardToVideoRequestQuality {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Low => serializer.serialize_str("LOW"),
            Self::Standard => serializer.serialize_str("STANDARD"),
            Self::High => serializer.serialize_str("HIGH"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StoryboardToVideoRequestQuality {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "LOW" => Ok(Self::Low),
            "STANDARD" => Ok(Self::Standard),
            "HIGH" => Ok(Self::High),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StoryboardToVideoRequestQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "LOW"),
            Self::Standard => write!(f, "STANDARD"),
            Self::High => write!(f, "HIGH"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
