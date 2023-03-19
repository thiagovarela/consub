<script lang="ts">
	import BasePage from '$lib/ui/BasePage.svelte';
	import type { PageData } from './$types';
	import { page } from '$app/stores';

	export let data: PageData;
</script>

<BasePage>
	<span slot="header-left">
		<div class="flex items-center">
			<a href="{$page.url.pathname}/../" class="flex">
				<h1 class="text-sm font-medium text-gray-900">Clippings</h1>
			</a>
		</div>
	</span>
	<span slot="header-middle">
		<h1 class="text-xl font-semibold text-gray-900">Categories</h1>
	</span>
	<span slot="header-actions">
		<a
			href="{$page.url.pathname}/edit"
			class="border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
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
						Name
					</th>
					<th
						scope="col"
						class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
						Language
					</th>
				</tr>
			</thead>
			{#each data.items as item}
				<tr class="even:bg-white odd:bg-gray-50">
					<td
						class="px-4 py-3.5 whitespace-nowrap text-sm font-medium text-slate-900 sm:px-6">
						<a
							href="{$page.url.pathname}/edit/{item.id}"
							class="text-slate-600 hover:text-gray-900">
							{item.name}
						</a>
					</td>
					<td class="px-3 py-3.5 whitespace-nowrap text-sm text-gray-500">
						{item.locale}
					</td>
				</tr>
			{/each}
		</table>
	</span>
</BasePage>
