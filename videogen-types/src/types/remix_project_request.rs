pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RemixProjectRequest {
    /// Ordered list of edits to apply. Each runs asynchronously as its own remix action.
    #[serde(rename = "remixActions")]
    #[serde(default)]
    pub remix_actions: Vec<RemixAction>,
    /// When true, the project is duplicated first and the edits are applied to the copy, leaving the original untouched. The response's `projectId` is the copy. Defaults to false (edits the project in place).
    #[serde(rename = "saveAsNewProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_as_new_project: Option<bool>,
}

impl RemixProjectRequest {
    pub fn builder() -> RemixProjectRequestBuilder {
        <RemixProjectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemixProjectRequestBuilder {
    remix_actions: Option<Vec<RemixAction>>,
    save_as_new_project: Option<bool>,
}

impl RemixProjectRequestBuilder {
    pub fn remix_actions(mut self, value: Vec<RemixAction>) -> Self {
        self.remix_actions = Some(value);
        self
    }

    pub fn save_as_new_project(mut self, value: bool) -> Self {
        self.save_as_new_project = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemixProjectRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`remix_actions`](RemixProjectRequestBuilder::remix_actions)
    pub fn build(self) -> Result<RemixProjectRequest, BuildError> {
        Ok(RemixProjectRequest {
            remix_actions: self.remix_actions.ok_or_else(|| BuildError::missing_field("remix_actions"))?,
            save_as_new_project: self.save_as_new_project,
        })
    }
}

