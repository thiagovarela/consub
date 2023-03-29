<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData, ActionData } from '../$types';
	import Tiptap from '$lib/Tiptap.svelte';
	import Svelecte from 'svelecte';
	import { DateInput } from 'date-picker-svelte';
	import { locales } from '$lib/locales';
	import Toggle from '$lib/ui/forms/Toggle.svelte';
	import { notifier } from '$lib/notifier';
	import Modal from '$lib/ui/Modal.svelte';
	import ImageSet from '$lib/ui/images/ImageSet.svelte';

	export let data: PageData;
	export let form: ActionData;

	let tiptap: Tiptap;

	let title: string | undefined = form?.post?.title ?? data?.post?.title ?? undefined;

	let bodyJson: string | undefined = data?.post?.body_json
		? JSON.stringify(data.post.body_json)
		: undefined;
	let bodyText: string | undefined = data?.post?.body_text ?? undefined;
	let bodyHtml: string | undefined = data?.post?.body_html ?? undefined;

	let publishedAt = form?.post?.published_at ?? data?.post?.published_at ?? null;
	if (publishedAt) {
		publishedAt = new Date(publishedAt);
	}

	$: canSave = title && bodyText;

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

	let showFeatureImageModal = false;
	let selectedImageId: string | undefined;
	let selectedImage: { image_set: any };
	let postFearuredImageId: string | undefined = data?.featuredImage?.id ?? undefined;

	if (data?.featuredImageMedia) {
		selectedImage = data.featuredImageMedia;
		selectedImageId = data.featuredImageMedia.id;
	}

	function onFeatureImageSelect(event: {
		currentTarget: { dataset: { imageId: string | undefined } };
	}) {
		showFeatureImageModal = false;
		selectedImageId = event.currentTarget.dataset.imageId;
		selectedImage = data.recentlyUploaded.find((image) => image.id === selectedImageId);
	}
</script>

<style lang="postcss">
	.details-open {
		@apply transform transition ease-in-out duration-500 sm:duration-700 translate-x-0;
	}
	.details-closed {
		@apply transform transition ease-in-out duration-500 sm:duration-700 translate-x-full;
	}
	:root {
		--date-input-width: 'auto';
	}
</style>

<svelte:head>
	<meta
		name="viewport"
		content="width=device-width, minimum-scale=1.0, maximum-scale = 1.0, user-scalable = no" />
</svelte:head>

<div class="mx-auto">
	<!-- Page header -->
	<div
		class="mx-auto md:flex md:items-center md:justify-between md:space-x-5 lg:px-8 px-4 sm:px-6 ">
		<div class="flex items-center space-x-5">
			<!-- <div class="flex-shrink-0">
				<div class="relative">
					<img
						class="h-16 w-16 rounded-full"
						src="https://images.unsplash.com/photo-1463453091185-61582044d556?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
						alt="" />
					<span class="absolute inset-0 rounded-full shadow-inner" aria-hidden="true" />
				</div>
			</div> -->
			<div>
				<h1 class="text-2xl font-bold text-gray-900">Posts</h1>
				{#if data?.post?.published_at}
					<p class="text-sm font-medium text-gray-500">
						Published by <a href="#" class="text-gray-900">Author here</a>
						on
						<time datetime="2020-08-25">{data?.post.published_at}</time>
					</p>
				{/if}
			</div>
		</div>
		<!-- <div
			class="justify-stretch mt-6 flex flex-col-reverse space-y-4 space-y-reverse sm:flex-row-reverse sm:justify-end sm:space-y-0 sm:space-x-3 sm:space-x-reverse md:mt-0 md:flex-row md:space-x-3">
			<button
				type="button"
				class="inline-flex items-center justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50">
				Disqualify
			</button>
			<button
				type="button"
				class="inline-flex items-center justify-center rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600">
				Advance to offer
			</button>
		</div> -->
	</div>
	<form
		method="POST"
		use:enhance={() => {
			return ({ update }) => {
				update({ reset: false });
			};
		}}>
		<div class="mt-8 grid grid-cols-1 gap-6 px-6 lg:grid-flow-col-dense lg:grid-cols-4">
			<div class="space-y-6 lg:col-span-3 lg:col-start-1">
				<ul>
					{#if selectedImage}
						<li class="mb-4 max-w-lg">
							<ImageSet class="rounded" imageSet={selectedImage.image_set} />
							<input
								type="hidden"
								name="featured_post_image_id"
								bind:value={postFearuredImageId} />
							<input
								type="hidden"
								name="featured_image_id"
								bind:value={selectedImageId} />
						</li>
					{:else}
						<li>
							<button
								type="button"
								on:click={() => (showFeatureImageModal = true)}
								class="sm:py-1.5 text-gray-400">
								Add feature image
							</button>
						</li>
					{/if}

					<li>
						<div class="overflow-hidden rounded-lg ring-none">
							<label for="title" class="sr-only">Post title</label>
							<textarea
								rows="2"
								bind:value={title}
								name="title"
								id="title"
								class="pl-0 w-full resize-none border-0 bg-transparent text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:leading-6 text-4xl"
								placeholder="Post title..." />
						</div>
					</li>
					<li>
						<div class="flex-1 overflow-y-auto">
							<div
								aria-labelledby="primary-heading"
								class="flex h-full flex-1 flex-col place-items-center">
								<input type="hidden" name="body_json" bind:value={bodyJson} />
								<input type="hidden" name="body_html" bind:value={bodyHtml} />
								<input type="hidden" name="body_text" bind:value={bodyText} />
								<Tiptap
									bind:contentJsonString={bodyJson}
									bind:contentHtml={bodyHtml}
									bind:contentText={bodyText}
									bind:this={tiptap} />
							</div>
						</div>
					</li>
				</ul>
			</div>
			<div class="lg:col-span-2 lg:col-start-4">
				<section aria-labelledby="post-base" class="lg:col-span-1 lg:col-start-2">
					<div class="bg-white px-4 py-5 shadow sm:rounded-lg sm:px-6">
						<h2 class="text-lg font-medium text-gray-900">Post Settings</h2>
						<!-- Form -->
						<div class="mt-6">
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
										value={form?.post?.locale ?? data?.post?.locale ?? ''}
										options={localeData}
										placeholder="Language & Region" />
								</div>
							</div>
							<div class="mt-4">
								<label
									for="published_at"
									class="mt-1 block text-sm font-medium text-gray-700">
									Published At
								</label>
								<div class="mt-1 block">
									<input type="hidden" name="published_at" value={publishedAt} />
									<DateInput
										bind:value={publishedAt}
										placeholder="Choose a date" />
								</div>
							</div>
							<div class="mt-4">
								<label
									for="is_featured"
									class="block text-sm font-medium text-gray-900">
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
							<div class="mt-4">
								<label
									for="url_slug"
									class="block text-sm font-medium text-gray-900">
									URL Slug
								</label>
								<div class="mt-1">
									<input
										type="text"
										name="slug"
										value={form?.post?.slug ?? data?.post?.slug ?? ''}
										id="slug"
										placeholder="Leave it blank to auto-generate"
										class="block w-full rounded-md border-gray-300 shadow-sm focus:border-slate-500 focus:ring-slate-500 sm:text-sm" />
								</div>
								<p class="mt-2 text-sm text-gray-500">
									https://yoursite.com/posts/
								</p>
							</div>
							<div class="mt-4">
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
						</div>
						<div class="justify-stretch mt-6 flex flex-col">
							<button
								type="submit"
								disabled={!canSave}
								formaction={!data?.post ? '?/create' : '?/update'}
								class="inline-flex items-center justify-center rounded-md bg-slate-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-slate-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-25">
								Save
							</button>
						</div>
					</div>
				</section>

				<section aria-labelledby="meta-seo" class="mt-4">
					<div class="bg-white px-4 py-5 shadow sm:rounded-lg sm:px-6">
						<h2 class="text-lg font-medium text-gray-900">Meta & SEO</h2>
						<!-- Form -->
						<div class="mt-6">
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
										value={form?.post?.category_id ??
											data?.post?.category_id ??
											''}
										options={categoryData}
										placeholder="Category"
										creatable={true} />
								</div>
							</div>
							<div class="mt-4">
								<label
									for="meta_title"
									class="block text-sm font-medium text-gray-900">
									Meta Title
								</label>
								<div class="mt-1">
									<input
										type="text"
										name="meta_title"
										value={form?.post?.meta_title ??
											data?.post?.meta_title ??
											''}
										id="meta_title"
										class="block w-full rounded-md border-gray-300 shadow-sm focus:border-slate-500 focus:ring-slate-500 sm:text-sm" />
								</div>
							</div>
							<div class="mt-4">
								<label
									for="meta_description"
									class="block text-sm font-medium text-gray-900">
									Meta Description
								</label>
								<div class="mt-1">
									<textarea
										id="meta_description"
										name="meta_description"
										value={form?.post?.meta_description ??
											data?.post?.meta_description ??
											''}
										rows="2"
										class="block w-full rounded-md border-gray-300 shadow-sm focus:border-slate-500 focus:ring-slate-500 sm:text-sm" />
								</div>
							</div>
							<div class="mt-4">
								<label
									for="meta_keywords"
									class="block text-sm font-medium text-gray-900">
									Meta Keywords
								</label>
								<div class="mt-1">
									<input
										type="text"
										name="meta_keywords"
										value={form?.post?.meta_keywords ??
											data?.post?.meta_keywords ??
											''}
										id="meta_keywords"
										class="block w-full rounded-md border-gray-300 shadow-sm focus:border-slate-500 focus:ring-slate-500 sm:text-sm" />
								</div>
							</div>
						</div>
						<div class="justify-stretch mt-6 flex flex-col">
							<button
								type="button"
								disabled={!canSave}
								formaction={!data?.post ? '?/create' : '?/update'}
								class="inline-flex items-center justify-center rounded-md bg-slate-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-slate-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-25">
								Save
							</button>
						</div>
					</div>
				</section>
			</div>
		</div>
	</form>
</div>

<Modal bind:show={showFeatureImageModal}>
	<div>
		{#await data.recentlyUploaded}
			Loading...
		{:then images}
			<div class="grid grid-cols-2 md:grid-cols-3 gap-4">
				{#each images as image}
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<div on:click={onFeatureImageSelect} data-image-id={image.id}>
						<ImageSet class="h-auto" imageSet={image.image_set} />
					</div>
				{/each}
			</div>
		{/await}
	</div>
</Modal>
