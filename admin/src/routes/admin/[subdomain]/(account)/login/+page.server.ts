import { error, redirect } from '@sveltejs/kit';
import type { RequestEvent } from './$types';

import { env } from '$env/dynamic/private';
import { AccountsService } from '$lib/api/services/AccountsService';

export const actions = {
	login: async ({ request, cookies, params, locals }: RequestEvent) => {
		const values = await request.formData();
		const email = values.get('email')?.toString();
		const password = values.get('password')?.toString();

		if (!email || !password) {
			throw error(400, 'Credentials are required');
		}

		let accessToken = await AccountsService.getAccessTokenWithPassword({
			requestBody: { email, password }
		});

		if (accessToken) {
			const prefix = `/admin/${params.subdomain}`;

			let secure = true;
			let sameSite = 'strict';
			let maxAge = 60 * 60 * 24 * 7;

			if (env.NODE_ENV == 'development') {
				secure = false;
				sameSite = 'lax';
				maxAge = 60 * 60 * 24 * 30;
			}

			cookies.set('sessionid', accessToken.token, {
				path: prefix,
				httpOnly: true,
				sameSite: sameSite as any,
				secure,
				maxAge
			});

			console.log(`redirecting to ${prefix}/dashboard`);

			throw redirect(303, `${prefix}/dashboard`);
		}

		throw error(401, 'Invalid credentials');
	},
	logout: async ({ cookies, params }: RequestEvent) => {
		cookies.delete('sessionid');
		const prefix = `/admin/${params.subdomain}`;
		throw redirect(303, `${prefix}/login`);
	}
};
