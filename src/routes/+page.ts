import { defaultState, type PageState, servers, state } from '$lib/store';
import {
	get_all_servers,
	get_my_teams,
	get_current_server,
	get_my_team_members,
	get_my_channels
} from '$lib/controllers';

export const prerender = false;
export const ssr = false;

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
		await get_my_team_members().then((teamMembers) => {
			state.update((value) => ({ ...value, teamMembers: teamMembers ?? [] }));
			console.log('my_team_members', teamMembers);
			pageState.teamMembers = teamMembers ?? [];
		});
		await get_my_channels().then((channels) => {
			state.update((value) => ({ ...value, channels: channels ?? [] }));
			console.log('my_channels', channels);
			pageState.channels = channels ?? [];
		});
	}
	await get_all_servers().then((be_servers) => {
		if (be_servers) {
			pageState.servers = be_servers;
			servers.update(() => be_servers);
		}
	});
	await get_current_server().then((current) => {
		state.update((value) => ({ ...value, currentServer: current }));
		pageState.currentServer = current;
	});

	console.log(pageState);

	return { ...pageState };
};
