use models::{AccessToken, *};
use serde::Serialize;
use url::Url;

#[derive(Serialize, Clone, Default)]
pub(crate) struct UserState {
    #[serde(skip_serializing)]
    pub(crate) id: Option<UserId>,
    pub(crate) token: Option<AccessToken>,
    pub(crate) user_details: Option<UserDetails>,
    pub(crate) teams: Option<Vec<Team>>,
    pub(crate) team_members: Option<Vec<TeamMember>>,
    pub(crate) channels: Option<Vec<Channel>>,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct Server {
    pub(crate) name: String,
    pub(crate) url: Url,
}

#[derive(Serialize, Clone)]
pub(crate) struct ServerState {
    #[serde(skip_serializing)]
    pub(crate) current: Option<Server>,
    pub(crate) servers: Vec<Server>,
}

impl Default for ServerState {
    fn default() -> Self {
        let current = Some(Server {
            name: "localhost".to_owned(),
            url: Url::parse("http://localhost:8065").ok().unwrap(),
        });
        Self {
            current: current.to_owned(), // TODO add dev env
            servers: vec![
                current.unwrap(),
                Server {
                    name: "ITA".to_string(),
                    url: Url::parse("https://mm.ita-prog.pl").unwrap(),
                },
            ],
        }
    }
}
