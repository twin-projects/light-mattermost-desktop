import type { IndexedModel } from '$lib/types/indexed.model';

export const ids_order_sorting = <T extends IndexedModel>(order: string[]) =>
	(a: T, b: T): number =>
		order.indexOf(a.id) > order.indexOf(b.id)
			? -1
			: 1;
