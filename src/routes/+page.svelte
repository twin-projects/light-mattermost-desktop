<script lang="ts">
	import Login from '$lib/Login.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import AddServer from '$lib/AddServer.svelte';
	import type { Team } from '../types/team.model';
	import type { UserModel } from '../types/login.model';

	let url: string = 'http://localhost:8065';
	let teams: Team[] = [];
	let user: UserModel | null = null;

	const getCurrentServer = () => {
		invoke('get_current_server')
			.then(it => it as string)
			.then((current) => url = current);
	};
	getCurrentServer();

	const get_my_teams = () => {
		invoke('my_teams')
			.then(teams => teams as Team[])
			.then(it => {
				console.warn(it);
				return teams = it;
			});
	};

	$: if (user) get_my_teams();
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5">
		<h1 class="h1">Mattermost</h1>
		<section class="space-y-4">
					<button
						class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
						type="button" on:click={get_my_teams}>teams</button>
			{#if url === ''}
				<AddServer currentUrl={getCurrentServer} />
			{:else}
				<Login user={user} />
			{/if}
			{JSON.stringify(teams)}
		</section>
	</div>
</div>
