<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData, ActionData } from './$types';
	import Tiptap from '$lib/Tiptap.svelte';
	import Svelecte from 'svelecte';
	import { locales } from '$lib/locales';
	import Toggle from '$lib/ui/forms/Toggle.svelte';
	import { notifier } from '$lib/notifier';
	import { goto } from '$app/navigation';
	import BasePage from '$lib/ui/BasePage.svelte';
	import { page } from '$app/stores';

	export let data: PageData;
	export let form: ActionData;

	let tiptap: Tiptap;

	let initialContent = {
		type: 'doc',
		content: [
			{
				type: 'paragraph',
				content: [
					{
						type: 'text',
						text: 'Start typing...'
					}
				]
			}
		]
	};
	if (data?.post?.body_json) {
		initialContent = data.post.body_json;
	}

	let bodyJson: string = JSON.stringify(initialContent);
	let bodyText: string = data?.post?.body_text ?? '';
	let bodyHtml: string = data?.post?.body_html ?? '';

	let localeData: SelectOption[] = locales(data.languages);
	let selectedLocale: SelectOption | undefined;

	$: categoryData = data.categories
		.filter((category) => {
			let locale = selectedLocale?.value ?? data?.post?.locale;
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
<form
	method="POST"
	use:enhance={() => {
		return ({ update }) => {
			update({ reset: false });
		};
	}}>
	<input type="hidden" name="body_json" bind:value={bodyJson} />
	<input type="hidden" name="body_html" bind:value={bodyHtml} />
	<input type="hidden" name="body_text" bind:value={bodyText} />
	<BasePage>
		<span slot="header-left">
			<div class="flex posts-center">
				<a href="{$page.url.pathname}/../" class="flex">
					<h1 class="text-sm font-medium text-gray-900">Posts</h1>
				</a>
			</div>
		</span>
		<span slot="header-middle">
			<h1 class="text-xl font-semibold text-gray-900">Add post</h1>
		</span>
		<span slot="header-actions">
			<button
				type="button"
				on:click={() => goto($page.url.pathname + '/../')}
				class="border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
				Cancel
			</button>
			{#if data?.post?.id && !data?.post?.published_at}
				<input type="hidden" name="published_at" value={new Date().toISOString()} />
				<button
					formaction="?/publish"
					class="ml-4 border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
					Publish
				</button>
			{/if}
			{#if data?.post?.id && data?.post?.published_at}
				<button
					formaction="?/unpublish"
					class="ml-4 border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
					Unpublish
				</button>
			{/if}
			<button
				formaction={!data?.post ? '?/create' : '?/update'}
				class="ml-4 inline-flex border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
				Save
			</button>
		</span>
		<span slot="main">
			<div class="flex flex-1 flex-col justify-between">
				<div class="space-y-6 pt-6 pb-5">
					<div>
						<label for="title" class="block text-sm font-medium text-gray-900">
							Title
						</label>
						<div class="mt-1">
							<input
								type="text"
								required
								name="title"
								value={form?.post?.title ?? data?.post?.title ?? ''}
								id="title"
								class="block w-full rounded-md border-gray-300 shadow-sm focus:border-slate-500 focus:ring-slate-500 sm:text-sm" />
						</div>
					</div>					
					<div>
						<label for="is_featured" class="block text-sm font-medium text-gray-900">
							Featured
						</label>
						<div class="mt-1">
							<Toggle
								name="is_featured"
								checked={form?.post?.is_featured ??
									data?.post?.is_featured ??
									false} />
						</div>
					</div>

					<div class="flex-1 overflow-y-auto">
						<div
							aria-labelledby="primary-heading"
							class="flex h-full flex-1 flex-col place-posts-center">
							<Tiptap
								bind:contentJsonString={bodyJson}
								bind:contentHtml={bodyHtml}
								bind:contentText={bodyText}
								{initialContent}
								bind:this={tiptap} />
						</div>
					</div>

					<div class="lg:w-1/2">
						<label for="url_slug" class="block text-sm font-medium text-gray-900">
							URL Slug
						</label>
						<div class="mt-1">
							<input
								type="text"
								name="slug"
								value={form?.post?.slug ?? data?.post?.slug ?? ''}
								id="slug"
								class="block w-full rounded-md border-gray-300 shadow-sm focus:border-slate-500 focus:ring-slate-500 sm:text-sm" />
						</div>
					</div>
					<div class="lg:w-1/2">
						<label
							for="short_description"
							class="block text-sm font-medium text-gray-900">
							Short Description
						</label>
						<div class="mt-1">
							<textarea
								id="short_description"
								name="short_description"
								value={form?.post?.short_description ??
									data?.post?.short_description ??
									''}
								rows="4"
								class="block w-full rounded-md border-gray-300 shadow-sm focus:border-slate-500 focus:ring-slate-500 sm:text-sm" />
						</div>
					</div>

					<div class="lg:w-1/2">
						<label for="locale" class="mt-1 block text-sm font-medium text-gray-700">
							Language & Region
						</label>
						<div class="mt-1">
							<Svelecte
								name="locale"
								inputId="locale"
								bind:readSelection={selectedLocale}
								value={form?.post?.locale ?? data?.post?.locale ?? ''}
								options={localeData}
								placeholder="Language & Region" />
						</div>
					</div>

					<div class="lg:w-1/2">
						<label for="category" class="mt-1 block text-sm font-medium text-gray-700">
							Category
						</label>
						<div class="mt-1">
							<Svelecte
								name="category_id"
								inputId="category_id"
								value={form?.post?.category_id ?? data?.post?.category_id ?? ''}
								options={categoryData}
								placeholder="Category"
								creatable={true} />
						</div>
					</div>
				</div>
			</div>
		</span>
	</BasePage>
</form>
