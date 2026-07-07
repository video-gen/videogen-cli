pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of file to upload. Optional; when omitted, the type is inferred after upload processing completes.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateFileUploadRequestType {
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
impl Serialize for CreateFileUploadRequestType {
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

impl<'de> Deserialize<'de> for CreateFileUploadRequestType {
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

impl fmt::Display for CreateFileUploadRequestType {
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
