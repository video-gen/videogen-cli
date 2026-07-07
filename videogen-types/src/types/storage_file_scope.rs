pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// File scope.
/// 
/// - `GLOBAL`: user-uploaded or standalone generated files that persist indefinitely.
/// - `PROJECT`: project-specific files (e.g. text-to-speech clips in a generated project).
/// - `EXPORT`: project exports.
/// - `TEMPORARY`: short-lived files guaranteed to be available for 24 hours, after which they may be archived at any time. Not analyzed (no description, transcript, or embedding).
/// - `ENTITY`: files attached to a reusable entity (e.g. a voice sample for an actor), shared across your team.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StorageFileScope {
    Global,
    Project,
    Export,
    Temporary,
    Entity,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StorageFileScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Global => serializer.serialize_str("GLOBAL"),
            Self::Project => serializer.serialize_str("PROJECT"),
            Self::Export => serializer.serialize_str("EXPORT"),
            Self::Temporary => serializer.serialize_str("TEMPORARY"),
            Self::Entity => serializer.serialize_str("ENTITY"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StorageFileScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "GLOBAL" => Ok(Self::Global),
            "PROJECT" => Ok(Self::Project),
            "EXPORT" => Ok(Self::Export),
            "TEMPORARY" => Ok(Self::Temporary),
            "ENTITY" => Ok(Self::Entity),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StorageFileScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Global => write!(f, "GLOBAL"),
            Self::Project => write!(f, "PROJECT"),
            Self::Export => write!(f, "EXPORT"),
            Self::Temporary => write!(f, "TEMPORARY"),
            Self::Entity => write!(f, "ENTITY"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
