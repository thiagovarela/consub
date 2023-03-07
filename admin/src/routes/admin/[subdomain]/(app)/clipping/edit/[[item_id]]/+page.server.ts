import { error, fail } from '@sveltejs/kit';
import type { RequestEvent } from './$types';
import type { PageServerLoad } from './$types';
import { parse } from '$lib/locales';
import {
	ClippingsService,
	type ChangeClippingItemInput,
	type CreateClippingItemInput
} from '$lib/api';
import dayjs from 'dayjs';

export const load = (async ({ locals, params, request }: RequestEvent) => {
	let categories = await ClippingsService.listCategories({});
	let languages = parse(request.headers.get('accept-language')).map(
		(l: { code: any; region: any }) => `${l.code}-${l.region}`
	);

	if (params.item_id) {
		let item = await ClippingsService.getItemById({ itemId: params.item_id });
		return {
			item,
			categories,
			languages
		};
	} else {
		return {
			categories,
			languages
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
		const source = values.get('source')?.toString();
		const source_url = values.get('source_url')?.toString();
		const source_published_at = values.get('source_published_at')?.toString();
		const is_featured = values.get('source_url')?.toString();
		const category_id = values.get('category_id')?.toString();

		if (
			!title ||
			!body_json ||
			!body_html ||
			!body_text ||
			!source ||
			!source_url ||
			!source_published_at ||
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
			source,
			source_url,
			source_published_at: dayjs(source_published_at).format('YYYY-MM-DDTHH:mm:ss'),
			is_featured: is_featured ? true : false,
			category_id
		} satisfies CreateClippingItemInput;

		let item = await ClippingsService.createItem({ requestBody: input });
		return {
			item,
			success: true,
			message: 'Clipping item successfuly created'
		};
	},

	update: async ({ request, params }: RequestEvent) => {
		const values = await request.formData();

		const title = values.get('title')?.toString();
		let body_json = values.get('body_json')?.toString();
		const body_html = values.get('body_html')?.toString();
		const body_text = values.get('body_text')?.toString();
		const source = values.get('source')?.toString();
		const source_url = values.get('source_url')?.toString();
		let source_published_at = values.get('source_published_at')?.toString();
		const is_featured = values.get('is_featured')?.toString();
		const category_id = values.get('category_id')?.toString();

		if (!params.item_id) {
			throw error(400, 'Title and content are required');
		}

		source_published_at = source_published_at
			? dayjs(source_published_at).format('YYYY-MM-DDTHH:mm:ss')
			: undefined;
		console.log(body_json);

		body_json = body_json ? JSON.parse(body_json) : undefined;

		console.log(body_json);

		let input = {
			title,
			body_json,
			body_html,
			body_text,
			source,
			source_url,
			source_published_at,
			is_featured: is_featured ? true : false,
			category_id
		} satisfies ChangeClippingItemInput;

		console.log(input);

		let item = await ClippingsService.changeItem({
			itemId: params.item_id,
			requestBody: input
		});

		return {
			item,
			success: true,
			message: 'Clipping item successfuly updated.'
		};
	},
	publish: async ({ request, params }: RequestEvent) => {
		const values = await request.formData();
		const published_at = values.get('published_at')?.toString();

		if (!params.item_id || !published_at) {
			throw error(400, 'item_id and published_at are required.');
		}

		let input = {
			published_at: dayjs(published_at).format('YYYY-MM-DDTHH:mm:ss')
		} satisfies ChangeClippingItemInput;

		console.log('input', input);

		let item = await ClippingsService.changeItem({
			itemId: params.item_id,
			requestBody: input
		});

		return {
			item,
			success: true,
			message: 'Clipping item successfuly published.'
		};
	},
	unpublish: async ({ locals, params }: RequestEvent) => {
		if (!params.item_id) {
			throw error(400, 'item_id is required');
		}

		let input = {
			published_at: null
		} satisfies ChangeClippingItemInput;

		let item = await ClippingsService.changeItem({
			itemId: params.item_id,
			requestBody: input
		});

		return {
			item,
			success: true,
			message: 'Clipping item successfuly unpublished.'
		};
	}
};
