import { ChannelId } from "./channel.model.ts";
import { UserId } from "./login.model.ts";

export type PostId = String;
export type Timestamp = String;
export type Message = String;
export type PostType = String;

export type Post = {
    id: PostId,
    edit_at: Timestamp,
    update_at: Timestamp,
    delete_at: Timestamp,
    create_at: Timestamp,
    user_id: UserId,
    channel_id: ChannelId,
    root_id: String,
    original_id: String,
    message: Message,
    type: PostType,
    hashtag: HashTag,
    file_ids: Vec<FileId>,
    pending_post_id: PostId,
    props: any,
    metadata: MetaAcknowledgement,
};

export type PostThread = {
    order: PostId[],
    posts: Post[],
    next_post_id: PostId?,
    prev_post_id: PostId?,
    has_next: bool,
};
