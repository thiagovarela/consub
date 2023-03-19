import { MediaService } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async () => {
	let images = await MediaService.listImages({ take: 50 });

	return {
		images,
		pageTitle: 'Media Library'
	};
}) satisfies PageServerLoad;
