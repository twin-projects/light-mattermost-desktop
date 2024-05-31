import type { ApiErrorModel } from '$lib/types/api.error.model';

export const user_logged_in = (username: string) => ({
	message: `You are login as ${username}`,
	autohide: false,
	timeout: 10000,
	background: 'variant-filled-success'
});

export const failed_toast = (error: ApiErrorModel | string) => {
	if (typeof error !== 'string' && error.message !== undefined) {
		return {
			message: error.message,
			autohide: false,
			timeout: 10000,
			background: 'variant-filled-error'
		};
	} else {
		return {
			message: `${error}`,
			autohide: false,
			timeout: 10000,
			background: 'variant-filled-error'
		};
	}
};