import { state } from '$lib/store';
import { get_current_server } from '$lib/controllers';
import { goto } from '$app/navigation';

export const prerender = 'auto';
export const ssr = false;

const get_server = async () => {
	await get_current_server()
		.then((current) => {
			state.update((value) =>
				({ ...value, currentServer: current })
			);
			return current;
		})
		.then((value) => {
			if (value?.url === '') goto('/server').catch(console.error);
		});
};

export const load = async () => await get_server();