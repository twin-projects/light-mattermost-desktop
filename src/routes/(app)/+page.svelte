<script lang="ts">
	import type { PageData } from '$lib/store';
	import { initializeStores, Toast } from '@skeletonlabs/skeleton';
	import UserTeams from '$lib/components/team/UserTeams.svelte';
	import Sidebar from '$lib/components/ui/Sidebar.svelte';

	export let data: PageData;

	let open = true;

	initializeStores();

</script>

<Toast />
<div class="flex flex-row antialiased ">
	<Sidebar
		open={open}
		sidebar_action={() => open = !open}
		channels={data.channels.filter(channel => channel.display_name )}
	/>
	<section
		class={`flex flex-col w-full ${open ? 'ml-48': 'ml-12'}`}
	>
		<UserTeams teams={data.teams} />
	</section>
</div>