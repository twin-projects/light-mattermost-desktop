import type { ServerModel } from '$lib/types/server.model';
import type { UserModel } from '$lib/types/login.model';
import type { TeamModel } from '$lib/types/team.model';
import type { Writable } from 'svelte/store';
import { writable } from 'svelte/store';

export interface PageState {
	currentServer: ServerModel | null;
	user: UserModel | null;
	teams: TeamModel[];
	servers: ServerModel[];
}

export interface PageData {
	currentServer: ServerModel | null;
	user: UserModel | null;
	teams: TeamModel[];
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
	servers: []
} as PageState;

export const state = writable(defaultState);

