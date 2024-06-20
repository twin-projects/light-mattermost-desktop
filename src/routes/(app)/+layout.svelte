<script lang="ts">
	import { AppBar, getToastStore, initializeStores, ListBoxItem, Toast } from '@skeletonlabs/skeleton';
	import { Icon } from 'svelte-icons-pack';
	import { FaSolidCirclePlus, FaSolidCircleUser, FaSolidServer } from 'svelte-icons-pack/fa';
	import { changeServer, state } from '$lib/store';
	import { goto } from '$app/navigation';
	import Dropdown from '$lib/components/ui/Dropdown.svelte';
	import { handle_result } from '$lib/utils/server.utils';
	import { failed_toast } from '$lib/utils/toast';
	import { logout } from '$lib/controllers';
	import type { ApiErrorModel } from '$lib/types/api.error.model';

	initializeStores();

	const toastStore = getToastStore();
	let serverValue: string = $state.currentServer?.name ?? 'Select';

	const goToAddServer = async () => goto('/add_server').catch(console.error);

	const toast_error = (error: string | ApiErrorModel) => toastStore.trigger(failed_toast(error));

	const sendChangeServer = async (serverName: string) => {
		await changeServer(serverName).then((change_server_result) => handle_result(
			change_server_result,
			toast_error,
			async (server) => {
				await logout().then(logoutResult => handle_result(logoutResult, toast_error, () => $state.user = null));
				$state.currentServer = server.current;
				serverValue = server.current.name;
				goto('/login').catch(console.error);
			}
		));
	};
</script>

<Toast />
<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
	<svelte:fragment slot="lead">
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
	<h1 class="text-4xl">
		Mattermost
	</h1>
	<svelte:fragment slot="trail">
		<a href="/profile">
			<Icon src={FaSolidCircleUser} />
		</a>
	</svelte:fragment>
</AppBar>
<slot />
