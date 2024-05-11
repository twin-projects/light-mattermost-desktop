import { defaultState, type PageState, state } from '$lib/store';
import { get_my_teams } from '$lib/controllers';

export const prerender = 'auto';
export const ssr = false;

export const load = async () => {
	let pageState: PageState = defaultState;
	state.subscribe((value) => {
		pageState = value;
	});
	if (pageState.user !== null) {
		await get_my_teams().then((teams) => {
			state.update((value) => ({ ...value, teams: teams ?? [] }));
			pageState.teams = teams ?? [];
		});
	}
	return { ...pageState };
};