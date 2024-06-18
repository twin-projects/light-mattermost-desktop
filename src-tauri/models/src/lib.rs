use std::collections::BTreeMap;
use std::fmt::Formatter;

use nutype::nutype;
use serde::{Deserialize, Serialize};
use url::Url;

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
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
    derive(
        Debug,
        Display,
        Clone,
        PartialEq,
        Serialize,
        Deserialize,
        Deref,
        TryFrom
    ),
    sanitize(trim),
    validate(not_empty)
)]
pub struct AccessToken(String);
#[nutype(
    derive(
        Debug,
        Display,
        Clone,
        PartialEq,
        Serialize,
        Deserialize,
        Deref,
        TryFrom
    ),
    sanitize(trim),
    validate(not_empty)
)]
pub struct Email(String);

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

pub type Timestamp = u64;
pub type FileDimension = usize;

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct HashTag(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct PostType(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct EmbedType(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct Message(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct ChannelId(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct TeamId(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct UserId(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Deref, From))]
pub struct PostId(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileId(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct EmojiName(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct EmojiId(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct EmbedUrl(Url);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileName(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileExt(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct MimeType(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct ChannelPurpose(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct ChannelType(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct ChannelHeader(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct ChannelName(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct ChannelDisplayName(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct TeamName(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct TeamDisplayName(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct TeamDescription(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct PolicyId(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct SchemaId(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct CompanyName(String);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileWidth(FileDimension);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileHeight(FileDimension);

#[nutype(derive(Debug, Display, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileSize(FileDimension);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetaEmbed {
    #[serde(rename = "type")]
    pub embed_type: EmbedType,
    pub url: EmbedUrl,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetaReaction {
    #[serde(rename = "type")]
    pub embed_type: EmbedType,
    pub url: EmbedUrl,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetaEmoji {
    pub id: EmojiId,
    pub creator_id: UserId,
    pub name: EmojiName,
    pub update_at: Timestamp,
    pub delete_at: Timestamp,
    pub create_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetaFile {
    pub id: FileId,
    pub user_id: UserId,
    pub post_id: PostId,
    pub update_at: Timestamp,
    pub delete_at: Timestamp,
    pub create_at: Timestamp,
    pub name: FileName,
    pub extension: FileExt,
    pub size: FileSize,
    pub width: Option<FileWidth>,
    pub height: Option<FileHeight>,
    pub mime_type: MimeType,
    pub has_preview_image: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetaPriority {
    pub priority: String,
    pub requested_ack: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetaAcknowledgement {
    #[serde(default)]
    pub user_id: Option<UserId>,
    #[serde(default)]
    pub post_id: Option<PostId>,
    #[serde(default)]
    pub acknowledged_at: Option<Timestamp>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostMetadata {
    pub embeds: Vec<MetaEmbed>,
    pub emojis: Vec<MetaEmoji>,
    pub files: Vec<MetaFile>,
    pub reactions: Vec<MetaReaction>,
    pub priorities: Vec<MetaPriority>,
    pub acknowledges: Vec<MetaAcknowledgement>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Post {
    pub id: PostId,
    pub edit_at: Timestamp,
    pub update_at: Timestamp,
    pub delete_at: Timestamp,
    pub create_at: Timestamp,
    pub user_id: UserId,
    pub channel_id: ChannelId,
    #[serde(default)]
    pub root_id: String,
    #[serde(default)]
    pub original_id: String,
    pub message: Message,
    #[serde(rename = "type")]
    pub post_type: PostType,
    #[serde(default)]
    pub hashtag: Option<HashTag>,
    #[serde(default)]
    pub file_ids: Vec<FileId>,
    #[serde(default)]
    pub pending_post_id: Option<PostId>,
    #[serde(default)]
    pub props: serde_json::Value,
    #[serde(default)]
    pub metadata: Option<MetaAcknowledgement>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostThread {
    pub order: Vec<PostId>,
    pub posts: BTreeMap<PostId, Post>,
    #[serde(default)]
    pub next_post_id: Option<PostId>,
    #[serde(default)]
    pub prev_post_id: Option<PostId>,
    pub has_next: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub id: Option<ChannelId>,
    pub create_at: Timestamp,
    pub update_at: Timestamp,
    pub delete_at: Timestamp,
    pub team_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<ChannelType>,
    pub display_name: Option<ChannelDisplayName>,
    pub name: Option<ChannelName>,
    pub header: Option<ChannelHeader>,
    pub purpose: Option<ChannelPurpose>,
    pub last_post_at: Timestamp,
    pub total_msg_count: i64,
    pub extra_update_at: Timestamp,
    pub creator_id: Option<UserId>,
    pub scheme_id: Option<SchemaId>,
    pub props: Option<NotifyProps>,
    pub group_constrained: Option<bool>,
    pub total_msg_count_root: Option<i64>,
    pub last_root_post_at: Option<Timestamp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub login_id: Login,
    pub password: Pass,
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
    pub id: Option<TeamId>,
    pub display_name: Option<TeamDisplayName>,
    pub name: Option<TeamName>,
    pub description: Option<TeamDescription>,
    pub email: Option<Email>,
    pub company_name: Option<CompanyName>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeamMember {
    pub team_id: String,
    pub user_id: String,
    pub roles: String,
    pub delete_at: Timestamp,
    pub scheme_guest: bool,
    pub scheme_user: bool,
    pub scheme_admin: bool,
    pub explicit_roles: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    user_id: UserId,
    roles: String,
    last_viewed_at: Timestamp,
    msg_count: i16,
    mention_count: i16,
    mention_count_root: i16,
    urgent_mention_count: i16,
    msg_count_root: i16,
    notify_props: NotifyProps,
    last_update_at: Timestamp,
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

#[derive(Serialize, Deserialize, Clone, Debug, thiserror::Error)]
pub struct ServerApiError {
    pub id: String,
    pub message: String,
    pub request_id: String,
    pub status_code: i16,
}

impl std::fmt::Display for ServerApiError {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(&serde_json::to_string(self).unwrap())
    }
}
