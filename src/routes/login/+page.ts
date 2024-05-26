import { initNavigation } from '$lib/store';

export const prerender = false;
export const ssr = false;

export const load = async () => {
	await initNavigation();
};
