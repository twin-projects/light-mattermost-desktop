<script lang="ts">
	import { goto } from '$app/navigation';
	import { initializeStores } from '@skeletonlabs/skeleton';
	import { addServer } from '$lib/store';

	initializeStores();

	let server_name = '';
	let server_address = '';

	const addServerHandle = async () => {
		await addServer(server_name, server_address);
	};
	const goToLogin = async (event) => {
		event.preventDefault();
		event.stopPropagation();
		goto('/login');
	};
</script>

<div class="card p-4 flex gap-4 flex-col">
	<h1>Add server</h1>
	<div class="w-full max-w-xs">
		<form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4" on:submit={addServerHandle}>
			<div class="mb-4">
				<label class="block text-gray-700 text-sm font-bold mb-2" for="server_name">
					Server name
				</label>
				<input
					bind:value={server_name}
					class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
					id="server_name"
					placeholder="Server name"
					type="text"
				/>
			</div>
			<div class="mb-6">
				<label class="block text-gray-700 text-sm font-bold mb-2" for="server_address">
					Server address
				</label>
				<input
					bind:value={server_address}
					class="shadow appearance-none border border-red-500 rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
					id="server_address"
					placeholder="http://mattermost.com"
					type="url"
				/>
			</div>
			<div class="flex items-center justify-between">
				<button
					class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
				>
					Create server
				</button>
				<button
					class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
					on:click={goToLogin}
				>
					Go to login
				</button>
			</div>
		</form>
	</div>
</div>
