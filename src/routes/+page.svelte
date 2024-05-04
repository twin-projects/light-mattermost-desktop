<script lang="ts">
	import Login from '$lib/Login.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import AddServer from '$lib/AddServer.svelte';

	let url: string = 'http://localhost:8065';

	const getCurrentServer = () => {
		invoke('get_current_server')
			.then(it => it as string)
			.then((current) => {
				url = current;
			});
	};
	getCurrentServer();
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5">
		<h1 class="h1">Mattermost</h1>
		<section class="space-y-4">
			{#if url === ''}
				<AddServer currentUrl={getCurrentServer} />
			{:else}
				<Login />
			{/if}
		</section>
	</div>
</div>
