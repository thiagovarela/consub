import { ClippingsService } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async () => {
	let categories = await ClippingsService.listCategories({});

	return {
		items: categories
	};
}) satisfies PageServerLoad;
