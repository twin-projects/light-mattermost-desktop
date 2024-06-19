<script lang="ts">

	import type { PageData } from '$lib/store';
	import { initializeStores, Toast } from '@skeletonlabs/skeleton';
	import UserTeams from '$lib/components/team/UserTeams.svelte';
	import Sidebar from '$lib/components/ui/Sidebar.svelte';
	import ChannelPosts from '$lib/components/ui/channel_posts/ChannelPosts.svelte';

	export let data: PageData;

	let open = true;

	initializeStores();

</script>

<Toast />
<div class="flex flex-row antialiased">
	<Sidebar
		open={open}
		sidebar_action={() => open = !open}
		channels={data.channels.filter(channel => channel.display_name )}
	/>
    <div class={`flex flex-col justify-between ${open ? 'ml-48': 'ml-12'}`}>
        <section
            class={`flex flex-col w-full`}
        >
            <UserTeams teams={data.teams} />
        </section>

        <ChannelPosts thread={data.channelPosts} />
        <input class="mt-5 mb-2" />
    </div>
</div>
