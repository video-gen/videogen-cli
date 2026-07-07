pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListRemixActionsResponse {
    /// Remix actions for the project, most recent first.
    #[serde(rename = "remixActions")]
    #[serde(default)]
    pub remix_actions: Vec<RemixActionRun>,
}

impl ListRemixActionsResponse {
    pub fn builder() -> ListRemixActionsResponseBuilder {
        <ListRemixActionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRemixActionsResponseBuilder {
    remix_actions: Option<Vec<RemixActionRun>>,
}

impl ListRemixActionsResponseBuilder {
    pub fn remix_actions(mut self, value: Vec<RemixActionRun>) -> Self {
        self.remix_actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRemixActionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`remix_actions`](ListRemixActionsResponseBuilder::remix_actions)
    pub fn build(self) -> Result<ListRemixActionsResponse, BuildError> {
        Ok(ListRemixActionsResponse {
            remix_actions: self.remix_actions.ok_or_else(|| BuildError::missing_field("remix_actions"))?,
        })
    }
}
