import { error, fail } from '@sveltejs/kit';
import type { RequestEvent } from './$types';
import type { PageServerLoad } from './$types';
import { parse } from '$lib/locales';
import {
	BlogsService,	
	type ChangePostInput,	
	type CreatePostInput
} from '$lib/api';
import dayjs from 'dayjs';

export const load = (async ({ params, request }: RequestEvent) => {
	let categories = await BlogsService.listPostCategories({});
	let languages = parse(request.headers.get('accept-language')).map(
		(l: { code: any; region: any }) => `${l.code}-${l.region}`
	);

	if (params.post_id) {
		let post = await BlogsService.getPost({ postId: params.post_id });
		return {
			post,
			categories,
			languages,
			pageTitle: 'Post'
		};
	} else {
		return {
			categories,
			languages,
			pageTitle: 'Post'
		};
	}
}) satisfies PageServerLoad;

export const actions = {
	create: async ({ request }: RequestEvent) => {
		const values = await request.formData();
		const title = values.get('title')?.toString();
		let body_json = values.get('body_json')?.toString();
		const body_html = values.get('body_html')?.toString();
		const body_text = values.get('body_text')?.toString();
		const locale = values.get('locale')?.toString();		
		const is_featured = values.get('is_featured')?.toString();
		const category_id = values.get('category_id')?.toString();

		if (
			!title ||
			!body_json ||
			!body_html ||
			!body_text ||
			!locale
		) {
			return fail(400, { message: 'Required fields are missing!' });
		}
		body_json = JSON.parse(body_json);

		let input = {
			title,
			body_json,
			body_html,
			body_text,
			locale,
			is_featured: is_featured ? true : false,
			category_id
		} satisfies CreatePostInput;

		let item = await BlogsService.createPost({ requestBody: input });
		return {
			item,
			success: true,
			message: 'Post successfuly created'
		};
	},

	update: async ({ request, params }: RequestEvent) => {
		const values = await request.formData();

		const title = values.get('title')?.toString();
		let body_json = values.get('body_json')?.toString();
		const body_html = values.get('body_html')?.toString();
		const body_text = values.get('body_text')?.toString();		
		const is_featured = values.get('is_featured')?.toString();
		const category_id = values.get('category_id')?.toString();

		if (!params.post_id) {
			throw error(400, 'Title and content are required');
		}		

		body_json = body_json ? JSON.parse(body_json) : undefined;

		console.log(body_json);

		let input = {
			title,
			body_json,
			body_html,
			body_text,			
			is_featured: is_featured ? true : false,
			category_id
		} satisfies ChangePostInput;

		let item = await BlogsService.changePost({
			postId: params.post_id,
			requestBody: input
		});

		return {
			item,
			success: true,
			message: 'Post successfuly updated.'
		};
	},
	publish: async ({ request, params }: RequestEvent) => {
		const values = await request.formData();
		const published_at = values.get('published_at')?.toString();

		if (!params.post_id || !published_at) {
			throw error(400, 'post_id and published_at are required.');
		}

		let input = {
			published_at: dayjs(published_at).format('YYYY-MM-DDTHH:mm:ss')
		} satisfies ChangePostInput;

		let post = await BlogsService.changePost({
			postId: params.post_id,
			requestBody: input
		});

		return {
			post,
			success: true,
			message: 'Post successfuly published.'
		};
	},
	unpublish: async ({ locals, params }: RequestEvent) => {
		if (!params.post_id) {
			throw error(400, 'post_id is required');
		}

		let input = {
			published_at: null
		} satisfies ChangePostInput;

		let post = await BlogsService.changePost({
			postId: params.post_id,
			requestBody: input
		});

		return {
			post,
			success: true,
			message: 'Post successfuly unpublished.'
		};
	}
};
