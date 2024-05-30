<script lang="ts">
	import type { TeamModel } from '$lib/types/team.model';

	export let teams: TeamModel[];
</script>

<div class="card p-4 flex gap-4 flex-col">
	<div class="w-full max-w-xs">
		{#if teams.length > 0}
			{#each teams as team}
				<div class="card">
					<h4 class="card-header">Team: {team.display_name}</h4>
					<ul class="card-hover">
						<li class="text-lg">{team.description}</li>
					</ul>
				</div>
			{/each}
		{/if}
	</div>
</div>
