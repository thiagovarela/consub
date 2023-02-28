<script lang="ts">
	import { goto } from '$app/navigation';
	import type { PageData } from './$types';

	export let data: PageData;

	let localeNames = new Intl.DisplayNames(['en'], { type: 'language' });
</script>

<div class="py-6 px-4 sm:px-6 lg:px-8">
	<div class="sm:flex sm:items-center">
		<div class="sm:flex-auto">
			<h1 class="text-xl font-semibold text-gray-900">Posts</h1>
			<p class="mt-2 text-sm text-gray-700">A list of all the posts in your account.</p>
		</div>
		<div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
			<button
				type="button"
				on:click={() => goto('posts/edit')}
				class="inline-flex items-center justify-center  border border-transparent bg-emerald-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2 sm:w-auto">
				Add post
			</button>
		</div>
	</div>
	<div class="mt-8 flex flex-col">
		<div class="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
			<div class="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
				<div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
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
								<th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">
									<span class="sr-only">Edit</span>
								</th>
							</tr>
						</thead>
						<tbody class="bg-white">
							{#each data.posts as post}
								<tr>
									<td
										class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
										<a
											class="text-emerald-600 hover:text-emerald-900"
											href="posts/edit/{post.id}">
											{post.title}
										</a>
									</td>
									<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
										{post.published_at ?? 'Not Published'}
									</td>
									<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
										{localeNames.of(post.locale)}
									</td>
									<td
										class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
										<a
											href="posts/edit/{post.id}"
											class="text-emerald-600 hover:text-emerald-900">
											Edit
											<span class="sr-only">, {post.title}</span>
										</a>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</div>
