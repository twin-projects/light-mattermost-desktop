use serde::{Deserialize, Serialize};

pub enum ApiEvent {
    LoginEvent(String, String),
    MyTeams,
}

pub enum Response {
    LoginResponse(
        String, // token
        String, // user_id
        String, // user name
    ),
    MyTeams(
        Vec<Team>, // teams
    ),
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub login_id: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timezone {
    #[serde(rename(serialize = "automaticTimezone", deserialize = "automaticTimezone"))]
    pub automatic_timezone: String,
    #[serde(rename(serialize = "manualTimezone", deserialize = "manualTimezone"))]
    pub manual_timezone: String,
    #[serde(rename(
        serialize = "useAutomaticTimezone",
        deserialize = "useAutomaticTimezone"
    ))]
    pub use_automatic_timezone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub auth_data: String,
    pub auth_service: String,
    pub email: String,
    pub nickname: String,
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub roles: String,
}

#[derive(Serialize, Clone)]
pub struct UserDetails {
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Team {
    pub id: String,
    pub display_name: String,
    pub name: String,
    pub description: String,
    pub email: String,
    pub company_name: String,
}
