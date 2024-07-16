import type { Option } from 'fp-ts/Option';
import type { IndexedModel } from '$lib/types/indexed.model';

type PostId = string
type UserId = string
type ChannelId = string
type Message = string
type PostType = string
type HashTag = string
type FileId = string

export type MetaAcknowledgement = {
	user_id: UserId,
	post_id: PostId,
	acknowledged_at: Date,
}

export interface PostModel extends IndexedModel {
	id: PostId,
	edit_at: Date,
	update_at: Date,
	delete_at: Date,
	create_at: Date,
	user_id?: UserId,
	channel_id: ChannelId,
	root_id: string,
	original_id: string,
	message: Message,
	post_type: PostType,
	hashtag: HashTag,
	file_ids?: FileId[],
	pending_post_id: PostId,
	props: unknown,
	metadata?: MetaAcknowledgement,
}

export type PostThread = {
	order: PostId[],
	posts: Map<string, PostModel>,
	next_post_id: Option<PostId>,
	prev_post_id: Option<PostId>,
	has_next: boolean,
}
