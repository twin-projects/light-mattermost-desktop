<script lang="ts">
	import '../app.postcss';
	import { AppBar, ListBoxItem } from '@skeletonlabs/skeleton';
	import { Icon } from 'svelte-icons-pack';
	import { FaSolidBars, FaSolidCircleUser, FaSolidServer, FaSolidCirclePlus } from 'svelte-icons-pack/fa';
	import { servers, state } from '$lib/store';
	import { goto } from '$app/navigation';
	import Dropdown from '$lib/ui/Dropdown.svelte';

	let serverValue: string = $state.currentServer?.name ?? 'Select';

    const goToAddServer = async () => {
        goto("/add_server");
    };
</script>

<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
	<svelte:fragment slot="lead">
		<Icon src={FaSolidBars} />
		<Dropdown value={serverValue}>
			<svelte:fragment slot="title">
				<Icon src={FaSolidServer} />
			</svelte:fragment>
			<svelte:fragment slot="elements">
                <form action="?/changeServer">
				{#each $servers as server}
					<ListBoxItem bind:group={serverValue} name="medium" value={server.name}>
						<button>
                            {server.name}
                        </button>
					</ListBoxItem>
				{/each}
                </form>
			</svelte:fragment>
		</Dropdown>
        <button on:click={goToAddServer}>
            <Icon src={FaSolidCirclePlus} />
        </button>
	</svelte:fragment>
	<h1 class="text-4xl">Mattermost</h1>
	<svelte:fragment slot="trail">
		<Icon src={FaSolidCircleUser} />
	</svelte:fragment>
</AppBar>
<slot />
