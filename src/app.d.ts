// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
import type { UserModel } from './types/login.model';
import type { TeamModel } from './types/team.model';
import type { ServerModel } from './types/server.model';

declare namespace App {
	// interface Locals {}
	interface PageData {
		currentServer: ServerModel | null;
		user: UserModel | null;
		teams: TeamModel[];
	}
	interface PageState {
		currentServer: ServerModel | null;
		user: UserModel | null;
		teams: TeamModel[];
	}

	// interface Error {}
	// interface Platform {}
}
