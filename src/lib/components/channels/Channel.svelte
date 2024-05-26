<script lang="ts">
	import type { ChannelModel } from '$lib/types/channel.model';
	import { ListBoxItem } from '@skeletonlabs/skeleton';

	export let channel: ChannelModel
</script>

<ListBoxItem bind:group={channel.name} name="medium" value={channel.name}>
	{channel.display_name ?? channel.name}
</ListBoxItem>