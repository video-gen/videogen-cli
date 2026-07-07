pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// File type. Null when the file is still being processed and the type has not yet been determined.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StorageFileType {
    Image,
    Video,
    Audio,
    Pdf,
    Slideshow,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StorageFileType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Image => serializer.serialize_str("IMAGE"),
            Self::Video => serializer.serialize_str("VIDEO"),
            Self::Audio => serializer.serialize_str("AUDIO"),
            Self::Pdf => serializer.serialize_str("PDF"),
            Self::Slideshow => serializer.serialize_str("SLIDESHOW"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StorageFileType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "IMAGE" => Ok(Self::Image),
            "VIDEO" => Ok(Self::Video),
            "AUDIO" => Ok(Self::Audio),
            "PDF" => Ok(Self::Pdf),
            "SLIDESHOW" => Ok(Self::Slideshow),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StorageFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Image => write!(f, "IMAGE"),
            Self::Video => write!(f, "VIDEO"),
            Self::Audio => write!(f, "AUDIO"),
            Self::Pdf => write!(f, "PDF"),
            Self::Slideshow => write!(f, "SLIDESHOW"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
