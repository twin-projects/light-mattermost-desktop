import { type PageState, refresh } from '$lib/store';
import { goto } from '$app/navigation';

export const prerender = false;
export const ssr = false;

export const load = async (): Promise<PageState> =>
	refresh(() => goto('/login'));
