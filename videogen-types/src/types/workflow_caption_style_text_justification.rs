pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkflowCaptionStyleTextJustification {
    Left,
    Center,
    Right,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkflowCaptionStyleTextJustification {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Left => serializer.serialize_str("LEFT"),
            Self::Center => serializer.serialize_str("CENTER"),
            Self::Right => serializer.serialize_str("RIGHT"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkflowCaptionStyleTextJustification {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "LEFT" => Ok(Self::Left),
            "CENTER" => Ok(Self::Center),
            "RIGHT" => Ok(Self::Right),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkflowCaptionStyleTextJustification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Left => write!(f, "LEFT"),
            Self::Center => write!(f, "CENTER"),
            Self::Right => write!(f, "RIGHT"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
