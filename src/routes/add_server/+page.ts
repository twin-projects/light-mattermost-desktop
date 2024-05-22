import { defaultState, type PageState, state } from '$lib/store';

export const prerender = false; // 'auto'; //true;
export const ssr = true;

export const load = () => {
	let pageState: PageState = defaultState;
	state.subscribe(value => {
		pageState = value;
	});
	return {
		servers: pageState.servers
	};
};
