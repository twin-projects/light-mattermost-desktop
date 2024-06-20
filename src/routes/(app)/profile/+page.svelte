<script lang="ts">
	import { state } from '$lib/store';
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';

	const logout_and_redirect = async () => {
		await invoke('logout');
		$state.user = null;
		goto('/login').catch(console.error);
	};
</script>

<section class="container m-auto p-12">
	<h2 class="text-2xl font-bold pb-2">
		Account
	</h2>
	<h3>
		You are logged in as {$state.user?.username}
	</h3>

	<button
		on:click={logout_and_redirect}
		class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
	>
		Logout
	</button>
</section>