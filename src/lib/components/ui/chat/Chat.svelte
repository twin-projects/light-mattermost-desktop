<script lang="ts">
	import type { PostModel } from '$lib/types/post.thread.model';
	import type { UserModel } from '$lib/types/login.model';
	import type { PostCollectionModel } from '$lib/types/ui/posts/post.collection.model';
	import SubPosts from '$lib/components/ui/chat/SubPosts.svelte';

	export let user: UserModel;
	export let posts: PostModel[];

	let post_roots: PostCollectionModel[];

	$: if (posts) {
		const roots = posts.filter(post => !post.root_id);
		post_roots = roots.map(root => ({
			root,
			children: posts
				.filter(post => post.root_id && post.root_id === root.id)
		}) as PostCollectionModel);
	}

	const make_post_class = (post: PostModel, user: UserModel): string =>
		`flex ${user.id === post.user_id
			? 'justify-end'
			: 'justify-start'
		} ${post.root_id === '' ? 'mb-2' : 'mb-1'}`;
</script>

<div class="container mx-auto shadow-lg rounded-lg">
	<div class="px-5 py-5 flex justify-between items-center bg-white border-b-2">
		<div class="w-10/12">
			<input
				type="text"
				name=""
				id=""
				placeholder="search"
				class="rounded-2xl bg-gray-100 py-3 px-5 w-full"
			/>
		</div>
		<div
			class="h-12 w-12 p-2 bg-yellow-500 rounded-full text-white font-semibold flex items-center justify-center"
		>
			{user.username}
		</div>
	</div>

	<div class="flex flex-row justify-between bg-white">
		<div class="w-full px-5 flex flex-col justify-between">
			<div class="flex flex-col mt-5">
				{#each post_roots as post}
					<div class={make_post_class(post.root, user)}>
						{#if user.id !== post.root.user_id}
							<img
								src="https://source.unsplash.com/vpOeXr5wmR4/600x600"
								class="object-cover h-8 w-8 rounded-full"
								alt=""
							/>
						{/if}
						<div
							class={`ml-2 py-3 px-4 rounded-tl-3xl rounded-tr-xl text-white ${
								user.id === post.root.user_id
								? 'bg-blue-400 rounded-bl-3xl '
								: 'bg-gray-400 rounded-br-3xl'
							} `}
						>
							{post.root.message}
						</div>
						{#if user.id === post.root.user_id}
							<img
								src="https://source.unsplash.com/vpOeXr5wmR4/600x600"
								class="object-cover h-8 w-8 rounded-full"
								alt=""
							/>
						{/if}
					</div>

					<SubPosts post={post} />
				{/each}
			</div>
			<div class="py-5">
				<input
					class="w-full bg-gray-300 py-5 px-3 rounded-xl text-black"
					type="text"
					placeholder="type your message here..."
				/>
			</div>
		</div>
	</div>
</div>

