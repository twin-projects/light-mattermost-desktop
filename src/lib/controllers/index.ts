import type { TeamModel } from '$lib/types/team.model';
import type { ChannelId } from '$lib/types/channel.model';
import type { ChannelPosts } from '$lib/types/posts.model';
import type { ChangeServerResult, ServerModel } from '$lib/types/server.model';
import type { UserModel, UserId } from '$lib/types/login.model';
import type { Either } from 'fp-ts/Either';
import { left, right } from 'fp-ts/Either';
import type { ApiErrorModel } from '$lib/types/api.error.model';
import type { TeamMemberModel } from '$lib/types/team.member.model';
import type { ChannelModel } from '$lib/types/channel.model';
import { invoke, type InvokeArgs } from '@tauri-apps/api/tauri';

type CommandCallback<T> = Promise<Either<ApiErrorModel | string, T>>

const parse_error = (error: ApiErrorModel | undefined): ApiErrorModel | string => {
	if (error?.message) {
		return JSON.parse(`${error}`) as ApiErrorModel;
	}
	return `${error}`;
};

const to = <T>(result: unknown): T => result as T;
const NOOP = () => {
};

const handle_command = async <R>(
	cmd: string,
	on_success: (result: unknown) => R,
	args?: InvokeArgs
): Promise<Either<string | ApiErrorModel, R>> => {
	const log_label = cmd.replaceAll(/_/g, ' ');
	return invoke(cmd, args)
		.then(result => {
			console.info(log_label, result);
			return right(on_success(result));
		})
		.catch(error => {
			console.error(log_label, error);
			return left(parse_error(error));
		});
};

export const get_current_server = async (): CommandCallback<ServerModel> =>
	handle_command('get_current_server', to<ServerModel>);

export const change_server = async (server: string): CommandCallback<ChangeServerResult> =>
	handle_command('change_server', to<ChangeServerResult>, { serverName: server });

export const get_all_servers = async (): CommandCallback<ServerModel[]> =>
	handle_command('get_all_servers', to<ServerModel[]>);

export const get_my_teams = async (): CommandCallback<TeamModel[]> =>
	handle_command('my_teams', to<TeamModel[]>);

export const get_my_team_members = async (): CommandCallback<TeamMemberModel[]> =>
	handle_command('my_team_members', to<TeamMemberModel[]>);

export const get_my_channels = async (): CommandCallback<ChannelModel[]> =>
	handle_command('my_channels', to<ChannelModel[]>);

export const add_server = async (name: string, url: string): CommandCallback<ServerModel> =>
	handle_command('add_server', to<ServerModel>, { name, url });

export const login = async (login_id: string, password: string): CommandCallback<UserModel> =>
	handle_command('login', to<UserModel>, { login: login_id, password });

export const logout = async (): CommandCallback<void> =>
	handle_command('logout', NOOP);

export const channel_posts = async (channel: ChannelId): CommandCallback<ChannelPosts> =>
    handle_command("channel_posts", to<ChannelPosts>, { channel });

export const user_unread = async (user_id: UserId, channel_id: ChannelId): CommandCallback<ChannelPosts> =>
    handle_command("user_unread", to<ChannelPosts>, { channelId: channel_id, userId: user_id });

export const users = async (page: int): CommandCallback<ChannelPosts> =>
    handle_command("users", to<ChannelPosts>, { page, perPage: 100 });
