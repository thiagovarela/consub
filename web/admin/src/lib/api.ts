import type { Cookies } from '@sveltejs/kit'
import wretch from 'wretch'
import { env } from '$env/dynamic/private'

export const api = wretch().url(env.API_ENDPOINT)

export const client = (url: URL) => {
	if (env.TEST_DOMAIN) {
		const client = api.headers({ 'X-Forwarded-Host': env.TEST_DOMAIN })
		return client
	}
	const client = api.headers({ 'X-Forwarded-Host': url.hostname })
	return client
}

export const authenticated = (cookies: Cookies, url: URL) => {
	const sessionid = cookies.get('sessionid')
	const sessionkey = cookies.get('sessionkey') || ''

	return client(url).headers({
		Authorization: `Bearer ${sessionid}`,
		'X-Api-Key': sessionkey
	})
}
