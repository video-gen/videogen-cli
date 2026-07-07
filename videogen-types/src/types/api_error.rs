pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApiError {
    /// Human-readable error description.
    #[serde(default)]
    pub message: String,
    /// Machine-readable error code in snake_case (e.g. `invalid_api_key`, `insufficient_credits`). Not always present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// What is needed to resolve the error. Present when the error can be fixed by fulfilling a specific requirement (e.g. purchasing an add-on).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<ApiErrorRequirement>,
    /// Opaque internal error code for debugging. Include this when contacting support.
    #[serde(rename = "internalErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_error_code: Option<String>,
}

impl ApiError {
    pub fn builder() -> ApiErrorBuilder {
        <ApiErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiErrorBuilder {
    message: Option<String>,
    code: Option<String>,
    requirement: Option<ApiErrorRequirement>,
    internal_error_code: Option<String>,
}

impl ApiErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn requirement(mut self, value: ApiErrorRequirement) -> Self {
        self.requirement = Some(value);
        self
    }

    pub fn internal_error_code(mut self, value: impl Into<String>) -> Self {
        self.internal_error_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApiError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ApiErrorBuilder::message)
    pub fn build(self) -> Result<ApiError, BuildError> {
        Ok(ApiError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            code: self.code,
            requirement: self.requirement,
            internal_error_code: self.internal_error_code,
        })
    }
}
