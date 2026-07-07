pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A transition applied at a boundary. `DYNAMIC` auto-varies the style across boundaries; `NONE` removes transitions; the rest apply that fixed style everywhere.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemixTransitionStyle {
    Dynamic,
    None,
    Fade,
    Rise,
    Pan,
    Pop,
    Wipe,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RemixTransitionStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Dynamic => serializer.serialize_str("DYNAMIC"),
            Self::None => serializer.serialize_str("NONE"),
            Self::Fade => serializer.serialize_str("FADE"),
            Self::Rise => serializer.serialize_str("RISE"),
            Self::Pan => serializer.serialize_str("PAN"),
            Self::Pop => serializer.serialize_str("POP"),
            Self::Wipe => serializer.serialize_str("WIPE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RemixTransitionStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DYNAMIC" => Ok(Self::Dynamic),
            "NONE" => Ok(Self::None),
            "FADE" => Ok(Self::Fade),
            "RISE" => Ok(Self::Rise),
            "PAN" => Ok(Self::Pan),
            "POP" => Ok(Self::Pop),
            "WIPE" => Ok(Self::Wipe),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RemixTransitionStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dynamic => write!(f, "DYNAMIC"),
            Self::None => write!(f, "NONE"),
            Self::Fade => write!(f, "FADE"),
            Self::Rise => write!(f, "RISE"),
            Self::Pan => write!(f, "PAN"),
            Self::Pop => write!(f, "POP"),
            Self::Wipe => write!(f, "WIPE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
