<script lang="ts">
	import { enhance } from '$app/forms';

	import type { PageData, ActionData } from './$types';

	import Tiptap from '$lib/Tiptap.svelte';

	export let data: PageData;
	export let form: ActionData;

	let post = data.post;

	let tiptap: Tiptap;

	let content: string = JSON.stringify(post.body);
	let title: string = post.title;

	let allowedLocales = ['pt-PT', 'pt-BR', 'en-US', 'es-ES', 'fr-FR'];
	let localeNames = new Intl.DisplayNames(['en'], { type: 'language' });
	let detailsOpen = false;
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

<div class="relative py-6 px-4 sm:px-6 lg:px-8">
	<div class="absolute top-0 right-0 pt-6 pr-6">
		<button on:click={() => (detailsOpen = true)}>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">
				<path fill="none" d="M0 0h24v24H0z" />
				<path
					d="M2 18h7v2H2v-2zm0-7h9v2H2v-2zm0-7h20v2H2V4zm18.674 9.025l1.156-.391 1 1.732-.916.805a4.017 4.017 0 0 1 0 1.658l.916.805-1 1.732-1.156-.391c-.41.37-.898.655-1.435.83L19 21h-2l-.24-1.196a3.996 3.996 0 0 1-1.434-.83l-1.156.392-1-1.732.916-.805a4.017 4.017 0 0 1 0-1.658l-.916-.805 1-1.732 1.156.391c.41-.37.898-.655 1.435-.83L17 11h2l.24 1.196c.536.174 1.024.46 1.434.83zM18 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />
			</svg>
		</button>
	</div>

	<form method="POST" use:enhance>
		<input type="hidden" name="body" bind:value={content} />
		<input type="hidden" name="title" bind:value={title} />

		<div
			class="relative z-10"
			aria-labelledby="slide-over-title"
			role="dialog"
			aria-modal="true">
			<!-- Background backdrop, show/hide based on slide-over state. -->
			<div
				class="fixed inset-0"
				class:hidden={!detailsOpen}
				on:click={() => (detailsOpen = false)} />

			<div class=" overflow-hidden">
				<div class="absolute inset-0 overflow-hidden">
					<div
						class="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10 sm:pl-16">
						<!--
					Slide-over panel, show/hide based on slide-over state.
		  
					Entering: "transform transition ease-in-out duration-500 sm:duration-700"
					  From: "translate-x-full"
					  To: "translate-x-0"
					Leaving: "transform transition ease-in-out duration-500 sm:duration-700"
					  From: "translate-x-0"
					  To: "translate-x-full"
				  -->
						<div
							class="pointer-events-auto w-screen max-w-md"
							class:details-open={detailsOpen}
							class:details-closed={!detailsOpen}>
							<form
								class="flex h-full flex-col divide-y divide-gray-200 bg-white shadow-xl">
								<div class="h-0 flex-1 overflow-y-auto">
									<div class="bg-emerald-700 py-6 px-4 sm:px-6">
										<div class="flex items-center justify-between">
											<h2
												class="text-lg font-medium text-white"
												id="slide-over-title">
												New Project
											</h2>
											<div class="ml-3 flex h-7 items-center">
												<button
													on:click={() => (detailsOpen = false)}
													type="button"
													class="rounded-md bg-emerald-700 text-emerald-200 hover:text-white focus:outline-none focus:ring-2 focus:ring-white">
													<span class="sr-only">Close panel</span>
													<!-- Heroicon name: outline/x-mark -->
													<svg
														class="h-6 w-6"
														xmlns="http://www.w3.org/2000/svg"
														fill="none"
														viewBox="0 0 24 24"
														stroke-width="1.5"
														stroke="currentColor"
														aria-hidden="true">
														<path
															stroke-linecap="round"
															stroke-linejoin="round"
															d="M6 18L18 6M6 6l12 12" />
													</svg>
												</button>
											</div>
										</div>
										<div class="mt-1">
											<p class="text-sm text-emerald-300">
												Get started by filling in the information below to
												create your new project.
											</p>
										</div>
									</div>
									<div class="flex flex-1 flex-col justify-between">
										<div class="divide-y divide-gray-200 px-4 sm:px-6">
											<div class="space-y-6 pt-6 pb-5">
												<div>
													<label
														for="url_slug"
														class="block text-sm font-medium text-gray-900">
														URL Slug
													</label>
													<div class="mt-1">
														<input
															type="text"
															name="url_slug"
															bind:value={post.slug}
															id="url_slug"
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
															bind:value={post.short_description}
															rows="4"
															class="block w-full rounded-md border-gray-300 shadow-sm focus:border-emerald-500 focus:ring-emerald-500 sm:text-sm" />
													</div>
												</div>
											</div>
											<div class="pt-4 pb-6">
												<div class="flex text-sm">
													<a
														href="#"
														class="group inline-flex items-center font-medium text-emerald-600 hover:text-emerald-900">
														<!-- Heroicon name: mini/link -->
														<svg
															class="h-5 w-5 text-emerald-500 group-hover:text-emerald-900"
															xmlns="http://www.w3.org/2000/svg"
															viewBox="0 0 20 20"
															fill="currentColor"
															aria-hidden="true">
															<path
																d="M12.232 4.232a2.5 2.5 0 013.536 3.536l-1.225 1.224a.75.75 0 001.061 1.06l1.224-1.224a4 4 0 00-5.656-5.656l-3 3a4 4 0 00.225 5.865.75.75 0 00.977-1.138 2.5 2.5 0 01-.142-3.667l3-3z" />
															<path
																d="M11.603 7.963a.75.75 0 00-.977 1.138 2.5 2.5 0 01.142 3.667l-3 3a2.5 2.5 0 01-3.536-3.536l1.225-1.224a.75.75 0 00-1.061-1.06l-1.224 1.224a4 4 0 105.656 5.656l3-3a4 4 0 00-.225-5.865z" />
														</svg>
														<span class="ml-2">Copy link</span>
													</a>
												</div>
												<div class="mt-4 flex text-sm">
													<a
														href="#"
														class="group inline-flex items-center text-gray-500 hover:text-gray-900">
														<!-- Heroicon name: mini/question-mark-circle -->
														<svg
															class="h-5 w-5 text-gray-400 group-hover:text-gray-500"
															xmlns="http://www.w3.org/2000/svg"
															viewBox="0 0 20 20"
															fill="currentColor"
															aria-hidden="true">
															<path
																fill-rule="evenodd"
																d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM8.94 6.94a.75.75 0 11-1.061-1.061 3 3 0 112.871 5.026v.345a.75.75 0 01-1.5 0v-.5c0-.72.57-1.172 1.081-1.287A1.5 1.5 0 108.94 6.94zM10 15a1 1 0 100-2 1 1 0 000 2z"
																clip-rule="evenodd" />
														</svg>
														<span class="ml-2">
															Learn more about sharing
														</span>
													</a>
												</div>
											</div>
										</div>
									</div>
								</div>
								<div class="flex flex-shrink-0 justify-end px-4 py-4">
									<button
										type="button"
										class="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2">
										Cancel
									</button>
									<button
										type="submit"
										class="ml-4 inline-flex justify-center rounded-md border border-transparent bg-emerald-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2">
										Save
									</button>
								</div>
							</form>
						</div>
					</div>
				</div>
			</div>
		</div>
	</form>

	<div>
		<Tiptap
			getContent={(jsonContent) => {
				let first = jsonContent.content.at(0);
				if (first.type === 'heading') {
					title = first?.content?.at(0)?.text;
				}
				content = JSON.stringify(jsonContent);
			}}
			initialContent={post.body}
			bind:this={tiptap} />
	</div>
</div>
