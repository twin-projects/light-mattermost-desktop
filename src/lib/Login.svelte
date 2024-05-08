<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { UserModel } from '../types/login.model';

	export let user: UserModel | null;

	let login = '';
	let password = '';
    let instance = '';

	const authenticate = async () => {
		const response: UserModel = await invoke('login', { login, password, instance });
		console.log('res auth', response);
		user = response;
	};

	const logout = async () => {
		await invoke('logout');
		user = null;
	};
</script>

<div class="card p-4 flex gap-4 flex-col">
	<div class="w-full max-w-xs">
		{#if !user?.username && !user?.first_name && !user?.last_name && !user?.email && !user?.nickname}
			<form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
				<div class="mb-4">
					<label class="block text-gray-700 text-sm font-bold mb-2" for="username">
						Instance
					</label>
					<input
						bind:value={instance}
						class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
						id="instance" placeholder="https://mattermost.example.com" type="url"
                    />
				</div>
				<div class="mb-4">
					<label class="block text-gray-700 text-sm font-bold mb-2" for="username">
						Username
					</label>
					<input
						bind:value={login}
						class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
						id="username" placeholder="Username" type="text"
                    />
				</div>
				<div class="mb-6">
					<label class="block text-gray-700 text-sm font-bold mb-2" for="password">
						Password
					</label>
					<input
						bind:value={password}
						class="shadow appearance-none border border-red-500 rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
						id="password" placeholder="******************" type="password"
                    />
					<p class="text-red-500 text-xs italic">Please choose a password.</p>
				</div>
				<div class="flex items-center justify-between">
					<button
						class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
						on:click={authenticate}
						type="button">
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
		{/if}
	</div>
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

	<p class="text-center text-gray-500 text-xs">
		&copy;2020 Acme Corp. All rights reserved.
	</p>
</div>
