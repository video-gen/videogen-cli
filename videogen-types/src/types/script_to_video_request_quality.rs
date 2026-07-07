pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Image generation quality tier for AI-generated visuals. LOW is fastest and cheapest; STANDARD balances quality and cost; HIGH is highest quality. Only applies when `visualStyle.type` is AI_IMAGE or ENTITY; STOCK pulls existing footage and is unaffected. Defaults to STANDARD.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScriptToVideoRequestQuality {
    Low,
    Standard,
    High,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScriptToVideoRequestQuality {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Low => serializer.serialize_str("LOW"),
            Self::Standard => serializer.serialize_str("STANDARD"),
            Self::High => serializer.serialize_str("HIGH"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScriptToVideoRequestQuality {
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

impl fmt::Display for ScriptToVideoRequestQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "LOW"),
            Self::Standard => write!(f, "STANDARD"),
            Self::High => write!(f, "HIGH"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
