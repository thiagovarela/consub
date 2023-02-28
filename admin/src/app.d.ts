import type { ConsubAPI } from '$lib/api/api';
// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	export type Nullable<T> = T | null;
	namespace App {
		// interface Error {}
		interface Locals {
			api: ConsubAPI;
			headerLanguage: string;
			subdomain?: string;
		}
		// interface PageData {}
		// interface Platform {}
	}

	interface SelectOption {
		label: string;
		value: string;
	}
}

export {};
