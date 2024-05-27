<script lang="ts">
	import type { PageData } from '$lib/store';
	import { goto } from '$app/navigation';
	import { initializeStores, Toast } from '@skeletonlabs/skeleton';
	import UserTeams from '$lib/team/UserTeams.svelte';
	import Channels from '$lib/components/channels/Channels.svelte';

	export let data: PageData;

	initializeStores();

	$: if (data.user === null) goto('/login').catch(console.error);
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<Toast />
	<div class="space-y-5">
		<section class="space-y-4">
			<UserTeams teams={data.teams} />
			<Channels channels={data.channels ?? []} />
		</section>
	</div>
</div>
