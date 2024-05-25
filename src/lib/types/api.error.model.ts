export type ApiErrorModel = {
	id: string,
	message: string, // the reason for the error
	request_id: string, // the ID of the request
	status_code: number, // the HTTP status code
}
