<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import type { PageData } from './$types';
	import ListPage from '$lib/ui/ListPage.svelte';
	import BasePage from '$lib/ui/BasePage.svelte';

	export let data: PageData;

	$: url = `/admin/${$page.params.subdomain}/clipping`;

	let localeNames = new Intl.DisplayNames(['en'], { type: 'language' });
</script>

<BasePage>
	<span slot="header-left">
		<h1 class="text-xl font-semibold text-gray-900">Clippings</h1>
	</span>
	<span slot="header-actions">
		<a
			href="{url}/categories"
			class="border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
			Manage Categories
		</a>
		<a
			href="{url}/edit"
			class="ml-4 border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
			Create
		</a>
	</span>
	<span slot="main">
		<table class="min-w-full divide-y divide-gray-300">
			<thead class="bg-gray-50">
				<tr>
					<th
						scope="col"
						class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">
						Title
					</th>
					<th
						scope="col"
						class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
						Published Date
					</th>
					<th
						scope="col"
						class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
						Language
					</th>
					<th
						scope="col"
						class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
						Category
					</th>
				</tr>
			</thead>
			<tbody class="bg-white">
				{#each data.items as item}
					<tr class="even:bg-white odd:bg-gray-50">
						<td
							class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
							<a
								class="text-slate-600 hover:text-slate-900"
								href="clipping/edit/{item.id}">
								{item.title}
							</a>
						</td>
						<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
							{item.published_at ?? 'Not Published'}
						</td>
						<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
							{localeNames.of(item.locale)}
						</td>
						<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
							{data.categories.find((c) => c.id === item.category_id)?.name ??
								'Uncategorized'}
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</span>
</BasePage>
