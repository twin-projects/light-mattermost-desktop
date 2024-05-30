<script lang="ts">
	import { state } from '$lib/store';
</script>

<section class="container m-auto p-12">
	<h2 class="text-2xl font-bold pb-2">
		Account
	</h2>
	<h3>
		You are logged in as {$state.user?.username}
	</h3>
</section>