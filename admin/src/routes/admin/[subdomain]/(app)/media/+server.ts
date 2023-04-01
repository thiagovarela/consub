import { OpenAPI } from '$lib/api';
import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const POST = (async ({ request, fetch }) => {
	let incoming = await request.formData();

	// TODO: use streams

	// FIXME: using axios is not working for some reason, for now we use fetch
	let response = await fetch(`${OpenAPI.BASE}/admin/media/images`, {
		method: 'POST',
		headers: {
			Authorization: `Bearer ${OpenAPI.TOKEN}`
		},
		body: incoming
	});
	const resp = await response.json();

	return json(resp);
}) satisfies RequestHandler;
