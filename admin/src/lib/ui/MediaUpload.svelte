<script lang="ts">
	import Dropzone from 'svelte-file-dropzone';
	import loadImage from 'blueimp-load-image';
	import { page } from '$app/stores';
	import { srcset } from '$lib/images';
	import ImageSet from './images/ImageSet.svelte';
	export let show: boolean = false;
	export let onMediaSelect: Function | undefined;

	let url = `/admin/${$page.params.subdomain}/media`;

	enum Tabs {
		Files,
		Recent
	}

	let tab: Tabs = Tabs.Files;

	async function getRecent() {
		let response = await fetch(`${url}/images`);
		let data = await response.json();
		return data;
	}

	export let images: Map<string, Image[]> = new Map();
	let thumbs: Image[] = [];

	interface Image {
		file: File;
		width: number;
		height: number;
	}

	function imageSize(image: Image): string {
		return `${image.width}x${image.height}`;
	}

	async function resizeImage(
		file: File,
		maxWidth: number,
		maxHeight: number,
		quality?: number
	): Promise<File> {
		console.log('resizing image', maxWidth, maxHeight, quality);
		let loadedImage = await loadImage(file, {
			canvas: true,
			maxWidth,
			maxHeight,
			cover: true,
			orientation: true,
			imageSmoothingQuality: 'high'
		});

		// @ts-ignore - the loadedImage library returns a img element but we only use canvas.
		let blob = await getCanvasBlob(loadedImage.image, quality);
		let source = new File([blob], `image_${maxWidth}x${maxHeight}.webp`, {
			type: 'image/webp'
		});
		return Promise.resolve(source);
	}

	function handleFilesSelect(e: { detail: { acceptedFiles: any } }) {
		const { acceptedFiles } = e.detail;

		acceptedFiles.forEach(async (file: File) => {
			let optimized: any[] = [];
			let imageBitmap = await createImageBitmap(file);
			let quality = 0.7;

			let resized = await resizeImage(file, 320, 320, quality);
			thumbs.push({ file: resized, width: 320, height: 320 });
			thumbs = thumbs; // Triggers svelte reactivity for arrays

			if (imageBitmap.width > 640) {
				resized = await resizeImage(file, 640, 480, quality);
				optimized.push({ file: resized, width: 640, height: 480 });
			}

			if (imageBitmap.width > 1280) {
				resized = await resizeImage(file, 1280, 720, quality);
				optimized.push({ file: resized, width: 1280, height: 720 });
			}

			if (imageBitmap.width > 1920) {
				resized = await resizeImage(file, 1920, 1080, quality);
				optimized.push({ file: resized, width: 1920, height: 1080 });
			}

			if (imageBitmap.width > 2400) {
				resized = await resizeImage(file, 2400, 1920, quality);
				optimized.push({ file: resized, width: 2400, height: 1920 });
			}

			resized = await resizeImage(file, imageBitmap.width, imageBitmap.height, quality);
			optimized.push({ file: resized, width: imageBitmap.width, height: imageBitmap.height });

			images.set(file.name, optimized);
		});
	}

	function getCanvasBlob(canvas: HTMLCanvasElement, quality?: number): Promise<Blob> {
		return new Promise(function (resolve, reject) {
			canvas.toBlob(
				function (blob) {
					if (blob) {
						resolve(blob);
					} else {
						reject('Error converting to webp');
					}
				},
				'image/webp',
				quality
			);
		});
	}

	async function uploadFiles() {
		for (const item of images) {
			console.log('sending image', item);

			const imageset = item[1];
			const formData = new FormData();
			imageset.forEach((image) => {
				console.log('Preparing', image.file.name, image.file.size / 1024, 'kb');
				formData.append(imageSize(image), image.file);
			});

			let response = await fetch(`/admin/${$page.params.subdomain}/media`, {
				method: 'POST',
				body: formData
			});
			console.log(response);
			let res = await response.json();
			console.log('response', res);
		}
	}

	function close() {
		images.clear();
		thumbs = [];
		show = false;
	}
</script>

<style lang="postcss">
	.tab-selected {
		@apply bg-slate-100 text-slate-700 rounded-md px-3 py-2 text-sm font-medium;
	}
	.tab {
		@apply text-gray-500 hover:text-gray-700 rounded-md px-3 py-2 text-sm font-medium;
	}
</style>

<div
	class="relative z-10"
	class:hidden={!show}
	aria-labelledby="modal-title"
	role="dialog"
	aria-modal="true">
	<!--
      Background backdrop, show/hide based on modal state.
  
      Entering: "ease-out duration-300"
        From: "opacity-0"
        To: "opacity-100"
      Leaving: "ease-in duration-200"
        From: "opacity-100"
        To: "opacity-0"
    -->
	<div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />

	<div class="fixed inset-0 z-10 overflow-y-auto">
		<div
			class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
			<!--
          Modal panel, show/hide based on modal state.
  
          Entering: "ease-out duration-300"
            From: "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            To: "opacity-100 translate-y-0 sm:scale-100"
          Leaving: "ease-in duration-200"
            From: "opacity-100 translate-y-0 sm:scale-100"
            To: "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
        -->
			<div
				class="relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6">
				<div>
					<div class="sm:hidden">
						<label for="___tabs" class="sr-only">Select a tab</label>
						<!-- Use an "onChange" listener to redirect the user to the selected tab URL. -->
						<select
							name="___tabs"
							id="___tabs"
							class="block w-full rounded-md border-gray-300 focus:border-indigo-500 focus:ring-indigo-500">
							<option>My Account</option>

							<option>Company</option>

							<option selected>Team Members</option>

							<option>Billing</option>
						</select>
					</div>

					<div class="hidden sm:block">
						<nav class="flex space-x-4" aria-label="Tabs">
							<!-- Current: "bg-indigo-100 text-indigo-700", Default: "text-gray-500 hover:text-gray-700" -->
							<button
								on:click|preventDefault={() => (tab = Tabs.Files)}
								class:tab={tab != Tabs.Files}
								class:tab-selected={tab == Tabs.Files}>
								Upload Files
							</button>

							<button
								on:click|preventDefault={() => (tab = Tabs.Recent)}
								class:tab={tab != Tabs.Recent}
								class:tab-selected={tab == Tabs.Recent}>
								Recent Uploaded
							</button>
						</nav>
					</div>
				</div>
				{#if tab == Tabs.Files}
					<div>
						<div class="mt-3 text-center sm:mt-5">
							<Dropzone
								on:drop={handleFilesSelect}
								accept={[
									'image/png',
									'image/jpeg',
									'image/webp',
									'image/gif',
									'image/svg+xml'
								]} />
						</div>
					</div>
					<div>
						{#if thumbs.length > 0}
							<div class="text-sm">Preview</div>
							<div class="columns-2 md:columns-3 lg:columns-4">
								{#each thumbs as item}
									<img
										class="h-auto max-w-full rounded-lg border border-gray-200"
										src={URL.createObjectURL(item.file)}
										alt="thumbnail" />
								{/each}
							</div>
						{/if}
					</div>
					<div
						class="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
						<button
							on:click={() => uploadFiles()}
							type="button"
							class="inline-flex w-full justify-center rounded-md bg-slate-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-slate-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-slate-600 sm:col-start-2">
							Upload
						</button>
						<button
							on:click={() => close()}
							type="button"
							class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:col-start-1 sm:mt-0">
							Cancel
						</button>
					</div>
				{/if}
				{#if tab == Tabs.Recent}
					<div>
						<div class="grid grid-cols-2 md:grid-cols-6 gap-4">
							{#await getRecent() then recentImages}
								{#each recentImages as image}
									<div>
										<button
											on:click|preventDefault={() => {
												if (onMediaSelect) {
													onMediaSelect(image.image_set);
												}
												close();
											}}>
											<ImageSet imageSet={image.image_set} />
										</button>
									</div>
								{/each}
							{/await}
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
