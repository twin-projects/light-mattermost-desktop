import type { PostModel } from '$lib/types/post.thread.model';

export type PostCollectionModel = {
	root: PostModel;
	children: PostModel[]
}
