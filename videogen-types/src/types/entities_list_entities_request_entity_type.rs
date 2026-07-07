pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListEntitiesRequestEntityType {
    Actor,
    Product,
    VisualStyle,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListEntitiesRequestEntityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Actor => serializer.serialize_str("ACTOR"),
            Self::Product => serializer.serialize_str("PRODUCT"),
            Self::VisualStyle => serializer.serialize_str("VISUAL_STYLE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListEntitiesRequestEntityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ACTOR" => Ok(Self::Actor),
            "PRODUCT" => Ok(Self::Product),
            "VISUAL_STYLE" => Ok(Self::VisualStyle),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListEntitiesRequestEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Actor => write!(f, "ACTOR"),
            Self::Product => write!(f, "PRODUCT"),
            Self::VisualStyle => write!(f, "VISUAL_STYLE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
