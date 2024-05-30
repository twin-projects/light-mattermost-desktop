// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
import type { UserModel } from './types/login.model';
import type { TeamModel } from './types/team.model';
import type { ServerModel } from './types/server.model';
import type { TeamMemberModel } from '$lib/types/team.member.model';

declare namespace App {
	// interface Locals {}
	interface PageData {
		currentServer: ServerModel | null;
		user?: UserModel | null;
		teams?: TeamModel[];
		teamMembers?: TeamMemberModel[];
	}
	interface PageState {
		currentServer: ServerModel | null;
		user?: UserModel | null;
		teams?: TeamModel[];
		teamMembers?: TeamMemberModel[];
	}
	interface Locals {
		user?: UserModel
	}
	// interface Error {}
	// interface Platform {}
}
