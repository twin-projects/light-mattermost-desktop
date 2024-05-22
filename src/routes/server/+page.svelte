<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { state } from '$lib/store';
</script>

<form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
	<div class="mb-4">
		<label class="block text-gray-700 text-sm font-bold mb-2" for="server_name">
			Server name
		</label>
		<input
			class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
			id="server_name" placeholder="server_name" type="text">
		<label
			class="block text-gray-700 text-sm font-bold mb-2"
			for="server_url">
			Server url
		</label>
		<input
			class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
			id="server_url" placeholder="server_url" type="url">
	</div>
	<button
		class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
		type="button">
		add
	</button>
</form>
