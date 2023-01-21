import { authenticated } from '$lib/api'
import type { RequestEvent } from '../$types'
import type { PageLoad } from './$types'

export const load = (async ({ cookies, url }: RequestEvent) => {
	let posts = authenticated(cookies, url).url('/blogs/admin/posts').get().json()

	return {
		posts
	}
}) satisfies PageLoad
