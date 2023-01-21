import { authenticated } from '$lib/api'
import type { RequestEvent } from '../$types'
import type { PageLoad } from './$types'

export const load = (async ({ cookies, url, params }: RequestEvent) => {
	if (!params.post_id) {
		return {
			post: {}
		}
	}

	let post = await authenticated(cookies, url)
		.url(`/blogs/admin/posts/${params.post_id}`)
		.get()
		.json()

	return {
		post
	}
}) satisfies PageLoad
