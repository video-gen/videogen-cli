pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEntityRequest {
    /// New display name. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New description. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateEntityRequest {
    pub fn builder() -> UpdateEntityRequestBuilder {
        <UpdateEntityRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEntityRequestBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl UpdateEntityRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEntityRequest`].
    pub fn build(self) -> Result<UpdateEntityRequest, BuildError> {
        Ok(UpdateEntityRequest {
            name: self.name,
            description: self.description,
        })
    }
}

