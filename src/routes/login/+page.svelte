<script lang="ts">
	import type { UserModel } from '$lib/types/login.model';
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';
	import { getToastStore, initializeStores, Toast } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
	import { loginCmd, state } from '$lib/store';
	import { failed_toast, user_logged_in } from '$lib/toast';
	import { handle_response } from '$lib/utils/server.utils';

	initializeStores();

	const toastStore = getToastStore();

	let loginId = 'admin';
	let password = 'admin123!';

	const toastMessage = (user: UserModel) => {
        console.info(user);
		toastStore.trigger(user_logged_in(user?.username));
	};

	const authenticate = async () => {
        console.info("authenticate", loginId, password);
	    const response = await loginCmd(loginId, password);
        console.log("authenticate response", response);
        return handle_response(
            response,
            (error) => {
                console.error(error);
                return toastStore.trigger(failed_toast(error));
            },
            (user) => {
                console.log(user);
                state.update((value) => ({ ...value, user }));
                toastMessage(user);
                return goto('/').catch(console.error);
            }
        );
	};

	const logout = async () => {
		await invoke('logout');
		$page.data.user = null;
	};
</script>

<div class="card p-4 flex gap-4 flex-col">
	<Toast />
	<h1>Login</h1>
	<div class="w-full max-w-2xl">
		{#if !$page.data.user}
			<form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4" on:submit={authenticate}>
				<div class="mb-4">
					<label class="block text-gray-700 text-sm font-bold mb-2" for="username">
						Username
					</label>
					<input
						bind:value={loginId}
						class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
						id="username"
						placeholder="Username"
						type="text"
					/>
				</div>
				<div class="mb-6">
					<label class="block text-gray-700 text-sm font-bold mb-2" for="password">
						Password
					</label>
					<input
						bind:value={password}
						class="shadow appearance-none border border-red-500 rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
						id="password"
						placeholder="******************"
						type="password"
					/>
					<p class="text-red-500 text-xs italic">Please choose a password.</p>
				</div>
				<div class="flex items-center justify-between">
					<button
						class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
						type="submit"
					>
						Sign In
					</button>
				</div>
			</form>
		{:else}
			<button
				on:click={logout}
				class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
			>Logout
			</button>
			<!--			<UserDetails user={user} />-->
		{/if}
	</div>
</div>
