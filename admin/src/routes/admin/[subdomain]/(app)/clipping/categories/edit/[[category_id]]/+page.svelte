<script lang="ts">
	import BasePage from '$lib/ui/BasePage.svelte';
	import type { ActionData, PageData } from './$types';
	import { page } from '$app/stores';
	import Svelecte from 'svelecte';
	import { locales } from '$lib/locales';
	import { enhance } from '$app/forms';
	import { notifier } from '$lib/notifier';

	export let data: PageData;
	export let form: ActionData;

	let localeData: SelectOption[] = locales(data.languages);

	console.log($page);

	$: if (form?.success) {
		notifier.success(form.message);
	}
</script>

<form
	class="space-y-8 divide-y divide-gray-200"
	method="POST"
	use:enhance={() => {
		return ({ update }) => {
			update({ reset: false });
		};
	}}>
	<BasePage>
		<span slot="header-left">
			<div class="flex items-center">
				<a href="{$page.url.pathname}/../" class="flex">
					<h1 class="text-sm font-medium text-gray-900">Categories</h1>
				</a>
			</div>
		</span>
		<span slot="header-middle">
			<h1 class="text-xl font-semibold text-gray-900">Edit category</h1>
		</span>
		<span slot="header-actions">
			<button
				type="button"
				class="border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
				Cancel
			</button>
			<button
				type="submit"
				formaction={data.item?.id ? '?/update' : '?/create'}
				class="ml-4 border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
				Save
			</button>
		</span>
		<span slot="main">
			<div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4">
				<div class="sm:col-span-3">
					<label for="name" class="block text-sm font-medium text-gray-700">Name</label>
					<div class="mt-1">
						<input
							type="text"
							name="name"
							required
							value={form?.item?.name ?? data?.item?.name ?? ''}
							id="name"
							class="block w-full rounded-md border-gray-300 shadow-sm focus:border-gray-500 focus:ring-gray-500 sm:text-sm" />
					</div>
				</div>

				<div class="sm:col-span-3">
					<label for="locale" class="block text-sm font-medium text-gray-700">
						Language & Region
					</label>
					<div class="mt-1">
						<Svelecte
							name="locale"
							inputId="locale"
							required
							value={form?.item?.locale ?? data?.item?.locale ?? ''}
							options={localeData}
							placeholder="Language & Region" />
					</div>
				</div>
			</div>
		</span>
	</BasePage>
</form>
