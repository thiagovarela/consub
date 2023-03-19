import { MediaService } from '$lib/api';
import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET = (async ({ request }) => {
	let images = await MediaService.listImages({ take: 50 });
	return json(images);
}) satisfies RequestHandler;
