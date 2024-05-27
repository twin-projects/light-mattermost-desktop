<script lang="ts">
	import '../app.postcss';
	import { AppBar, ListBoxItem } from '@skeletonlabs/skeleton';
	import { Icon } from 'svelte-icons-pack';
	import {
		FaSolidBars,
		FaSolidCircleUser,
		FaSolidServer,
		FaSolidCirclePlus
	} from 'svelte-icons-pack/fa';
	import { state, changeServer } from '$lib/store';
	import { goto } from '$app/navigation';
	import Dropdown from '$lib/ui/Dropdown.svelte';

	let serverValue: string = $state.currentServer?.name ?? 'Select';

	const goToAddServer = async () => {
		goto('/add_server');
	};
	const sendChangeServer = async (serverName) => {
		const res = await changeServer(serverName);
        state.currentServer = res.current;
        serverValue = res.current.name;
	};
</script>

<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
	<svelte:fragment slot="lead">
		<Icon src={FaSolidBars} />
		<Dropdown value={serverValue}>
			<svelte:fragment slot="title">
				<Icon src={FaSolidServer} />
			</svelte:fragment>
			<svelte:fragment slot="elements">
				{#each $state.servers as server}
					<ListBoxItem bind:group={serverValue} name="medium" value={server.name}>
						<button class="w-full" on:click={() => sendChangeServer(server.name)}>
							{server.name}
						</button>
					</ListBoxItem>
				{/each}
			</svelte:fragment>
		</Dropdown>
		<button on:click={goToAddServer}>
			<Icon src={FaSolidCirclePlus} />
		</button>
	</svelte:fragment>
	<h1 class="text-4xl">Mattermost</h1>
	<svelte:fragment slot="trail">
		<Icon src={FaSolidCircleUser} />
	</svelte:fragment>
</AppBar>
<slot />
