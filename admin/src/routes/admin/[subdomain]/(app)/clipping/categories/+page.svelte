<script lang="ts">
	import ListPage from '$lib/ui/ListPage.svelte';
	import type { PageData } from './$types';
	import { page } from '$app/stores';

	export let data: PageData;

	$: url = `/admin/${$page.params.subdomain}/clipping/categories`;
</script>

<ListPage name="category" namePlural="categories" editUrl="{url}/edit">
	<span slot="table">
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
						class="px-4 py-3.5 whitespace-nowrap text-sm font-medium text-gray-900 sm:px-6">
						<a
							href="{url}/edit/{item.id}"
							class="text-emerald-600 hover:text-emerald-900">
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
</ListPage>
