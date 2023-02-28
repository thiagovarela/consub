import type { RequestEvent } from './$types';

export const load = async ({ locals }: RequestEvent) => {
	let posts = await locals.api.posts();

	return {
		posts
	};
};
