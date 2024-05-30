import type { TeamModel } from '$lib/types/team.model';
import type { ChangeServerResult, ServerModel } from '$lib/types/server.model';
import type { UserModel } from '$lib/types/login.model';
import type { Either } from 'fp-ts/Either';
import type { ApiErrorModel } from '$lib/types/api.error.model';
import { invoke } from '@tauri-apps/api/tauri';
import { left, right} from 'fp-ts/Either';
import type { TeamMemberModel } from '$lib/types/team.member.model';
import type { ChannelModel } from '$lib/types/channel.model';

const handle_error = (err: undefined) =>
    console.error(err);

const parse_error = (error: undefined): ApiErrorModel =>
	JSON.parse(`${error}`);

export const get_current_server = async (): Promise<ServerModel | null> =>
	invoke('get_current_server')
		.then((server_url) => server_url as ServerModel)
		.then((current) => {
			console.log('current server', current.name, current.url);
			return current;
		})
		.catch(() => null);

export const change_server = async (server: string): Promise<ChangeServerResult | null> =>
	invoke('change_server', { serverName: server })
		.then((server_url) => server_url as ChangeServerResult)
		.then((result) => {
			const current = result.current;
			console.log('current server', current.name, current.url);
			return result;
		})
		.catch(() => null);

export const get_all_servers = async (): Promise<ServerModel[] | null> =>
	invoke('get_all_servers')
		.then((servers) => servers as ServerModel[])
		.catch(() => null);

export const get_my_teams = async () =>
	invoke('my_teams')
		.then((myTeams) => myTeams as TeamModel[])
		.catch(() => null);

export const get_my_team_members = async () =>
	invoke('my_team_members')
		.then((myTeamMembers) => myTeamMembers as TeamMemberModel[])
		.catch(handle_error);

export const get_my_channels = async () =>
	invoke('my_channels')
		.then((channels) => channels as ChannelModel[])
		.catch(handle_error);

export const add_server = async (name: string, url: string): Promise<ServerModel | null> =>
	invoke('add_server', { name, url })
		.then((current) => {
			console.log(`Switch to server url: ${current}`);
			return current as ServerModel;
		})
		.catch(() => null);

export const login = async (login_id: string, password: string): Promise<Either<ApiErrorModel, UserModel>> =>
	invoke('login', { login: login_id, password })
		.then((user) => { console.info(user); return right(user as UserModel); })
		.catch((error) => { console.error(error); return left(parse_error(error)); });

