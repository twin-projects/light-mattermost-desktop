import type { ServerModel } from '$lib/types/server.model';
import type { UserModel } from '$lib/types/login.model';
import type { TeamModel } from '$lib/types/team.model';
import { writable } from 'svelte/store';

export interface PageState {
	currentServer: ServerModel | null;
	user: UserModel | null;
	teams: TeamModel[];
}

export interface PageData {
	currentServer: ServerModel | null;
	user: UserModel | null;
	teams: TeamModel[];
}

export const defaultState = {
	currentServer: {
		url: '',
		name: ''
	} as ServerModel,
	user: null,
	teams: []
} as PageState;

export const state = writable(defaultState);

