import { BlogsService } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async () => {
	let posts = await BlogsService.listPosts({});
	let categoriesIds: string[] = posts
		.map((post) => post.category_id)
		.filter((id) => id !== null) as string[];

	let promises = categoriesIds.map((id) => {
		return BlogsService.getPostCategoryById({ categoryId: id });
	});
	let categories = await Promise.all(promises);

	return {
		posts,
		categories,
		pageTitle: 'Posts'
	};
}) satisfies PageServerLoad;
