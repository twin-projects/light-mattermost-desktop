import { servers, state } from '$lib/store';
import { get_all_servers, get_current_server } from '$lib/controllers';
import { goto } from '$app/navigation';

export const prerender = 'auto';
export const ssr = true;

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

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	servers.subscribe((value) => {
		get_all_servers().then((be_servers) => {
			if (be_servers) {
				value = be_servers;
			}
		});
	});
};

export const load = async () => await get_server();