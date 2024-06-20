<script lang="ts">

	import { Icon, type IconType } from 'svelte-icons-pack';
	import type { ChannelModel } from '$lib/types/channel.model';
    import { state } from "$lib/store";

	export let channel: ChannelModel;
	export let id: string;
	export let label: string;
	export let href: string;
	export let icon: IconType | null = null;
	export let channel_selected: (channel: ChannelModel) => void

    const onclick = (ev) => {
        ev.preventDefault();
        channel_selected(channel);
    };

    const currentClass = () => {
        console.log('is current', $state.currentChannel.display_name, $state.currentChannel.id, channel);
        if (!$state.currentChannel.id == channel.id) return '';
        console.log('is current');
        return 'bg-gray-50 text-gray-600 text-gray-800 border-l-4 border-transparent border-indigo-500';
    };

</script>

<li data-channel-id={id}>
	<a href={href}
         on:click={onclick}
		 class={
            `relative flex flex-row items-center h-11 focus:outline-none hover:bg-gray-50 text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-indigo-500 pr-6 ${
        $state.currentChannel.id == channel.id 
        ? 'bg-gray-50 text-gray-600 text-gray-800 border-l-4 border-transparent border-indigo-500' 
        : ''
            }`
         }>
		{#if icon}
			<span class="inline-flex justify-center items-center ml-6">
				<Icon src={icon} size="12" />
			</span>
		{/if}
		<span class="ml-2 text-sm tracking-wide truncate">
			{label}
		</span>
	</a>
</li>
