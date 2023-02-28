import { ClippingsService } from '$lib/api';
import type { RequestEvent } from './$types';

export const load = async ({ locals }: RequestEvent) => {
	let items = await ClippingsService.listItems({});

	return {
		items
	};
};
