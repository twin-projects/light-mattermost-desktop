<script lang="ts">
	import type { PageData } from '$lib/store';
	import { goto } from '$app/navigation';
	import UserTeams from '$lib/team/UserTeams.svelte';

	export let data: PageData;

	$: if (data.user === null) goto('/login').catch(console.error);
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5">
		<section class="space-y-4">
			{#if data.teams.length > 0}
				<UserTeams teams={data.teams} />
			{/if}
		</section>
	</div>
</div>
