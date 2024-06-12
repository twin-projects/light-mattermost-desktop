import { either } from 'fp-ts';
import { pipe } from 'fp-ts/function';
import type { Either } from 'fp-ts/Either';
import { type PageState, state } from '$lib/store';

export const handle_result = <T, R>(
	response: either.Either<R, T>,
	on_error: (e: R) => void,
	on_success: (data: T) => void
): void => {
	pipe(
		response,
		either.fold(
			(error) => on_error(error),
			(data) => on_success(data))
	);
};

export const result_updater = <E, T>(result: Either<E, T>, apply: (state: PageState, data: T) => PageState) => {
	pipe(result, either.fold(
		console.error,
		(data) => state.update((value) => apply(value, data))
	));
};

