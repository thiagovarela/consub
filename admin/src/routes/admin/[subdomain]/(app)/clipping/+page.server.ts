import { ClippingsService } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async () => {
	let items = await ClippingsService.listItems({});
	let categoriesIds: string[] = items
		.map((item) => item.category_id)
		.filter((id) => id !== null) as string[];

	let promises = categoriesIds.map((id) => {
		return ClippingsService.getCategoryById({ categoryId: id });
	});
	let categories = await Promise.all(promises);

	return {
		items,
		categories,
		pageTitle: 'Clipping'
	};
}) satisfies PageServerLoad;
