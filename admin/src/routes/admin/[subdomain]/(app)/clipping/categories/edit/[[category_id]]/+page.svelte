<script lang="ts">
	import Breadcrumb from '$lib/ui/Breadcrumb.svelte';
	import type { ActionData, PageData } from './$types';
	import { page } from '$app/stores';
	import Svelecte from 'svelecte';
	import { locales } from '$lib/locales';
	import { enhance } from '$app/forms';
	import { notifier } from '$lib/notifier';

	export let data: PageData;
	export let form: ActionData;

	let localeData: SelectOption[] = locales(data.languages);

	$: url = `/admin/${$page.params.subdomain}/clipping/categories`;

	let breadcrumbs = [
		{
			href: `/admin/${$page.params.subdomain}/clipping`,
			text: 'Clippings'
		},
		{
			href: `/admin/${$page.params.subdomain}/clipping/categories`,
			text: 'Categories'
		},
		{
			href: $page.url.toString(),
			text: 'Edit Category'
		}
	];

	$: if (form?.success) {
		notifier.success(form.message);
	}
</script>

<div class="relative px-6 py-6">
	<form
		class="space-y-8 divide-y divide-gray-200"
		method="POST"
		use:enhance={() => {
			return ({ update }) => {
				update({ reset: false });
			};
		}}>
		<div class="space-y-8 divide-y divide-gray-200">
			<div>
				<div>
					<Breadcrumb links={breadcrumbs} />
				</div>
			</div>

			<div class="pt-8">
				<div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4">
					<div class="sm:col-span-3">
						<label for="name" class="block text-sm font-medium text-gray-700">
							Name
						</label>
						<div class="mt-1">
							<input
								type="text"
								name="name"
								required
								value={form?.item?.name ?? data?.item?.name ?? ''}
								id="name"
								class="block w-full rounded-md border-gray-300 shadow-sm focus:border-emerald-500 focus:ring-emerald-500 sm:text-sm" />
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
			</div>
		</div>

		<div class="pt-5">
			<div class="flex justify-end">
				<button
					type="button"
					class="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2">
					Cancel
				</button>
				<button
					type="submit"
					formaction={data.item?.id ? '?/update' : '?/create'}
					class="ml-3 inline-flex justify-center rounded-md border border-transparent bg-emerald-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2">
					Save
				</button>
			</div>
		</div>
	</form>
</div>
