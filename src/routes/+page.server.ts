import { change_server } from "$lib/controllers";
import { defaultState, type PageState, servers, state } from '$lib/store';

/** @type {import('./$types').Actions} */
export const actions = {
	changeServer: async (event) => {
        console.log(event);
        const server_name = "";
        return;
		await change_server(server_name).then(async (newServers) => {
		 	state.update((value) => ({ ...value, current: newServers.current, servers: newServers.list }));
		 });
	},
};
