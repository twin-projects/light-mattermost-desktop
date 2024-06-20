import { type PageState, refresh } from '$lib/store';

export const prerender = false;
export const ssr = false;

export const load = async (): Promise<PageState> =>
	refresh();
