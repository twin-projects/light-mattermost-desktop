import type { ServerModel } from '$lib/types/server.model';
import type { UserModel } from '$lib/types/login.model';
import type { TeamModel } from '$lib/types/team.model';
import type { ChannelModel } from '$lib/types/channel.model';
import type { TeamMemberModel } from '$lib/types/team.member.model';
import { writable } from 'svelte/store';
import {
	add_server,
	change_server,
	channel_posts,
	get_all_servers,
	get_current_server,
	get_my_channels,
	get_my_team_members,
	get_my_teams,
	login
} from '$lib/controllers';
import type { ApiErrorModel } from '$lib/types/api.error.model';
import { result_updater, unwrap } from '$lib/utils/server.utils';
import type { PostModel } from '$lib/types/post.thread.model';

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
	errors?: ApiErrorModel[];
}

export interface PageData {
	id: string;
	currentServer: ServerModel | null;
	user: UserModel | null;
	teams: TeamModel[];
	teamMembers: TeamMemberModel[];
	channels: ChannelModel[];
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
	errors: []
} as PageState;

export const state = writable(defaultState);

const refresh_login_user = async () => {
	await get_my_teams().then((result) =>
		result_updater(result, (state, teams) =>
			({ ...state, teams: teams ?? [] }))
	);
	await get_my_team_members().then((result) =>
		result_updater(result, (state, teamMembers) =>
			({ ...state, teamMembers: teamMembers ?? [] })));

	await get_my_channels()
		.then((result) => {
				result_updater(result,
					(state, channels) => {
						channels.forEach(channel => {
							channel_posts(channel.id).then(post_result => {
									unwrap(post_result, (post_thread) => {
										const posts: PostModel[] = [];
										for (const key in post_thread.posts) {
											// eslint-disable-next-line @typescript-eslint/ban-ts-comment
											// @ts-expect-error
											posts.push(post_thread.posts[key]);
										}
										channel.post_thread = { order: post_thread.order, posts, has_next: post_thread.has_next };
									});
								}
							);
						});
						return ({ ...state, channels: channels ?? [] });
					});
			}
		);
};

export const refresh = async (on_unlogged?: () => Promise<void>): Promise<PageState> => {
	let pageState: PageState = defaultState;

	state.subscribe((value) => pageState = value);

	if (pageState.user === null) {
		if (on_unlogged) on_unlogged().catch(console.error);
	} else {
		await refresh_login_user();
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
