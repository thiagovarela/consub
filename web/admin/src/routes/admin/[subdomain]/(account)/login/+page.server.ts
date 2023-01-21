import { error, redirect } from '@sveltejs/kit'
import type { RequestHandler, RequestEvent } from './$types'

import { client } from '$lib/api'

import { env } from '$env/dynamic/private'

export const actions = {
	login: async ({ request, cookies, params, url }: RequestEvent) => {
		const values = await request.formData()
		const email = values.get('email')
		const password = values.get('password')

		let api = client(url)

		const response: { token: string; access_key_id: string } = await api
			.url(`/accounts/users/access-tokens/passwords`)
			.post({ email, password })
			.json()

		console.log(response)

		if (response.token) {
			const prefix = `/admin/${params.subdomain}`

			let secure = true
			let sameSite = 'strict'
			let maxAge = 60 * 60 * 24 * 7

			if (env.NODE_ENV == 'development') {
				secure = false
				sameSite = 'lax'
				maxAge = 60 * 60 * 24 * 30
			}

			cookies.set('sessionid', response.token, {
				path: prefix,
				httpOnly: true,
				sameSite,
				secure,
				maxAge
			})

			cookies.set('sessionkey', response.access_key_id, {
				path: prefix,
				httpOnly: true,
				sameSite,
				secure,
				maxAge
			})

			console.log('redirecting to dashboard')

			throw redirect(307, `${prefix}/dashboard`)
		}

		throw error(401, 'Invalid credentials')
	},
	logout: async ({ cookies, params }) => {
		cookies.delete('sessionid')
		cookies.delete('sessionkey')
		const prefix = `/admin/${params.sudomain}`
		throw redirect(307, `${prefix}/login`)
	}
} satisfies RequestHandler
