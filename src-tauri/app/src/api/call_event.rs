use std::fmt;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug)]
pub enum ApiEvent {
    LoginEvent(String, String),
    MyTeams,
    MyTeamMembers,
    MyChannels,
}

#[derive(Debug)]
pub enum Response {
    LoginResponse(
        String, // token
        String, // user_id
        String, // user name
    ),
    MyTeams(
        Vec<Team>, // teams
    ),
    MyTeamMembers(
        Vec<TeamMember>, // team members
    ),
    MyChannels(
        Vec<Channel>, // team members
    ),
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Serialize, Clone, Debug)]
pub struct UserDetails {
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Team {
    pub id: Option<String>,
    pub display_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub company_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeamMember {
    pub team_id: String,
    pub user_id: String,
    pub roles: String,
    pub delete_at: i64,
    pub scheme_guest: bool,
    pub scheme_user: bool,
    pub scheme_admin: bool,
    pub explicit_roles: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub id: Option<String>,
    pub create_at: i64,
    pub update_at: i64,
    pub delete_at: i64,
    pub team_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub display_name: Option<String>,
    pub name: Option<String>,
    pub header: Option<String>,
    pub purpose: Option<String>,
    pub last_post_at: i64,
    pub total_msg_count: i64,
    pub extra_update_at: i64,
    pub creator_id: Option<String>,
    pub scheme_id: Option<String>,
    pub props: Option<NotifyProps>,
    pub group_constrained: Option<bool>,
    pub total_msg_count_root: Option<i64>,
    pub last_root_post_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    user_id: String,
    roles: String,
    last_viewed_at: i16,
    msg_count: i16,
    mention_count: i16,
    mention_count_root: i16,
    urgent_mention_count: i16,
    msg_count_root: i16,
    notify_props: NotifyProps,
    last_update_at: i16,
    scheme_guest: bool,
    scheme_user: bool,
    scheme_admin: bool,
    explicit_roles: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotifyProps {
    channel_auto_follow_threads: Option<String>,
    desktop: Option<String>,
    email: Option<String>,
    ignore_channel_mentions: Option<String>,
    mark_unread: Option<String>,
    push: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Error)]
pub struct ServerApiError {
    pub id: String,
    pub message: String,
    pub request_id: String,
    pub status_code: i16,
}

impl fmt::Display for ServerApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&serde_json::to_string(self).unwrap())
    }
}
