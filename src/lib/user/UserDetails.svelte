<script lang="ts">
	import type { UserModel } from '$lib/types/login.model';

	export let user: UserModel;
</script>

{#if user?.first_name}
	<p class="card-footer">first name {user?.first_name}</p>
{/if}
{#if user?.last_name}
	<p class="card-footer">lastName {user?.last_name}</p>
{/if}
{#if user?.username}
	<p class="card-footer">username {user?.username}</p>
{/if}
{#if user?.email}
	<p class="card-footer">email {user?.email}</p>
{/if}
{#if user?.nickname}
	<p class="card-footer">nickname {user?.nickname}</p>
{/if}
