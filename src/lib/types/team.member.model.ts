export type TeamMemberModel =   {
	"team_id": string,
	"user_id": string,
	"roles": string,
	"delete_at": number,
	"scheme_guest": boolean,
	"scheme_user": boolean,
	"scheme_admin": boolean,
	"explicit_roles": string
}