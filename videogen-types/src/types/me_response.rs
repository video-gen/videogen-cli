pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeResponse {
    /// The id of the API key used to authenticate this request.
    #[serde(rename = "apiKeyId")]
    #[serde(default)]
    pub api_key_id: String,
    /// The nickname given to the API key when it was created.
    #[serde(rename = "apiKeyNickname")]
    #[serde(default)]
    pub api_key_nickname: String,
    /// The email address of the account the API key belongs to.
    #[serde(default)]
    pub email: String,
    /// The display name of the account the API key belongs to. `null` if the account has not set one.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The id of the team the API key belongs to.
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
}

impl MeResponse {
    pub fn builder() -> MeResponseBuilder {
        <MeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeResponseBuilder {
    api_key_id: Option<String>,
    api_key_nickname: Option<String>,
    email: Option<String>,
    display_name: Option<String>,
    team_id: Option<String>,
}

impl MeResponseBuilder {
    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    pub fn api_key_nickname(mut self, value: impl Into<String>) -> Self {
        self.api_key_nickname = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn team_id(mut self, value: impl Into<String>) -> Self {
        self.team_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](MeResponseBuilder::api_key_id)
    /// - [`api_key_nickname`](MeResponseBuilder::api_key_nickname)
    /// - [`email`](MeResponseBuilder::email)
    /// - [`team_id`](MeResponseBuilder::team_id)
    pub fn build(self) -> Result<MeResponse, BuildError> {
        Ok(MeResponse {
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
            api_key_nickname: self.api_key_nickname.ok_or_else(|| BuildError::missing_field("api_key_nickname"))?,
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            display_name: self.display_name,
            team_id: self.team_id.ok_or_else(|| BuildError::missing_field("team_id"))?,
        })
    }
}
