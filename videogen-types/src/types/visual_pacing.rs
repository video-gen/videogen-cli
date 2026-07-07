pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How quickly visuals change. FAST shows more, shorter shots; SLOW holds each visual longer. Defaults to MEDIUM.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualPacing {
    Fast,
    Medium,
    Slow,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VisualPacing {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fast => serializer.serialize_str("FAST"),
            Self::Medium => serializer.serialize_str("MEDIUM"),
            Self::Slow => serializer.serialize_str("SLOW"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VisualPacing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "FAST" => Ok(Self::Fast),
            "MEDIUM" => Ok(Self::Medium),
            "SLOW" => Ok(Self::Slow),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VisualPacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fast => write!(f, "FAST"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::Slow => write!(f, "SLOW"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
