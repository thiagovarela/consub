import type { ParamMatcher } from '@sveltejs/kit';

export const match = ((param) => {
	return /^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$/.test(param);
}) satisfies ParamMatcher;
