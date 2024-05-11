import { defaultState, type PageState, state } from '$lib/store';

export const prerender = true;
export const ssr = false;

export const load = () => {
	let pageState: PageState = defaultState;
	state.subscribe(value => {
		pageState = value;
	});
	return {
		user: pageState.user
	};
};