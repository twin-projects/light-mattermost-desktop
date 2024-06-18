import type { NotifyPropsModel } from '$lib/types/notify.props.model';

export type ChannelId = String;

export type ChannelModel = {
	id: ChannelId
	create_at: number
	update_at: number
	delete_at: number
	team_id: string
	type: string
	display_name: string
	name: string
	header: string
	purpose: string
	last_post_at: number
	total_msg_count: number
	extra_update_at: number
	creator_id: string
	scheme_id: string
	props: NotifyPropsModel,
	group_constrained: false,
	total_msg_count_root: number
	last_root_post_at: number
}
