import type { ChangePostInput, CreatePostInput } from '$lib/api/types/blogs';
import { error, redirect } from '@sveltejs/kit';
import type { RequestEvent } from './$types';
import type { PageServerLoad } from './$types';

export const load = (async ({ locals, params }: RequestEvent) => {
	if (!params.post_id) {
		return {
			post: {}
		};
	}
	let post = await locals.api.post(params.post_id);

	return {
		post
	};
}) satisfies PageServerLoad;

export const actions = {
	create: async ({ request, locals, params }: RequestEvent) => {
		const values = await request.formData();
		const title = values.get('title')?.toString();
		const body = values.get('body')?.toString();

		if (!title || !body) {
			throw error(400, 'Title and body are required');
		}

		let input = {
			title,
			body: JSON.parse(body),
			locale: 'pt-BR',
			featured: false
		} satisfies CreatePostInput;

		let post = await locals.api.create_post(input);
		return {
			post
		};
	},

	update: async ({ request, locals, params }: RequestEvent) => {
		const values = await request.formData();
		const title = values.get('title')?.toString();
		const body = values.get('body')?.toString();

		if (!params.post_id || !title || !body) {
			throw error(400, 'Title and content are required');
		}

		let input = {
			title,
			body: JSON.parse(body)
		} satisfies ChangePostInput;

		let post = await locals.api.update_post(params.post_id, input);

		return {
			post
		};
	}
};
