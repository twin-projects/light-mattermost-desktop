export type UserModel = {
	username: string,
	email: string,
	nickname: string,
	first_name: string,
	last_name: string,
	last_password_update: number,
	locale: string,
	disable_welcome_email: boolean,
}