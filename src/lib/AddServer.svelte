<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	export let currentUrl: () => void;

	let url: string = '';

	const addServer = () => {
		invoke('add_server', { url })
			.then(current => console.log(`Switch to server url: ${current}`))
			.then(currentUrl);
	};
</script>

<form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
	<div class="mb-4">
		<label class="block text-gray-700 text-sm font-bold mb-2" for="username">
			Server url
		</label>
		<input
			bind:value={url}
			class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
			id="username" placeholder="Username" type="text">
	</div>
	<button
		class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
		on:click={addServer}
		type="button">
		add
	</button>
</form>