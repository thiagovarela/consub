<script lang="ts">
	// @ts-ignore
	import Dropzone from 'svelte-file-dropzone';
	import { page } from '$app/stores';
	import ImageSet from './images/ImageSet.svelte';
	export let onMediaSelect: Function | undefined = undefined;
	export let showTabs = false;
	import imageCompression from 'browser-image-compression';
	import axios from 'axios';

	let url = `/admin/${$page.params.subdomain}/media`;

	enum Tabs {
		Files,
		Recent
	}

	let tab: Tabs = Tabs.Files;
	let canUpload = false;
	let resizing = false;

	async function getRecent() {
		let response = await fetch(`${url}/images`);
		let data = await response.json();
		return data;
	}

	export let images: Map<string, Image[]> = new Map();
	let progressSteps: Map<string, number> = new Map();
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
		const options = {
			maxSizeMB: 1,
			maxWidthOrHeight: maxWidth,
			useWebWorker: true,
			maxIteration: 2,
			fileType: 'image/webp'
		};

		const blob = await imageCompression(file, options);
		console.log('resizing image', maxWidth, maxHeight, quality, blob);
		return Promise.resolve(new File([blob], file.name, { type: 'image/webp' }));
	}

	function updateProgress(fileName: string, step: number) {
		progressSteps.set(fileName, step);
		progressSteps = progressSteps; // Triggers svelte reactivity for maps
	}

	async function handleFilesSelect(e: { detail: { acceptedFiles: any } }) {
		canUpload = false;
		const { acceptedFiles } = e.detail;

		const promises = acceptedFiles.map(async (file: File) => {
			let optimized: any[] = [];
			let imageBitmap = await createImageBitmap(file);
			let quality = 0.8;
			resizing = true;

			let resized = await resizeImage(file, 320, 320, quality);
			thumbs.push({ file: resized, width: 320, height: 320 });
			thumbs = thumbs; // Triggers svelte reactivity for arrays
			updateProgress(file.name, 1);

			if (imageBitmap.width > 640) {
				resized = await resizeImage(file, 640, 480, quality);
				optimized.push({ file: resized, width: 640, height: 480 });
			}
			updateProgress(file.name, 25);

			if (imageBitmap.width > 1280) {
				resized = await resizeImage(file, 1280, 720, quality);
				optimized.push({ file: resized, width: 1280, height: 720 });
			}
			updateProgress(file.name, 45);

			if (imageBitmap.width > 1920) {
				resized = await resizeImage(file, 1920, 1080, quality);
				optimized.push({ file: resized, width: 1920, height: 1080 });
			}
			updateProgress(file.name, 65);

			if (imageBitmap.width > 2400) {
				resized = await resizeImage(file, 2400, 1920, quality);
				optimized.push({ file: resized, width: 2400, height: 1920 });
			}
			updateProgress(file.name, 85);

			resized = await resizeImage(file, imageBitmap.width, imageBitmap.height, quality);
			optimized.push({ file: resized, width: imageBitmap.width, height: imageBitmap.height });
			updateProgress(file.name, 100);

			images.set(file.name, optimized);
		});

		await Promise.all(promises);

		canUpload = images.size > 0;
	}

	async function uploadFiles() {
		resizing = false;
		for (const item of images) {
			console.log('sending image', item);

			const imageset = item[1];
			const formData = new FormData();
			imageset.forEach((image) => {
				let name = imageSize(image);
				console.log('Preparing', name, image.file.size / 1024 / 1024, 'mb');
				formData.append(name, image.file);
			});

			console.log(formData);

			axios
				.request({
					method: 'POST',
					url: `/admin/${$page.params.subdomain}/media`,
					data: formData,
					onUploadProgress: (p) => {
						updateProgress(item[0], Math.floor((p.progress ?? 1) * 100));
					}
				})
				.then((data) => {
					console.log(data);
				});
		}
	}

	function close() {
		images.clear();
		thumbs = [];
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
	{#if showTabs}
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
	{/if}
	{#if tab == Tabs.Files}
		<div>
			<div class="mt-3 text-center sm:mt-5">
				<Dropzone
					on:drop={async (e) => await handleFilesSelect(e)}
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
						<div class="flex flex-wrap">
							<div class="w-full bg-neutral-200 dark:bg-neutral-600">
								<div
									class="bg-slate-400 p-0.5 text-center text-xs font-medium leading-none text-primary-100"
									style="width: {progressSteps.get(item.file.name) ?? 1}%">
									{#if resizing}
										Resizing: {progressSteps.get(item.file.name) ?? 1}%
									{:else}
										Uploading: {progressSteps.get(item.file.name) ?? 1}%
									{/if}
								</div>
							</div>
							<img
								class="h-auto max-w-full rounded-lg border border-gray-200 block"
								src={URL.createObjectURL(item.file)}
								alt="thumbnail" />
						</div>
					{/each}
				</div>
			{/if}
		</div>
		<div class="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
			<button
				on:click={() => uploadFiles()}
				type="button"
				disabled={!canUpload}
				class="inline-flex w-full justify-center rounded-md bg-slate-600 px-3 py-2 text-sm 
						disabled:opacity-50 disabled:cursor-not-allowed
						font-semibold text-white shadow-sm hover:bg-slate-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-slate-600 sm:col-start-2">
				Upload
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
