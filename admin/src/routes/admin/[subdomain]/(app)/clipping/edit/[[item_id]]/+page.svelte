<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData, ActionData } from './$types';
	import Tiptap from '$lib/Tiptap.svelte';
	import Svelecte from 'svelecte';
	import { locales } from '$lib/locales';
	import Toggle from '$lib/ui/forms/Toggle.svelte';
	import { notifier } from '$lib/notifier';
	import { goto } from '$app/navigation';

	export let data: PageData;
	export let form: ActionData;

	let tiptap: Tiptap;
	let content: string = JSON.stringify(data?.item?.body);
	let title: string = data?.item?.title ?? 'Title...';
	// input date fields don't support timezones, so we need to strip the timezone
	let source_published_at =
		form?.item?.source_published_at.toString().split('T')[0] ??
		data?.item?.source_published_at?.toString().split('T')[0] ??
		'';
	let initialContent = data?.item?.body ?? {
		type: 'doc',
		content: [
			{
				type: 'heading',
				content: [
					{
						type: 'text',
						text: 'Title...'
					}
				]
			}
		]
	};

	let localeData: SelectOption[] = locales(data.languages);
	let selectedLocale: SelectOption | undefined;

	$: categoryData = data.categories
		.filter((category) => {
			let locale = selectedLocale?.value ?? data?.item?.locale;
			return locale === category.locale;
		})
		.map((category) => {
			return {
				label: category.name,
				value: category.id
			};
		});

	$: if (form?.success) {
		notifier.success(form.message);
	}
</script>

<style lang="postcss">
	.details-open {
		@apply transform transition ease-in-out duration-500 sm:duration-700 translate-x-0;
	}
	.details-closed {
		@apply transform transition ease-in-out duration-500 sm:duration-700 translate-x-full;
	}
</style>

<svelte:head>
	<meta
		name="viewport"
		content="width=device-width, minimum-scale=1.0, maximum-scale = 1.0, user-scalable = no" />
</svelte:head>

<div class="relative">
	<form
		method="POST"
		use:enhance={() => {
			return ({ update }) => {
				update({ reset: false });
			};
		}}>
		<input type="hidden" name="body" bind:value={content} />
		<input type="hidden" name="title" bind:value={title} />

		<div class="flex flex-1">
			<div class="flex-1 border-r border-gray-200 overflow-y-auto">
				<div
					aria-labelledby="primary-heading"
					class="flex h-full flex-1 flex-col place-items-center">
					<Tiptap
						getContent={(jsonContent) => {
							let first = jsonContent.content.at(0);
							if (first.type === 'heading') {
								title = first?.content?.at(0)?.text;
							}
							content = JSON.stringify(jsonContent);
						}}
						{initialContent}
						bind:this={tiptap} />
				</div>
			</div>

			<!-- Secondary column (hidden on smaller screens) -->
			<aside class="hidden h-full  w-96 overflow-y-auto bg-white lg:block">
				<div class="flex flex-col bg-white shadow-xl">
					<div class="flex-1 overflow-y-auto">
						<div class="bg-emerald-700 py-6 px-4 sm:px-6">
							<div class="flex items-center justify-between">
								<h2 class="text-lg font-medium text-white">Settings</h2>
							</div>
							<div class="mt-1">
								<p class="text-sm text-emerald-300" />
							</div>
						</div>
						<div class="flex flex-shrink-0 justify-end px-4 py-4">
							<button
								type="button"
								on:click={() => goto('../')}
								class="border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2">
								Cancel
							</button>
							{#if data?.item?.id && !data?.item?.published_at}
								<input
									type="hidden"
									name="published_at"
									value={new Date().toISOString()} />
								<button
									formaction="?/publish"
									class="ml-4 inline-flex justify-center border border-transparent bg-emerald-600 py-2 px-4 text-sm font-light text-white shadow-sm hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2">
									Publish
								</button>
							{/if}
							{#if data?.item?.id && data?.item?.published_at}
								<button
									formaction="?/unpublish"
									class="ml-4 inline-flex justify-center border border-transparent bg-emerald-600 py-2 px-4 text-sm font-light text-white shadow-sm hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2">
									Unpublish
								</button>
							{/if}
							<button
								formaction={!data?.item ? '?/create' : '?/update'}
								class="ml-4 inline-flex justify-center border border-transparent bg-emerald-600 py-2 px-4 text-sm font-light text-white shadow-sm hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2">
								Save
							</button>
						</div>
						<div class="flex flex-1 flex-col justify-between">
							<div class="px-4 sm:px-6">
								<div class="space-y-6 pt-6 pb-5">
									<div>
										<label
											for="source"
											class="block text-sm font-medium text-gray-900">
											Source
										</label>
										<div class="mt-1">
											<input
												type="text"
												required
												name="source"
												value={form?.item?.source ??
													data?.item?.source ??
													''}
												id="source"
												class="block w-full rounded-md border-gray-300 shadow-sm focus:border-emerald-500 focus:ring-emerald-500 sm:text-sm" />
										</div>
									</div>
									<div>
										<label
											for="source_url"
											class="block text-sm font-medium text-gray-900">
											Source URL
										</label>
										<div class="mt-1">
											<input
												type="text"
												name="source_url"
												required
												value={form?.item?.source_url ??
													data?.item?.source_url ??
													''}
												id="source_url"
												class="block w-full rounded-md border-gray-300 shadow-sm focus:border-emerald-500 focus:ring-emerald-500 sm:text-sm" />
										</div>
									</div>
									<div>
										<label
											for="source_published_at"
											class="block text-sm font-medium text-gray-900">
											Source Published At
										</label>
										<div class="mt-1">
											<input
												type="date"
												required
												name="source_published_at"
												value={source_published_at}
												id="source_published_at"
												class="block w-full rounded-md border-gray-300 shadow-sm focus:border-emerald-500 focus:ring-emerald-500 sm:text-sm" />
										</div>
									</div>
									<div>
										<label
											for="is_featured"
											class="block text-sm font-medium text-gray-900">
											Featured
										</label>
										<div class="mt-1">
											<Toggle
												name="is_featured"
												checked={form?.item?.is_featured ??
													data?.item?.is_featured ??
													false} />
										</div>
									</div>
									<div>
										<label
											for="url_slug"
											class="block text-sm font-medium text-gray-900">
											URL Slug
										</label>
										<div class="mt-1">
											<input
												type="text"
												name="slug"
												value={form?.item?.slug ?? data?.item?.slug ?? ''}
												id="slug"
												class="block w-full rounded-md border-gray-300 shadow-sm focus:border-emerald-500 focus:ring-emerald-500 sm:text-sm" />
										</div>
									</div>
									<div>
										<label
											for="short_description"
											class="block text-sm font-medium text-gray-900">
											Short Description
										</label>
										<div class="mt-1">
											<textarea
												id="short_description"
												name="short_description"
												value={form?.item?.short_description ??
													data?.item?.short_description ??
													''}
												rows="4"
												class="block w-full rounded-md border-gray-300 shadow-sm focus:border-emerald-500 focus:ring-emerald-500 sm:text-sm" />
										</div>
									</div>

									<div>
										<label
											for="locale"
											class="mt-1 block text-sm font-medium text-gray-700">
											Language & Region
										</label>
										<div class="mt-1">
											<Svelecte
												name="locale"
												inputId="locale"
												bind:readSelection={selectedLocale}
												value={form?.item?.locale ??
													data?.item?.locale ??
													''}
												options={localeData}
												placeholder="Language & Region" />
										</div>
									</div>

									<div>
										<label
											for="category"
											class="mt-1 block text-sm font-medium text-gray-700">
											Category
										</label>
										<div class="mt-1">
											<Svelecte
												name="category_id"
												inputId="category_id"
												value={form?.item?.category_id ??
													data?.item?.category_id ??
													''}
												options={categoryData}
												placeholder="Category"
												creatable={true} />
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</aside>
		</div>
	</form>
</div>