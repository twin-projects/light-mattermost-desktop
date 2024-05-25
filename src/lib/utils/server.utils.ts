import type { ApiErrorModel } from '$lib/types/api.error.model';
import { either } from 'fp-ts';
import { pipe } from 'fp-ts/function';

export const handle_response = <T>(
	response: either.Either<ApiErrorModel, T>,
	on_error: (e: ApiErrorModel) => void,
	on_success: (data: T) => void
): void => {
	pipe(
		response,
		either.fold(
			(error) => {
				console.error(error);
				on_error(error)
			},
			(data) => on_success(data))
	);
}