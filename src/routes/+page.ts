import { defaultState, type PageState, servers, state } from '$lib/store';
// import { get_all_servers, get_my_teams } from '$lib/controllers';

export const prerender = false; // 'auto';
export const ssr = true;

export const load = async () => {
	let pageState: PageState = defaultState;

	// state.subscribe((value) => {
	// 	pageState = value;
	// });
	// if (pageState.user !== null) {
	// 	await get_my_teams().then((teams) => {
	// 		state.update((value) => ({ ...value, teams: teams ?? [] }));
	// 		pageState.teams = teams ?? [];
	// 	});
	// }
	// await get_all_servers().then((be_servers) => {
	// 	if (be_servers) {
	// 		pageState.servers = be_servers;
	// 		servers.update(() => be_servers);
	// 	}
	// });

	return { ...pageState };
};
