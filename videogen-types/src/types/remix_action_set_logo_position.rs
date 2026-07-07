pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Position the logo is anchored to. Omit or pass `null` to keep the current position.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemixActionSetLogoPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RemixActionSetLogoPosition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TopLeft => serializer.serialize_str("TOP_LEFT"),
            Self::TopCenter => serializer.serialize_str("TOP_CENTER"),
            Self::TopRight => serializer.serialize_str("TOP_RIGHT"),
            Self::BottomLeft => serializer.serialize_str("BOTTOM_LEFT"),
            Self::BottomCenter => serializer.serialize_str("BOTTOM_CENTER"),
            Self::BottomRight => serializer.serialize_str("BOTTOM_RIGHT"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RemixActionSetLogoPosition {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TOP_LEFT" => Ok(Self::TopLeft),
            "TOP_CENTER" => Ok(Self::TopCenter),
            "TOP_RIGHT" => Ok(Self::TopRight),
            "BOTTOM_LEFT" => Ok(Self::BottomLeft),
            "BOTTOM_CENTER" => Ok(Self::BottomCenter),
            "BOTTOM_RIGHT" => Ok(Self::BottomRight),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RemixActionSetLogoPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TopLeft => write!(f, "TOP_LEFT"),
            Self::TopCenter => write!(f, "TOP_CENTER"),
            Self::TopRight => write!(f, "TOP_RIGHT"),
            Self::BottomLeft => write!(f, "BOTTOM_LEFT"),
            Self::BottomCenter => write!(f, "BOTTOM_CENTER"),
            Self::BottomRight => write!(f, "BOTTOM_RIGHT"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
