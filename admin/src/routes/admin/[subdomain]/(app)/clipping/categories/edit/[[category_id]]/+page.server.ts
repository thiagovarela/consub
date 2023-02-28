import { ClippingsService } from '$lib/api';
import { parse } from '$lib/locales';
import { error, fail } from '@sveltejs/kit';
import type { PageServerLoad, RequestEvent } from './$types';

export const load = (async ({ params, request }) => {
	let languages = parse(request.headers.get('accept-language')).map(
		(l: { code: any; region: any }) => `${l.code}-${l.region}`
	);

	if (!params.category_id)
		return {
			languages
		};
	let category = ClippingsService.getCategoryById({ categoryId: params.category_id });
	return {
		item: category,
		languages
	};
}) satisfies PageServerLoad;

export const actions = {
	create: async ({ request }: RequestEvent) => {
		const values = await request.formData();
		const name = values.get('name')?.toString();
		const locale = values.get('locale')?.toString();

		if (!name || !locale) {
			return fail(400, { message: 'Missing required fields' });
		}

		let item = await ClippingsService.createCategory({ requestBody: { name, locale } });
		return {
			item,
			success: true,
			message: 'Category successfully created.'
		};
	},

	update: async ({ request, params }: RequestEvent) => {
		const values = await request.formData();
		const name = values.get('name')?.toString();
		const locale = values.get('locale')?.toString();

		if (!params.category_id) {
			throw error(400, 'Missing required fields');
		}

		let item = await ClippingsService.changeCategory({
			categoryId: params.category_id,
			requestBody: { name, locale }
		});

		return {
			item,
			success: true,
			message: 'Category successfully updated.'
		};
	}
};
