use nutype::nutype;
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

pub type Timestamp = u64;
pub type FileDimension = usize;

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct HashTag(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct PostType(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct EmbedType(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct Message(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct ChannelId(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct UserId(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct PostId(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileId(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct EmojiName(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct EmojiId(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct EmbedUrl(Url);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileName(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileExt(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct MimeType(String);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileWidth(FileDimension);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
pub struct FileHeight(FileDimension);

#[nutype(derive(Debug, Clone, PartialEq, Serialize, Deserialize, Deref, From))]
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
    pub user_id: UserId,
    pub post_id: PostId,
    pub acknowledged_at: Timestamp
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
    pub root_id: String,
    pub original_id: String,
    pub message: Message,
    #[serde(rename = "type")]
    pub post_type: PostType,
    pub hashtag: HashTag,
    pub file_ids: Vec<FileId>,
    pub pending_post_id: PostId,
    pub props: serde_json::Value,
    pub metadata: MetaAcknowledgement,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Thead {
    pub order: Vec<PostId>,
    pub posts: Vec<Post>,
    pub next_post_id: Option<PostId>,
    pub prev_post_id: Option<PostId>,
    pub has_next: bool,
}
