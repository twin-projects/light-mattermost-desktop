<script lang="ts">

	import type { PageData } from '$lib/store';
	import { userUnread, state } from '$lib/store';
	import { initializeStores, Toast } from '@skeletonlabs/skeleton';
    import { result_updater } from '$lib/utils/server.utils';
	import UserTeams from '$lib/components/team/UserTeams.svelte';
	import Sidebar from '$lib/components/ui/Sidebar.svelte';
	import ChannelPosts from '$lib/components/ui/channel_posts/ChannelPosts.svelte';

	export let data: PageData;

	let open = true;

	initializeStores();

    const onChannelSelected = async (channel) => {
        console.log("selected channel", channel, data);
        $state.currentChannel = channel;
        await userUnread($state.user, channel);
    };

</script>

<Toast />
<div class="flex flex-row antialiased">
	<Sidebar
		open={open}
		sidebar_action={() => open = !open}
        channel_selected={onChannelSelected}
		channels={data.channels.filter(channel => channel.display_name )}
	/>
    <div class={`w-full flex flex-col justify-between ${open ? 'ml-48': 'ml-12'}`}>
        <section
            class={`flex flex-col w-full`}
        >
            <UserTeams teams={data.teams} />
        </section>

        <ChannelPosts thread={$state.channelPosts} />
        <input class="mt-5 mb-2" />
    </div>
</div>
