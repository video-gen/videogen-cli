pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RemixAction {
        #[serde(rename = "SET_BACKGROUND_MUSIC")]
        #[non_exhaustive]
        SetBackgroundMusic {
            #[serde(rename = "fileId")]
            #[serde(skip_serializing_if = "Option::is_none")]
            file_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            volume: Option<f64>,
        },

        #[serde(rename = "SET_LOGO")]
        #[non_exhaustive]
        SetLogo {
            #[serde(rename = "fileId")]
            #[serde(skip_serializing_if = "Option::is_none")]
            file_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            position: Option<RemixActionSetLogoPosition>,
            #[serde(rename = "sizePercent")]
            #[serde(skip_serializing_if = "Option::is_none")]
            size_percent: Option<f64>,
        },

        #[serde(rename = "ENABLE_CAPTIONS")]
        #[non_exhaustive]
        EnableCaptions {
            #[serde(rename = "captionStyle")]
            #[serde(skip_serializing_if = "Option::is_none")]
            caption_style: Option<WorkflowCaptionStyle>,
        },

        #[serde(rename = "DISABLE_CAPTIONS")]
        #[non_exhaustive]
        DisableCaptions {},

        #[serde(rename = "ADD_TRANSITIONS")]
        #[non_exhaustive]
        AddTransitions {
            #[serde(rename = "sectionTransition")]
            #[serde(skip_serializing_if = "Option::is_none")]
            section_transition: Option<RemixTransitionStyle>,
            #[serde(rename = "assetTransition")]
            #[serde(skip_serializing_if = "Option::is_none")]
            asset_transition: Option<RemixTransitionStyle>,
        },

        #[serde(rename = "RESIZE_PROJECT")]
        #[non_exhaustive]
        ResizeProject {
            #[serde(rename = "aspectRatio")]
            #[serde(default)]
            aspect_ratio: AspectRatio,
        },

        #[serde(rename = "CLEAN_UP_TRANSCRIPT")]
        #[non_exhaustive]
        CleanUpTranscript {
            #[serde(rename = "removeFillers")]
            #[serde(skip_serializing_if = "Option::is_none")]
            remove_fillers: Option<bool>,
            #[serde(rename = "removePauses")]
            #[serde(skip_serializing_if = "Option::is_none")]
            remove_pauses: Option<bool>,
            #[serde(rename = "fillerWords")]
            #[serde(skip_serializing_if = "Option::is_none")]
            filler_words: Option<Vec<String>>,
            #[serde(rename = "minPauseSeconds")]
            #[serde(skip_serializing_if = "Option::is_none")]
            min_pause_seconds: Option<f64>,
        },

        #[serde(rename = "CONVERT_IMAGES_TO_VIDEOS")]
        #[non_exhaustive]
        ConvertImagesToVideos {
            #[serde(rename = "motionPrompt")]
            #[serde(skip_serializing_if = "Option::is_none")]
            motion_prompt: Option<String>,
            #[serde(rename = "muteOutputVideos")]
            #[serde(skip_serializing_if = "Option::is_none")]
            mute_output_videos: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            quality: Option<VideoQuality>,
        },

        #[serde(rename = "REGENERATE_IMAGES")]
        #[non_exhaustive]
        RegenerateImages {
            #[serde(rename = "stylePrompt")]
            #[serde(default)]
            style_prompt: String,
            #[serde(rename = "modelMode")]
            #[serde(skip_serializing_if = "Option::is_none")]
            model_mode: Option<RemixActionRegenerateImagesModelMode>,
        },

        #[serde(rename = "UPSCALE_ASSETS")]
        #[non_exhaustive]
        UpscaleAssets {
            #[serde(rename = "includeVideos")]
            #[serde(skip_serializing_if = "Option::is_none")]
            include_videos: Option<bool>,
            #[serde(rename = "includeStockContent")]
            #[serde(skip_serializing_if = "Option::is_none")]
            include_stock_content: Option<bool>,
        },

        #[serde(rename = "CHANGE_NARRATOR")]
        #[non_exhaustive]
        ChangeNarrator {
            #[serde(rename = "voiceId")]
            #[serde(default)]
            voice_id: String,
            #[serde(rename = "avatarPresenterId")]
            #[serde(skip_serializing_if = "Option::is_none")]
            avatar_presenter_id: Option<String>,
            #[serde(rename = "voiceSpeed")]
            #[serde(skip_serializing_if = "Option::is_none")]
            voice_speed: Option<f64>,
        },

        #[serde(rename = "SHUFFLE_STOCK_VISUALS")]
        #[non_exhaustive]
        ShuffleStockVisuals {},

        #[serde(rename = "GENERATE_MUSIC")]
        #[non_exhaustive]
        GenerateMusic {
            #[serde(default)]
            prompt: String,
        },

        #[serde(rename = "TRANSLATE_PROJECT")]
        #[non_exhaustive]
        TranslateProject {
            #[serde(rename = "languageCode")]
            #[serde(default)]
            language_code: String,
            #[serde(rename = "changeVoice")]
            #[serde(skip_serializing_if = "Option::is_none")]
            change_voice: Option<bool>,
            #[serde(rename = "translateImageText")]
            #[serde(skip_serializing_if = "Option::is_none")]
            translate_image_text: Option<bool>,
        },

        #[serde(rename = "EDIT_WITH_AGENT")]
        #[non_exhaustive]
        EditWithAgent {
            #[serde(default)]
            prompt: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            mode: Option<RemixActionEditWithAgentMode>,
            #[serde(rename = "targetDurationSeconds")]
            #[serde(skip_serializing_if = "Option::is_none")]
            target_duration_seconds: Option<f64>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl RemixAction {
    pub fn set_background_music() -> Self {
        Self::SetBackgroundMusic { file_id: None, volume: None }
    }

    pub fn set_logo() -> Self {
        Self::SetLogo { file_id: None, position: None, size_percent: None }
    }

    pub fn enable_captions() -> Self {
        Self::EnableCaptions { caption_style: None }
    }

    pub fn disable_captions() -> Self {
        Self::DisableCaptions {}
    }

    pub fn add_transitions() -> Self {
        Self::AddTransitions { section_transition: None, asset_transition: None }
    }

    pub fn resize_project(aspect_ratio: AspectRatio) -> Self {
        Self::ResizeProject { aspect_ratio }
    }

    pub fn clean_up_transcript() -> Self {
        Self::CleanUpTranscript { remove_fillers: None, remove_pauses: None, filler_words: None, min_pause_seconds: None }
    }

    pub fn convert_images_to_videos() -> Self {
        Self::ConvertImagesToVideos { motion_prompt: None, mute_output_videos: None, quality: None }
    }

    pub fn regenerate_images(style_prompt: String) -> Self {
        Self::RegenerateImages { style_prompt, model_mode: None }
    }

    pub fn upscale_assets() -> Self {
        Self::UpscaleAssets { include_videos: None, include_stock_content: None }
    }

    pub fn change_narrator(voice_id: String) -> Self {
        Self::ChangeNarrator { voice_id, avatar_presenter_id: None, voice_speed: None }
    }

    pub fn shuffle_stock_visuals() -> Self {
        Self::ShuffleStockVisuals {}
    }

    pub fn generate_music(prompt: String) -> Self {
        Self::GenerateMusic { prompt }
    }

    pub fn translate_project(language_code: String) -> Self {
        Self::TranslateProject { language_code, change_voice: None, translate_image_text: None }
    }

    pub fn edit_with_agent(prompt: String) -> Self {
        Self::EditWithAgent { prompt, mode: None, target_duration_seconds: None }
    }

    pub fn set_background_music_with_file_id(file_id: String, volume: Option<f64>) -> Self {
        Self::SetBackgroundMusic { file_id: Some(file_id), volume }
    }

    pub fn set_background_music_with_volume(file_id: Option<String>, volume: f64) -> Self {
        Self::SetBackgroundMusic { file_id, volume: Some(volume) }
    }

    pub fn set_logo_with_file_id(file_id: String, position: Option<RemixActionSetLogoPosition>, size_percent: Option<f64>) -> Self {
        Self::SetLogo { file_id: Some(file_id), position, size_percent }
    }

    pub fn set_logo_with_position(file_id: Option<String>, position: RemixActionSetLogoPosition, size_percent: Option<f64>) -> Self {
        Self::SetLogo { file_id, position: Some(position), size_percent }
    }

    pub fn set_logo_with_size_percent(file_id: Option<String>, position: Option<RemixActionSetLogoPosition>, size_percent: f64) -> Self {
        Self::SetLogo { file_id, position, size_percent: Some(size_percent) }
    }

    pub fn enable_captions_with_caption_style(caption_style: WorkflowCaptionStyle) -> Self {
        Self::EnableCaptions { caption_style: Some(caption_style) }
    }

    pub fn add_transitions_with_section_transition(section_transition: RemixTransitionStyle, asset_transition: Option<RemixTransitionStyle>) -> Self {
        Self::AddTransitions { section_transition: Some(section_transition), asset_transition }
    }

    pub fn add_transitions_with_asset_transition(section_transition: Option<RemixTransitionStyle>, asset_transition: RemixTransitionStyle) -> Self {
        Self::AddTransitions { section_transition, asset_transition: Some(asset_transition) }
    }

    pub fn clean_up_transcript_with_remove_fillers(remove_fillers: bool, remove_pauses: Option<bool>, filler_words: Option<Vec<String>>, min_pause_seconds: Option<f64>) -> Self {
        Self::CleanUpTranscript { remove_fillers: Some(remove_fillers), remove_pauses, filler_words, min_pause_seconds }
    }

    pub fn clean_up_transcript_with_remove_pauses(remove_fillers: Option<bool>, remove_pauses: bool, filler_words: Option<Vec<String>>, min_pause_seconds: Option<f64>) -> Self {
        Self::CleanUpTranscript { remove_fillers, remove_pauses: Some(remove_pauses), filler_words, min_pause_seconds }
    }

    pub fn clean_up_transcript_with_filler_words(remove_fillers: Option<bool>, remove_pauses: Option<bool>, filler_words: Vec<String>, min_pause_seconds: Option<f64>) -> Self {
        Self::CleanUpTranscript { remove_fillers, remove_pauses, filler_words: Some(filler_words), min_pause_seconds }
    }

    pub fn clean_up_transcript_with_min_pause_seconds(remove_fillers: Option<bool>, remove_pauses: Option<bool>, filler_words: Option<Vec<String>>, min_pause_seconds: f64) -> Self {
        Self::CleanUpTranscript { remove_fillers, remove_pauses, filler_words, min_pause_seconds: Some(min_pause_seconds) }
    }

    pub fn convert_images_to_videos_with_motion_prompt(motion_prompt: String, mute_output_videos: Option<bool>, quality: Option<VideoQuality>) -> Self {
        Self::ConvertImagesToVideos { motion_prompt: Some(motion_prompt), mute_output_videos, quality }
    }

    pub fn convert_images_to_videos_with_mute_output_videos(motion_prompt: Option<String>, mute_output_videos: bool, quality: Option<VideoQuality>) -> Self {
        Self::ConvertImagesToVideos { motion_prompt, mute_output_videos: Some(mute_output_videos), quality }
    }

    pub fn convert_images_to_videos_with_quality(motion_prompt: Option<String>, mute_output_videos: Option<bool>, quality: VideoQuality) -> Self {
        Self::ConvertImagesToVideos { motion_prompt, mute_output_videos, quality: Some(quality) }
    }

    pub fn regenerate_images_with_model_mode(style_prompt: String, model_mode: RemixActionRegenerateImagesModelMode) -> Self {
        Self::RegenerateImages { style_prompt, model_mode: Some(model_mode) }
    }

    pub fn upscale_assets_with_include_videos(include_videos: bool, include_stock_content: Option<bool>) -> Self {
        Self::UpscaleAssets { include_videos: Some(include_videos), include_stock_content }
    }

    pub fn upscale_assets_with_include_stock_content(include_videos: Option<bool>, include_stock_content: bool) -> Self {
        Self::UpscaleAssets { include_videos, include_stock_content: Some(include_stock_content) }
    }

    pub fn change_narrator_with_avatar_presenter_id(voice_id: String, avatar_presenter_id: String, voice_speed: Option<f64>) -> Self {
        Self::ChangeNarrator { voice_id, avatar_presenter_id: Some(avatar_presenter_id), voice_speed }
    }

    pub fn change_narrator_with_voice_speed(voice_id: String, avatar_presenter_id: Option<String>, voice_speed: f64) -> Self {
        Self::ChangeNarrator { voice_id, avatar_presenter_id, voice_speed: Some(voice_speed) }
    }

    pub fn translate_project_with_change_voice(language_code: String, change_voice: bool, translate_image_text: Option<bool>) -> Self {
        Self::TranslateProject { language_code, change_voice: Some(change_voice), translate_image_text }
    }

    pub fn translate_project_with_translate_image_text(language_code: String, change_voice: Option<bool>, translate_image_text: bool) -> Self {
        Self::TranslateProject { language_code, change_voice, translate_image_text: Some(translate_image_text) }
    }

    pub fn edit_with_agent_with_mode(prompt: String, mode: RemixActionEditWithAgentMode, target_duration_seconds: Option<f64>) -> Self {
        Self::EditWithAgent { prompt, mode: Some(mode), target_duration_seconds }
    }

    pub fn edit_with_agent_with_target_duration_seconds(prompt: String, mode: Option<RemixActionEditWithAgentMode>, target_duration_seconds: f64) -> Self {
        Self::EditWithAgent { prompt, mode, target_duration_seconds: Some(target_duration_seconds) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
