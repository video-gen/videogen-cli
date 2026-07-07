pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Background analysis state for a file. Background analysis populates `description`, `transcript`, `durationSeconds`, and the search embedding after a file is uploaded or generated; this object lets you render a progress indicator while it runs (and skip rendering once it's done).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileAnalysisMetadata {
    /// Coarse-grained analysis state.
    ///
    /// - `UNATTEMPTED`: analysis has not started yet.
    /// - `LOADING`: analysis is in progress.
    /// - `FULFILLED`: analysis completed successfully. `description`, `transcript`, and `durationSeconds` are now populated where applicable for the file's type.
    /// - `REJECTED`: analysis failed permanently and will not be retried.
    #[serde(rename = "analysisLoadingState")]
    pub analysis_loading_state: FileAnalysisMetadataAnalysisLoadingState,
    /// Progress in `[0, 100]`. Always `100` when `analysisLoadingState` is `FULFILLED`. Otherwise the most recent in-flight progress reported by the analysis task (or `0` if no progress has been reported yet).
    #[serde(rename = "analysisProgressPercentage")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub analysis_progress_percentage: f64,
    /// Zero-based index of the current analysis task attempt. Only present while analysis is still loading (`UNATTEMPTED` or `LOADING`); omitted once analysis reaches a terminal state.
    #[serde(rename = "analysisAttemptIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_attempt_index: Option<i64>,
}

impl FileAnalysisMetadata {
    pub fn builder() -> FileAnalysisMetadataBuilder {
        <FileAnalysisMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileAnalysisMetadataBuilder {
    analysis_loading_state: Option<FileAnalysisMetadataAnalysisLoadingState>,
    analysis_progress_percentage: Option<f64>,
    analysis_attempt_index: Option<i64>,
}

impl FileAnalysisMetadataBuilder {
    pub fn analysis_loading_state(mut self, value: FileAnalysisMetadataAnalysisLoadingState) -> Self {
        self.analysis_loading_state = Some(value);
        self
    }

    pub fn analysis_progress_percentage(mut self, value: f64) -> Self {
        self.analysis_progress_percentage = Some(value);
        self
    }

    pub fn analysis_attempt_index(mut self, value: i64) -> Self {
        self.analysis_attempt_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileAnalysisMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`analysis_loading_state`](FileAnalysisMetadataBuilder::analysis_loading_state)
    /// - [`analysis_progress_percentage`](FileAnalysisMetadataBuilder::analysis_progress_percentage)
    pub fn build(self) -> Result<FileAnalysisMetadata, BuildError> {
        Ok(FileAnalysisMetadata {
            analysis_loading_state: self.analysis_loading_state.ok_or_else(|| BuildError::missing_field("analysis_loading_state"))?,
            analysis_progress_percentage: self.analysis_progress_percentage.ok_or_else(|| BuildError::missing_field("analysis_progress_percentage"))?,
            analysis_attempt_index: self.analysis_attempt_index,
        })
    }
}
