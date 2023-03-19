import { BlogsService } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async () => {
	let categories = await BlogsService.listPostCategories({});

	return {
		items: categories,
		pageTitle: 'Categories'
	};
}) satisfies PageServerLoad;
