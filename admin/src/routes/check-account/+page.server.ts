import { error, redirect } from '@sveltejs/kit';
import type { RequestEvent } from './$types';

export const actions = {
	default: async ({ request }: RequestEvent) => {
		const values = await request.formData();
		const subdomain = values.get('subdomain')?.toString();

		if (!subdomain) {
			throw error(400, 'Subdomain is required');
		}

		throw redirect(303, `/admin/${subdomain.toLowerCase()}/dashboard`);
	}
};
