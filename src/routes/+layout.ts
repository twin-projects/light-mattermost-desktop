import { defaultState, state } from '$lib/store';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async (event) => {
	let data = defaultState;
	state.subscribe(value => data = value);

	if (!data?.user && !event.route.id?.includes('/login')) {
		redirect(307, '/login');
	} else {
		return {
			...data
		};
	}
};