import { initNavigation } from '$lib/store';

export const prerender = false;
export const ssr = false;

export const load = async () => {
	const result = await initNavigation();
	return {
		...result
	};
};
