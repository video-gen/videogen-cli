pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The kind of edit a remix action applies.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemixActionType {
    SetBackgroundMusic,
    SetLogo,
    EnableCaptions,
    DisableCaptions,
    AddTransitions,
    ResizeProject,
    CleanUpTranscript,
    ConvertImagesToVideos,
    RegenerateImages,
    UpscaleAssets,
    ChangeNarrator,
    ShuffleStockVisuals,
    GenerateMusic,
    TranslateProject,
    EditWithAgent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RemixActionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SetBackgroundMusic => serializer.serialize_str("SET_BACKGROUND_MUSIC"),
            Self::SetLogo => serializer.serialize_str("SET_LOGO"),
            Self::EnableCaptions => serializer.serialize_str("ENABLE_CAPTIONS"),
            Self::DisableCaptions => serializer.serialize_str("DISABLE_CAPTIONS"),
            Self::AddTransitions => serializer.serialize_str("ADD_TRANSITIONS"),
            Self::ResizeProject => serializer.serialize_str("RESIZE_PROJECT"),
            Self::CleanUpTranscript => serializer.serialize_str("CLEAN_UP_TRANSCRIPT"),
            Self::ConvertImagesToVideos => serializer.serialize_str("CONVERT_IMAGES_TO_VIDEOS"),
            Self::RegenerateImages => serializer.serialize_str("REGENERATE_IMAGES"),
            Self::UpscaleAssets => serializer.serialize_str("UPSCALE_ASSETS"),
            Self::ChangeNarrator => serializer.serialize_str("CHANGE_NARRATOR"),
            Self::ShuffleStockVisuals => serializer.serialize_str("SHUFFLE_STOCK_VISUALS"),
            Self::GenerateMusic => serializer.serialize_str("GENERATE_MUSIC"),
            Self::TranslateProject => serializer.serialize_str("TRANSLATE_PROJECT"),
            Self::EditWithAgent => serializer.serialize_str("EDIT_WITH_AGENT"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RemixActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SET_BACKGROUND_MUSIC" => Ok(Self::SetBackgroundMusic),
            "SET_LOGO" => Ok(Self::SetLogo),
            "ENABLE_CAPTIONS" => Ok(Self::EnableCaptions),
            "DISABLE_CAPTIONS" => Ok(Self::DisableCaptions),
            "ADD_TRANSITIONS" => Ok(Self::AddTransitions),
            "RESIZE_PROJECT" => Ok(Self::ResizeProject),
            "CLEAN_UP_TRANSCRIPT" => Ok(Self::CleanUpTranscript),
            "CONVERT_IMAGES_TO_VIDEOS" => Ok(Self::ConvertImagesToVideos),
            "REGENERATE_IMAGES" => Ok(Self::RegenerateImages),
            "UPSCALE_ASSETS" => Ok(Self::UpscaleAssets),
            "CHANGE_NARRATOR" => Ok(Self::ChangeNarrator),
            "SHUFFLE_STOCK_VISUALS" => Ok(Self::ShuffleStockVisuals),
            "GENERATE_MUSIC" => Ok(Self::GenerateMusic),
            "TRANSLATE_PROJECT" => Ok(Self::TranslateProject),
            "EDIT_WITH_AGENT" => Ok(Self::EditWithAgent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RemixActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SetBackgroundMusic => write!(f, "SET_BACKGROUND_MUSIC"),
            Self::SetLogo => write!(f, "SET_LOGO"),
            Self::EnableCaptions => write!(f, "ENABLE_CAPTIONS"),
            Self::DisableCaptions => write!(f, "DISABLE_CAPTIONS"),
            Self::AddTransitions => write!(f, "ADD_TRANSITIONS"),
            Self::ResizeProject => write!(f, "RESIZE_PROJECT"),
            Self::CleanUpTranscript => write!(f, "CLEAN_UP_TRANSCRIPT"),
            Self::ConvertImagesToVideos => write!(f, "CONVERT_IMAGES_TO_VIDEOS"),
            Self::RegenerateImages => write!(f, "REGENERATE_IMAGES"),
            Self::UpscaleAssets => write!(f, "UPSCALE_ASSETS"),
            Self::ChangeNarrator => write!(f, "CHANGE_NARRATOR"),
            Self::ShuffleStockVisuals => write!(f, "SHUFFLE_STOCK_VISUALS"),
            Self::GenerateMusic => write!(f, "GENERATE_MUSIC"),
            Self::TranslateProject => write!(f, "TRANSLATE_PROJECT"),
            Self::EditWithAgent => write!(f, "EDIT_WITH_AGENT"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
