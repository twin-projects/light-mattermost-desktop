import type { ServerModel } from '$lib/types/server.model';
import type { UserModel } from '$lib/types/login.model';
import type { TeamModel } from '$lib/types/team.model';
import type { ChannelModel } from '$lib/types/channel.model';
import type { TeamMemberModel } from '$lib/types/team.member.model';
import { writable } from 'svelte/store';
import {
	add_server,
	change_server,
	get_all_servers,
	get_current_server,
	get_my_channels,
	get_my_team_members,
	get_my_teams,
	login,
    channel_posts,
    user_unread,
} from '$lib/controllers';
import type { ApiErrorModel } from '$lib/types/api.error.model';
import { result_updater } from '$lib/utils/server.utils';

export const addServer = add_server;
export const loginCmd = login;
export const changeServer = change_server;

export const userUnread = async (user, channel) => {
    user_unread(user.user_id, channel.id).then(result => {
        result_updater(result, (state, channelPosts) => ({ ...state, channelPosts }))
    })
};

export interface PageState {
	currentServer: ServerModel | null;
	user: UserModel | null;
	teams: TeamModel[];
	teamMembers: TeamMemberModel[];
	channels: ChannelModel[];
	servers: ServerModel[];
	errors?: ApiErrorModel[];
}

export interface PageData {
	currentServer: ServerModel | null;
	user: UserModel | null;
	teams: TeamModel[];
	teamMembers: TeamMemberModel[];
	channels: ChannelModel[];
    currentChannel: ChannelModel | null,
    channelPosts?: ChannelPosts;
	servers: ServerModel[];
}

export const defaultState = {
	currentServer: {
		url: '',
		name: ''
	} as ServerModel,
	user: null,
	teams: [],
	teamMembers: [],
	channels: [],
	servers: [],
    channelPosts: null,
	errors: [],
} as PageState;

export const state = writable(defaultState);

export const refresh = async (on_unlogged?: () => Promise<void>): Promise<PageState> => {
	let pageState: PageState = defaultState;

	state.subscribe((value) => pageState = value);

	if (pageState.user === null) {
		if (on_unlogged) on_unlogged().catch(console.error);
	} else {
		await get_my_teams().then((result) =>
			result_updater(result, (state, teams) =>
				({ ...state, teams: teams ?? [] }))
		);
		await get_my_team_members().then((result) =>
			result_updater(result, (state, teamMembers) =>
				({ ...state, teamMembers: teamMembers ?? [] })));
        let localChannels = [];
		await get_my_channels().then((result) => {
            console.log(result);
            localChannels = [...result.right];
			return result_updater(result, (state, channels) => ({ 
                ...state,
                currentChannel: channels ? channels[0] : null,
                channels: channels ?? [],
            }))
        });

        if (localChannels.length > 0) {
            let channel = localChannels[0];
            userUnread(pageState.user, channel).await;
        }
	}

	await get_all_servers().then((result) =>
		result_updater(result, (state, be_servers) =>
			({ ...state, servers: be_servers ?? [] })));

	await get_current_server().then((result) =>
		result_updater(result, (state, current) =>
			({ ...state, currentServer: current })));
	state.subscribe((state) => pageState = state);

	return { ...pageState };
};

export const initNavigation = async (): Promise<PageState> => refresh();

