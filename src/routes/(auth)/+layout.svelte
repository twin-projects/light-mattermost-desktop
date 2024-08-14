<script lang="ts">
import { FaSolidServer } from 'svelte-icons-pack/fa';
import { changeServer, state } from '$lib/store.js';
import { Icon } from 'svelte-icons-pack';
import { getToastStore, initializeStores, ListBoxItem, Toast } from '@skeletonlabs/skeleton';
import Dropdown from '$lib/components/ui/Dropdown.svelte';
import { handle_result } from '$lib/utils/server.utils';
import { logout } from '$lib/controllers';
import { goto } from '$app/navigation';
import type { ApiErrorModel } from '$lib/types/api.error.model';
import { failed_toast } from '$lib/utils/toast';

let serverValue: string = $state.currentServer?.name ?? 'Select';

initializeStores();
const toastStore = getToastStore();
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
<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5">
		<section class="space-y-4 m-12">
			<div class="flex items-center justify-center	">
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
			</div>
			<slot />
		</section>

	</div>
</div>
