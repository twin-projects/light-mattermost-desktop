<script lang="ts">

	import type { ChannelModel } from '$lib/types/channel.model';
	import { ListBox } from '@skeletonlabs/skeleton';
	import Channel from '$lib/components/channels/Channel.svelte';

	export let channels: ChannelModel[];
</script>

<section id="channels" aria-label="channels">
	<h2 class="text-3xl font-bold pb-4">Channels:</h2>
	{#if channels.length > 0}
		<ListBox>
			{#each channels.filter(channel => channel.display_name.length) as channel}
				<Channel channel={channel} />
			{/each}
		</ListBox>
	{/if}
</section>
