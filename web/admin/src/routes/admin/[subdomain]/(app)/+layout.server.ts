import { redirect } from '@sveltejs/kit'
import type { LayoutServerLoad } from './$types'

import { authenticated } from '$lib/api'

export const load = (async ({ cookies, url }) => {
	const sessionid = cookies.get('sessionid')

	if (!sessionid) {
		throw redirect(307, './login')
	}

	let user = await authenticated(cookies, url).url('/accounts/users/profile').get().json()
	console.log(user)
}) satisfies LayoutServerLoad
