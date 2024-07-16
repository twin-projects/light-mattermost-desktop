<script lang="ts">
	import type { PageData } from '$lib/store';
	import type { ChannelModel } from '$lib/types/channel.model';
	import type { PostModel } from '$lib/types/post.thread.model';
	import { initializeStores, Toast } from '@skeletonlabs/skeleton';
	import Sidebar from '$lib/components/ui/Sidebar.svelte';
	import Chat from '$lib/components/ui/chat/Chat.svelte';
	import { ids_order_sorting } from '$lib/utils/order.sorter.util';

	export let data: PageData;
	let open = true;
	let current_channel: ChannelModel;
	let channels: ChannelModel[] = [];
	let posts: PostModel[] = [];

	initializeStores();

	const select_channel = (id: string) => {

		const selected = data.channels
			.filter(channel => channel.display_name)
			.find(channel => channel.id === id);

		if (selected) {
			current_channel = selected;
			posts = channels
				.filter(channel => channel.id === current_channel.id)
				.flatMap(channel =>
					channel.post_thread.posts.sort(ids_order_sorting(channel.post_thread.order))
				);
		}
	};

	$: if (data.channels?.length > 0) {
		channels = data.channels;
	}
</script>

<Toast />
<div class="flex flex-row antialiased ">
	<Sidebar
		open={open}
		sidebar_action={() => open = !open}
		channels={data.channels.filter(channel => channel.display_name)}
		select={select_channel}
	/>
	<section
		class={`flex flex-col w-full ${open ? 'ml-48': 'ml-12'}`}
	>
		{#if data.user}
			<Chat posts={posts} user={data.user} />
		{/if}
	</section>
</div>
