import { change_server } from "$lib/controllers";
import { get_all_servers, get_my_teams } from '$lib/controllers';
import { defaultState, type PageState, servers, state } from '$lib/store';

/** @type {import('./$types').Actions} */
export const actions = {
	changeServer: async (event) => {
        console.log(event);
        const server_name = "";
        return;
		await change_server(server_name).then(async (newServers) => {
		 	state.update((value) => ({ ...value, current: newServers.currentServer, servers: newServers.list }));
		 });
	},
};

export const load = async () => {
	let pageState: PageState = defaultState;

	 state.subscribe((value) => {
	 	pageState = value;
	 });
	 if (pageState.user !== null) {
	 	await get_my_teams().then((teams) => {
	 		state.update((value) => ({ ...value, teams: teams ?? [] }));
	 		pageState.teams = teams ?? [];
	 	});
	 }
	 await get_all_servers().then((be_servers) => {
	 	if (be_servers) {
	 		pageState.servers = be_servers;
	 		servers.update(() => be_servers);
	 	}
	 });

	return { ...pageState };
};
