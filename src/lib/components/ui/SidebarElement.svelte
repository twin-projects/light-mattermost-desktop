<script lang="ts">

	import { Icon, type IconType } from 'svelte-icons-pack';

	export let label: string;
	export let href: string;
	export let icon: IconType | null = null;

</script>

<li>
	<a href={href}
		 class="relative flex flex-row items-center h-11 focus:outline-none hover:bg-gray-50 text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-indigo-500 pr-6">
		{#if icon}
			<span class="inline-flex justify-center items-center ml-6">
				<Icon src={icon} size="12" />
			</span>
		{/if}
		<span class="ml-2 text-sm tracking-wide truncate">
			{label}
		</span>
	</a>
</li>