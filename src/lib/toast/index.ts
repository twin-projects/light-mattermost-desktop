import type { ApiErrorModel } from '$lib/types/api.error.model';

export const user_logged_in = (username: string) => ({
	message: `You are login as ${username}`,
	autohide: false,
	timeout: 10000,
	background: 'variant-filled-success'
});

export const failed_toast = (error: ApiErrorModel) => ({
	message: error.message,
	autohide: false,
	timeout: 10000,
	background: 'variant-filled-error'
});