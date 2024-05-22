import { add_server } from "$lib/controllers";
import { defaultState, type PageState, servers, state } from '$lib/store';

/** @type {import('./$types').Actions} */
export const actions = {
	addServer: async (event) => {
        let server_name = "";
        let server_address = "";
        console.log(event);
        return;
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
