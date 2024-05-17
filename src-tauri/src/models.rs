use nutype::nutype;
use url::Url;

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From,))]
pub struct ServerUrl(Url);

impl ServerUrl {
    pub fn parse(url: &str) -> Result<Self, url::ParseError> {
        Ok(Self::new(Url::parse(url)?))
    }
}

#[nutype(
    derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, TryFrom),
    sanitize(trim),
    validate(not_empty)
)]
pub struct Login(String);

#[nutype(
    derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, TryFrom),
    sanitize(trim),
    validate(not_empty)
)]
pub struct Pass(String);

/// Non-empty, no-white character access token used to communicate with
/// MatterMost server
#[nutype(
    derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, TryFrom),
    sanitize(trim),
    validate(not_empty)
)]
pub struct AccessToken(String);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    pub login: Login,
    pub password: Pass,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerCredentials {
    pub url: ServerUrl,
    pub access_token: AccessToken,
}
