import { AccountsService } from '$lib/api';
import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import crypto from 'crypto';

export const load = (async ({ cookies, locals }) => {
	const sessionid = cookies.get('sessionid');

	if (!sessionid) {
		throw redirect(303, './login');
	}

	let user = await AccountsService.getUserProfile();

	let gravatar = crypto.createHash('md5').update(user.email).digest('hex');

	return {
		gravatar
	};
}) satisfies LayoutServerLoad;
