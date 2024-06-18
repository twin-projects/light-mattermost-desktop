<script lang="ts">
	import type { ChannelModel } from '$lib/types/channel.model';
	import { BsArrowBarLeft, BsArrowBarRight, BsChatSquareTextFill, BsUpc } from 'svelte-icons-pack/bs';
	import { Icon } from 'svelte-icons-pack';
	import SidebarElement from '$lib/components/ui/SidebarElement.svelte';
	import Tooltip from '$lib/components/ui/Tooltip.svelte';

	export let channels: ChannelModel[];
	export let open = true;
	export let sidebar_action: () => void

</script>
<div class={`fixed flex flex-col bg-white h-full border-r ${open ? 'w-48' : 'w-16' }`}>
	<div class="overflow-y-auto overflow-x-hidden flex-grow">
		<ul class="flex flex-col py-4 space-y-1">
			{#if channels.length > 0}
				<li class="px-5">
					<div class="flex flex-row items-center h-8">
						<div class="text-sm font-light tracking-wide text-gray-500">
							{#if open}
								Channels
							{:else }
								<Icon src={BsUpc} size="32" color="#6b7280" />
							{/if}
						</div>
					</div>
				</li>
				{#each channels as channel}
					{#if open}
						<SidebarElement
                            id={channel.id}
							label={channel.display_name}
							href="/"
							icon={open ? null : BsChatSquareTextFill}
						/>
					{:else}
						<Tooltip title={channel.display_name}>
							<SidebarElement
                                id={channel.id}
								label={channel.display_name}
								href="/"
								icon={open ? null : BsChatSquareTextFill}
							/>
						</Tooltip>
					{/if}
				{/each}
			{/if}
			<li
				class="relative inline-block items-center h-11 focus:outline-none hover:bg-gray-50 text-gray-600 hover:text-gray-800 border-l-4 border-transparent ml-4"
			>
				<button class="w-full" on:click={sidebar_action}>
					<Icon src={open ? BsArrowBarLeft : BsArrowBarRight} size="32" />
				</button>
			</li>
		</ul>
	</div>
</div>
