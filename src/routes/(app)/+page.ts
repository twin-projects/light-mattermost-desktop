import { defaultState, type PageState, state } from '$lib/store';
import { goto } from '$app/navigation';
import {
	get_all_servers,
	get_current_server,
	get_my_channels,
	get_my_team_members,
	get_my_teams
} from '$lib/controllers';

export const prerender = false;
export const ssr = false;

export const load = async () => {
	let pageState: PageState = defaultState;

	state.subscribe((value) =>  pageState = value);

	if (pageState.user === null) {
		goto('/login').catch(console.error);
	} else {
		await get_my_teams().then((teams) =>
			state.update((value) => ({ ...value, teams: teams ?? [] })));
		await get_my_team_members().then((teamMembers) =>
			state.update((value) => ({ ...value, teamMembers: teamMembers ?? [] })));
		await get_my_channels().then((channels) =>
			state.update((value) => ({ ...value, channels: channels ?? [] })));
	}
	await get_all_servers().then((be_servers) =>
		state.update((value) => ({ ...value, servers: be_servers ?? [] })));
	await get_current_server().then((current) =>
		state.update((value) => ({ ...value, currentServer: current })));

	state.subscribe((value) => {
		pageState = value;
	});
	return { ...pageState };
};
