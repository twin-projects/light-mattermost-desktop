import type { ServerModel } from '$lib/types/server.model';
import type { UserModel } from '$lib/types/login.model';
import type { TeamModel } from '$lib/types/team.model';
import type { Writable } from 'svelte/store';
import type { ChannelModel } from '$lib/types/channel.model';
import type { TeamMemberModel } from '$lib/types/team.member.model';
import { writable } from 'svelte/store';
import {
	add_server,
	change_server,
	get_all_servers,
	get_current_server, get_my_channels,
	get_my_team_members,
	get_my_teams,
	login
} from '$lib/controllers';

export const addServer = add_server;
export const loginCmd = login;
export const changeServer = change_server;

export interface PageState {
	currentServer: ServerModel | null;
	user: UserModel | null;
	teams: TeamModel[];
	teamMembers: TeamMemberModel[];
	channels: ChannelModel[];
	servers: ServerModel[];
}

export interface PageData {
	currentServer: ServerModel | null;
	user: UserModel | null;
	teams: TeamModel[];
	teamMembers: TeamMemberModel[];
	channels: ChannelModel[];
	servers: ServerModel[];
}

export const servers: Writable<ServerModel[]> = writable([]);

export const defaultState = {
	currentServer: {
		url: '',
		name: ''
	} as ServerModel,
	user: null,
	teams: [],
	teamMembers: [],
	channels: [],
	servers: []
} as PageState;

export const state = writable(defaultState);

export const initNavigation = async () => {
	let pageState: PageState = defaultState;

	state.subscribe((value) => {
		pageState = value;
	});
	if (pageState.user !== null) {
		await get_my_teams().then((teams) => {
			state.update((value) => ({ ...value, teams: teams ?? [] }));
			console.log('my_teams', teams);
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
