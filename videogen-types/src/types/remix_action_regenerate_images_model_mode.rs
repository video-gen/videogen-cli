pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Image generation quality tier. Defaults to `STANDARD`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemixActionRegenerateImagesModelMode {
    Standard,
    High,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RemixActionRegenerateImagesModelMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Standard => serializer.serialize_str("STANDARD"),
            Self::High => serializer.serialize_str("HIGH"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RemixActionRegenerateImagesModelMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "STANDARD" => Ok(Self::Standard),
            "HIGH" => Ok(Self::High),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RemixActionRegenerateImagesModelMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "STANDARD"),
            Self::High => write!(f, "HIGH"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
