pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Coarse-grained analysis state.
/// 
/// - `UNATTEMPTED`: analysis has not started yet.
/// - `LOADING`: analysis is in progress.
/// - `FULFILLED`: analysis completed successfully. `description`, `transcript`, and `durationSeconds` are now populated where applicable for the file's type.
/// - `REJECTED`: analysis failed permanently and will not be retried.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileAnalysisMetadataAnalysisLoadingState {
    Unattempted,
    Loading,
    Fulfilled,
    Rejected,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FileAnalysisMetadataAnalysisLoadingState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unattempted => serializer.serialize_str("UNATTEMPTED"),
            Self::Loading => serializer.serialize_str("LOADING"),
            Self::Fulfilled => serializer.serialize_str("FULFILLED"),
            Self::Rejected => serializer.serialize_str("REJECTED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FileAnalysisMetadataAnalysisLoadingState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "UNATTEMPTED" => Ok(Self::Unattempted),
            "LOADING" => Ok(Self::Loading),
            "FULFILLED" => Ok(Self::Fulfilled),
            "REJECTED" => Ok(Self::Rejected),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FileAnalysisMetadataAnalysisLoadingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unattempted => write!(f, "UNATTEMPTED"),
            Self::Loading => write!(f, "LOADING"),
            Self::Fulfilled => write!(f, "FULFILLED"),
            Self::Rejected => write!(f, "REJECTED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
