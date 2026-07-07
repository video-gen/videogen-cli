pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What is needed to resolve the error. Present when the error can be fixed by fulfilling a specific requirement (e.g. purchasing an add-on).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApiErrorRequirement {
    /// Requirement type (e.g. `purchase_add_on`, `upgrade_plan`).
    #[serde(default)]
    pub r#type: String,
    /// Key-value pairs with requirement-specific context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, String>>,
}

impl ApiErrorRequirement {
    pub fn builder() -> ApiErrorRequirementBuilder {
        <ApiErrorRequirementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiErrorRequirementBuilder {
    r#type: Option<String>,
    details: Option<HashMap<String, String>>,
}

impl ApiErrorRequirementBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn details(mut self, value: HashMap<String, String>) -> Self {
        self.details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiErrorRequirement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ApiErrorRequirementBuilder::r#type)
    pub fn build(self) -> Result<ApiErrorRequirement, BuildError> {
        Ok(ApiErrorRequirement {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            details: self.details,
        })
    }
}
