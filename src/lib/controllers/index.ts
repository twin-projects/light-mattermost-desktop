import { invoke } from '@tauri-apps/api/tauri';
import type { TeamModel } from '$lib/types/team.model';
import type { ServerModel } from '$lib/types/server.model';
import type { UserModel } from '$lib/types/login.model';

const handle_error = (e: undefined): null => {
	console.error(e);
	return null;
};

export const get_current_server = async (): Promise<ServerModel | null> =>
	invoke('get_current_server')
		.then(server_url => server_url as ServerModel)
		.then((current) => {
			console.log('current server', current.name, current.url);
			return current;
		})
		.catch(handle_error);

export const get_my_teams = async () =>
	invoke('my_teams')
		.then(myTeams => myTeams as TeamModel[])
		.catch(handle_error);

export const add_server = async (name: string, url: string): Promise<ServerModel | null> =>
	invoke('add_server', { name, url })
		.then((current) => {
			console.log(`Switch to server url: ${current}`);
			return current as ServerModel;
		})
		.catch(handle_error);

export const login = async (login_id: string, password: string) =>
	invoke('login', { login: login_id, password })
		.then(user => user as UserModel)
		.catch(handle_error);
