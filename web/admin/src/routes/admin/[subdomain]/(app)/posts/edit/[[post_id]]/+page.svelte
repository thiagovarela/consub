<script lang="ts">
	import Quill from 'quill'
	import { onMount } from 'svelte'
	import type { PageData } from './$types'

	export let data: PageData

	let post = data.post

	let allowedLocales = ['pt-PT', 'pt-BR', 'en-US', 'es-ES', 'fr-FR']
	let localeNames = new Intl.DisplayNames(['en'], { type: 'language' })

	let editor: Element

	export let toolbarOptions = [
		[{ header: 1 }, { header: 2 }, 'blockquote', 'link', 'image', 'video'],
		['bold', 'italic', 'underline', 'strike'],
		[{ list: 'ordered' }, { list: 'ordered' }],
		[{ align: [] }],
		['clean']
	]

	onMount(async () => {
		let quill = new Quill(editor, {
			modules: {
				toolbar: toolbarOptions
			},
			theme: 'snow',
			placeholder: 'Write your story...'
		})
		quill.setText(data.post.body)
	})
</script>

<svelte:head>
	<meta
		name="viewport"
		content="width=device-width, minimum-scale=1.0, maximum-scale = 1.0, user-scalable = no"
	/>
</svelte:head>

<div class="py-6 px-4 sm:px-6 lg:px-8">
	<div class="mx-auto">
		{#if data.post.id}
			<h1 class="text-2xl font-semibold text-gray-900">Edit {data.post.title}</h1>
		{:else}
			<h1 class="text-2xl font-semibold text-gray-900">Add a new post</h1>
		{/if}
		<div class="pt-5">
			<div class="flex justify-end">
				<button
					type="button"
					class="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
					>Cancel</button
				>
				<button
					type="submit"
					name="save"
					class="ml-3 inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
					>Save</button
				>
				<button
					type="submit"
					name="publish"
					class="ml-3 inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
					>Publish</button
				>
			</div>
		</div>
	</div>
	<div class="mt-8 flex flex-col">
		<form class="space-y-8 divide-y divide-gray-200">
			<div class="space-y-8 divide-y divide-gray-200">
				<div class="pt-8">
					<div>
						<h3 class="text-lg font-medium leading-6 text-gray-900">Post Content</h3>
						<p class="mt-1 text-sm text-gray-500">
							Use a permanent address where you can receive mail.
						</p>
					</div>
					<div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
						<div class="sm:col-span-3">
							<label for="first-name" class="block text-sm font-medium text-gray-700"
								>Title</label
							>
							<div class="mt-1">
								<input
									type="text"
									name="first-name"
									id="first-name"
									autocomplete="given-name"
									class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
								/>
							</div>
						</div>

						<div class="sm:col-span-3">
							<label for="locale" class="block text-sm font-medium text-gray-700"
								>Language</label
							>
							<div class="mt-1">
								<select
									id="locale"
									name="locale"
									autocomplete="locale"
									class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
								>
									{#each allowedLocales as locale}
										<option value={locale}>{localeNames.of(locale)}</option>
									{/each}
								</select>
							</div>
						</div>						

						<div class="sm:col-span-2">
							<div class="relative mt-5 flex items-start">
								<div class="flex h-5 items-center">
									<input
										id="featured"
										name="featured"
										type="checkbox"
										class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
									/>
								</div>
								<div class="ml-3 text-sm">
									<label for="candidates" class="font-medium text-gray-700"
										>Featured</label
									>
									<p class="text-gray-500">Mark this as featured</p>
								</div>
							</div>
						</div>

						<div class="sm:col-span-6">
							<div class="editor-wrapper">
								<div bind:this={editor} />
							</div>
						</div>

						
					</div>
				</div>
			</div>
		</form>
	</div>
</div>

<style>
	@import 'quill/dist/quill.snow.css';
</style>
