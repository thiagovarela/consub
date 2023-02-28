import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load = (async ({ cookies, locals }) => {
	const sessionid = cookies.get('sessionid');

	if (!sessionid) {
		throw redirect(303, './login');
	}
}) satisfies LayoutServerLoad;
