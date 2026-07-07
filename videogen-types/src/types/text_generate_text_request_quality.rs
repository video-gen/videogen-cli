pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Generation quality tier. `LOW` is fastest and cheapest; `STANDARD` balances quality and cost; `HIGH` is higher quality; `MAX` is highest quality. Defaults to `STANDARD`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenerateTextRequestQuality {
    Low,
    Standard,
    High,
    Max,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GenerateTextRequestQuality {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Low => serializer.serialize_str("LOW"),
            Self::Standard => serializer.serialize_str("STANDARD"),
            Self::High => serializer.serialize_str("HIGH"),
            Self::Max => serializer.serialize_str("MAX"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GenerateTextRequestQuality {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "LOW" => Ok(Self::Low),
            "STANDARD" => Ok(Self::Standard),
            "HIGH" => Ok(Self::High),
            "MAX" => Ok(Self::Max),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GenerateTextRequestQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "LOW"),
            Self::Standard => write!(f, "STANDARD"),
            Self::High => write!(f, "HIGH"),
            Self::Max => write!(f, "MAX"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
