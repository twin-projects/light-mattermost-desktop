<script lang="ts">
	import '../app.postcss';
	import { AppBar, ListBoxItem } from '@skeletonlabs/skeleton';
	import { Icon } from 'svelte-icons-pack';
	import { FaSolidBars, FaSolidCircleUser, FaSolidServer } from 'svelte-icons-pack/fa';
	import { servers, state } from '$lib/store';
	import Dropdown from '$lib/ui/Dropdown.svelte';

	let serverValue: string = $state.currentServer?.name ?? 'Select';
</script>

<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
	<svelte:fragment slot="lead">
		<Icon src={FaSolidBars} />
		<Dropdown value={serverValue}>
			<svelte:fragment slot="title">
				<Icon src={FaSolidServer} />
			</svelte:fragment>
			<svelte:fragment slot="elements">
				{#each $servers as server}
					<ListBoxItem bind:group={serverValue} name="medium" value={server.name}>
						{server.name}
					</ListBoxItem>
				{/each}
			</svelte:fragment>
		</Dropdown>
	</svelte:fragment>
	<h1 class="text-4xl">Mattermost</h1>
	<svelte:fragment slot="trail">
		<Icon src={FaSolidCircleUser} />
	</svelte:fragment>
</AppBar>
<slot />
