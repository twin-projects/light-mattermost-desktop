import { defaultState, type PageState, state, initNavigation } from '$lib/store';

export const prerender = true;
export const ssr = false;

export const load = async () => {
	await initNavigation();
};
