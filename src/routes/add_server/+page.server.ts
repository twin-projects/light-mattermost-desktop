import { add_server } from "$lib/controllers";
import { defaultState, type PageState, servers, state } from '$lib/store';
import { goto } from '$app/navigation';

/** @type {import('./$types').Actions} */
export const actions = {
	addServer: async ({ request }) => {
        const data = await request.formData();

        const server_name = data.get("server_name") || "";
        const server_address = data.get("server_address") || "";

		await add_server(server_name, server_address).then(async (newServers) => {
		 	state.update((value) => ({ ...value, servers: newServers }));
		 });
        // goto("/login").catch(console.error);
	},
};

export function load(event) {
	return {
		user: event.locals.user
	};
}
