use serde::Serialize;
use url::Url;
use crate::api::call_event::{Team, UserDetails};

#[derive(Serialize, Clone)]
pub(crate) struct UserState {
    #[serde(skip_serializing)]
    pub(crate) token: Option<String>,
    pub(crate) user_details: Option<UserDetails>,
    pub(crate) teams: Option<Vec<Team>>,
}

impl Default for UserState {
    fn default() -> Self {
        Self {
            token: None,
            user_details: None,
            teams: None,
        }
    }
}

#[derive(Serialize, Clone)]
pub(crate) struct ServerState {
    #[serde(skip_serializing)]
    pub(crate) current: Option<Url>,
    pub(crate) urls: Vec<Url>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            current: Url::parse("http://localhost:8065").ok(), // TODO add dev env
            urls: vec![],
        }
    }
}