<script lang="ts">
	import '../app.postcss';
	import { AppBar } from '@skeletonlabs/skeleton';
	import { Icon } from 'svelte-icons-pack';
	import { FaSolidBars, FaSolidCircleUser } from 'svelte-icons-pack/fa';
</script>

<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
	<svelte:fragment slot="lead">
		<Icon src={FaSolidBars} />
	</svelte:fragment>
	<h1 class="text-4xl">Mattermost</h1>
	<svelte:fragment slot="trail">
		<Icon src={FaSolidCircleUser} />
	</svelte:fragment>
</AppBar>
<slot />
