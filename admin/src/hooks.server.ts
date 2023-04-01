import type { Handle, HandleServerError } from '@sveltejs/kit';

import { getHeaderLanguage } from '$lib/locales';

import { logger } from '$lib/logger';
import { OpenAPI } from '$lib/api/core/OpenAPI';
import { env } from '$env/dynamic/private';

export const handle = (async ({ event, resolve }) => {
	let subdomain = event.params.subdomain;
	let cookies = event.cookies;
	OpenAPI.BASE = env.API_ENDPOINT;
	// TODO: This is a hack to get the subdomain to work with the API
	// Or maybe not, I'm not sure I'll actually use subdomains
	if (subdomain) {
		OpenAPI.HEADERS = {
			'X-Forwarded-Host': `${subdomain}.consub.io`,
			'X-Forwarded-App': `admin`
		};
	}
	if (cookies) {
		OpenAPI.TOKEN = cookies.get('sessionid');
	}
	event.locals.subdomain = event.params.subdomain;
	event.locals.headerLanguage = getHeaderLanguage(event.request.headers.get('accept-language'));

	const response = await resolve(event);

	return response;
}) satisfies Handle;

export const handleError = (({ error, event }) => {
	console.log(error);
	// logger.error(
	// 	`Request ${event.request.method} ${event.request.url} failed with ${error?.code ?? ''} ${
	// 		error?.message ?? ''
	// 	}`
	// );
	return {
		status: error.code ?? 500,
		message: error.message
	};
}) satisfies HandleServerError;
