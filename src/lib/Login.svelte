<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { UserModel } from '../types/login.model';

	let login = '';
	let password = '';

	let firstName = '';
	let lastName = '';
	let username = '';
	let email = '';
	let nickname = '';

	const authenticate = async () => {
		const response: UserModel = await invoke('login', { login, password });
		firstName = response.first_name;
		lastName = response.last_name;
		username = response.username;
		email = response.email;
		nickname = response.nickname;
	};

	const logout = async () => {
		await invoke('logout');
		firstName = '';
		lastName = '';
		username = '';
		email = '';
		nickname = '';
	};
</script>

<div class="card p-4 flex gap-4 flex-col">
	<div class="w-full max-w-xs">
		{#if !username && !firstName && !lastName && !email && !nickname}
			<form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
				<div class="mb-4">
					<label class="block text-gray-700 text-sm font-bold mb-2" for="username">
						Username
					</label>
					<input
						bind:value={login}
						class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
						id="username" placeholder="Username" type="text">
				</div>
				<div class="mb-6">
					<label class="block text-gray-700 text-sm font-bold mb-2" for="password">
						Password
					</label>
					<input
						bind:value={password}
						class="shadow appearance-none border border-red-500 rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
						id="password" placeholder="******************" type="password">
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
	{#if firstName}
		<p class="card-footer">firstName {firstName}</p>
	{/if}
	{#if lastName}
		<p class="card-footer">lastName {lastName}</p>
	{/if}
	{#if username}
		<p class="card-footer">username {username}</p>
	{/if}
	{#if email}
		<p class="card-footer">email {email}</p>
	{/if}
	{#if nickname}
		<p class="card-footer">nickname {nickname}</p>
	{/if}

	<p class="text-center text-gray-500 text-xs">
		&copy;2020 Acme Corp. All rights reserved.
	</p>
</div>
