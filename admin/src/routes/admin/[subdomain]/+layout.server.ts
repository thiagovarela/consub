import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load = (async ({ cookies, url, params }) => {
	const sessionid = cookies.get('sessionid');
	if (url.pathname === `/admin/${params.subdomain}`) {
		if (!sessionid) {
			throw redirect(307, `/admin/${params.subdomain}/login`);
		} else {
			throw redirect(307, `/admin/${params.subdomain}/dashboard`);
		}
	}
}) satisfies LayoutServerLoad;
