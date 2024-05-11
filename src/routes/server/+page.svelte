<script lang="ts">
	import { add_server } from '$lib/controllers';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { state } from '$lib/store';

	let name: string = '';
	let url: string = '';

	const add_new_server = () => {
		add_server(name, url).then((current) => {
				state.update((value) => ({ ...value, currentServer: current }));
				return $page.data.currentServer = current;
			}
		);
		goto('/').catch(console.error);
	};
</script>

<form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
	<div class="mb-4">
		<label class="block text-gray-700 text-sm font-bold mb-2" for="username">
			Server name
		</label>
		<input
			bind:value={name}
			class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
			id="username" placeholder="Username" type="text">
		<label
			class="block text-gray-700 text-sm font-bold mb-2"
			for="username">
			Server url
		</label>
		<input
			bind:value={url}
			class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
			id="username" placeholder="Username" type="text">
	</div>
	<button
		class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
		on:click={add_new_server}
		type="button">
		add
	</button>
</form>