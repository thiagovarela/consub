import { redirect } from '@sveltejs/kit'
import type { LayoutServerLoad } from './$types'

import { authenticated } from '$lib/api'

export const load = (async ({ cookies, url, params }) => {
	
    const sessionid = cookies.get('sessionid')
    
    if(url.pathname === `/admin/${params.subdomain}`) {
        if (!sessionid) {
            throw redirect(307, `./${params.subdomain}/login`)
        } else {
            throw redirect(307, `./${params.subdomain}/dashboard`)            
        }
    }
	
}) satisfies LayoutServerLoad
