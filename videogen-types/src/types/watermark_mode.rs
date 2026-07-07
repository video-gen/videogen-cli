pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Controls whether the VideoGen watermark is applied to the output. `AUTO` applies the watermark unless you have the Production API add-on. `VIDEO_GEN` always applies it. `NONE` removes the watermark — requires the Production API add-on; returns an error if you don't have it.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WatermarkMode {
    None,
    VideoGen,
    Auto,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WatermarkMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("NONE"),
            Self::VideoGen => serializer.serialize_str("VIDEO_GEN"),
            Self::Auto => serializer.serialize_str("AUTO"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WatermarkMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NONE" => Ok(Self::None),
            "VIDEO_GEN" => Ok(Self::VideoGen),
            "AUTO" => Ok(Self::Auto),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WatermarkMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::VideoGen => write!(f, "VIDEO_GEN"),
            Self::Auto => write!(f, "AUTO"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
