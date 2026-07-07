pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Metadata for a generated file. Obtain ids from tool results or `GET /v1/files`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageFile {
    /// File id (e.g. `vg_file_...`).
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    /// File type. Null when the file is still being processed and the type has not yet been determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<StorageFileType>,
    /// File scope.
    ///
    /// - `GLOBAL`: user-uploaded or standalone generated files that persist indefinitely.
    /// - `PROJECT`: project-specific files (e.g. text-to-speech clips in a generated project).
    /// - `EXPORT`: project exports.
    /// - `TEMPORARY`: short-lived files guaranteed to be available for 24 hours, after which they may be archived at any time. Not analyzed (no description, transcript, or embedding).
    /// - `ENTITY`: files attached to a reusable entity (e.g. a voice sample for an actor), shared across your team.
    pub scope: StorageFileScope,
    /// Display name for the file.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Duration in seconds for video and audio files. Null for images.
    #[serde(rename = "durationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<f64>,
    /// Transcript text for video and audio files, when available. Null for images or when no transcript has been generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// Thumbnail image source. Populated after hydration.
    #[serde(rename = "thumbnailSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_source: Option<FileSource>,
    /// Preview rendition source (720p for video, resized for images). Populated after hydration.
    #[serde(rename = "previewSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_source: Option<FileSource>,
    /// Highest-quality downloadable rendition. Populated after hydration.
    #[serde(rename = "downloadSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_source: Option<FileSource>,
    /// Private HLS streaming source. Populated for video and audio files once streaming renditions are ready. Uses a signed token; treat like other signed sources.
    #[serde(rename = "hlsSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_source: Option<FileSource>,
    /// Whether public preview is enabled for this file. When true, `staticPublicPreviewSource` is populated for all file types. For video and audio, `publicHlsUrl` and `publicPlaybackId` are also populated once embed streaming is ready.
    #[serde(rename = "isPublicPreviewEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public_preview_enabled: Option<bool>,
    /// Permanent public URL for the file's highest-quality rendition. Populated when `isPublicPreviewEnabled` is true. Does not expire (`expiresAt` is null). Use for direct links to images, downloads, or any file type. For embedded video or audio players, prefer `publicPlaybackId`.
    #[serde(rename = "staticPublicPreviewSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_public_preview_source: Option<FileSource>,
    /// Public HLS streaming URL for video and audio. Only present when `isPublicPreviewEnabled` is true and embed streaming is ready. Prefer `publicPlaybackId` with `@videogen/player` for embeds.
    #[serde(rename = "publicHlsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_hls_url: Option<String>,
    /// Encoded public playback id (e.g. `vg_play_...`) for video and audio embeds. Pass this to `@videogen/player` or `@videogen/player-react`. Only present when `isPublicPreviewEnabled` is true and embed streaming is ready. For a permanent direct file URL (any type), use `staticPublicPreviewSource` instead.
    #[serde(rename = "publicPlaybackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_playback_id: Option<String>,
    /// Tool type that generated this file (e.g. `GENERATE_IMAGE`, `TEXT_TO_SPEECH`). Only present when the file was created by a tool execution.
    #[serde(rename = "sourceToolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_tool_type: Option<String>,
    /// Execution id of the tool call that generated this file (e.g. `vg_tool_...`). Only present when the file was created by a tool execution.
    #[serde(rename = "sourceToolExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_tool_execution_id: Option<String>,
    /// Background analysis state for the file (used to populate `description`, `transcript`, `durationSeconds`, and the search embedding). Omitted when the file was returned via a path that does not check analysis progress (e.g. tool-result inline files and webhook payloads).
    #[serde(rename = "fileAnalysisMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_analysis_metadata: Option<FileAnalysisMetadata>,
}

impl StorageFile {
    pub fn builder() -> StorageFileBuilder {
        <StorageFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StorageFileBuilder {
    file_id: Option<String>,
    r#type: Option<StorageFileType>,
    scope: Option<StorageFileScope>,
    display_name: Option<String>,
    description: Option<String>,
    duration_seconds: Option<f64>,
    transcript: Option<String>,
    thumbnail_source: Option<FileSource>,
    preview_source: Option<FileSource>,
    download_source: Option<FileSource>,
    hls_source: Option<FileSource>,
    is_public_preview_enabled: Option<bool>,
    static_public_preview_source: Option<FileSource>,
    public_hls_url: Option<String>,
    public_playback_id: Option<String>,
    source_tool_type: Option<String>,
    source_tool_execution_id: Option<String>,
    file_analysis_metadata: Option<FileAnalysisMetadata>,
}

impl StorageFileBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: StorageFileType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn scope(mut self, value: StorageFileScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn duration_seconds(mut self, value: f64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn transcript(mut self, value: impl Into<String>) -> Self {
        self.transcript = Some(value.into());
        self
    }

    pub fn thumbnail_source(mut self, value: FileSource) -> Self {
        self.thumbnail_source = Some(value);
        self
    }

    pub fn preview_source(mut self, value: FileSource) -> Self {
        self.preview_source = Some(value);
        self
    }

    pub fn download_source(mut self, value: FileSource) -> Self {
        self.download_source = Some(value);
        self
    }

    pub fn hls_source(mut self, value: FileSource) -> Self {
        self.hls_source = Some(value);
        self
    }

    pub fn is_public_preview_enabled(mut self, value: bool) -> Self {
        self.is_public_preview_enabled = Some(value);
        self
    }

    pub fn static_public_preview_source(mut self, value: FileSource) -> Self {
        self.static_public_preview_source = Some(value);
        self
    }

    pub fn public_hls_url(mut self, value: impl Into<String>) -> Self {
        self.public_hls_url = Some(value.into());
        self
    }

    pub fn public_playback_id(mut self, value: impl Into<String>) -> Self {
        self.public_playback_id = Some(value.into());
        self
    }

    pub fn source_tool_type(mut self, value: impl Into<String>) -> Self {
        self.source_tool_type = Some(value.into());
        self
    }

    pub fn source_tool_execution_id(mut self, value: impl Into<String>) -> Self {
        self.source_tool_execution_id = Some(value.into());
        self
    }

    pub fn file_analysis_metadata(mut self, value: FileAnalysisMetadata) -> Self {
        self.file_analysis_metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StorageFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](StorageFileBuilder::file_id)
    /// - [`scope`](StorageFileBuilder::scope)
    pub fn build(self) -> Result<StorageFile, BuildError> {
        Ok(StorageFile {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            r#type: self.r#type,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            display_name: self.display_name,
            description: self.description,
            duration_seconds: self.duration_seconds,
            transcript: self.transcript,
            thumbnail_source: self.thumbnail_source,
            preview_source: self.preview_source,
            download_source: self.download_source,
            hls_source: self.hls_source,
            is_public_preview_enabled: self.is_public_preview_enabled,
            static_public_preview_source: self.static_public_preview_source,
            public_hls_url: self.public_hls_url,
            public_playback_id: self.public_playback_id,
            source_tool_type: self.source_tool_type,
            source_tool_execution_id: self.source_tool_execution_id,
            file_analysis_metadata: self.file_analysis_metadata,
        })
    }
}
