<script lang="ts">
	import Dropzone from 'svelte-file-dropzone';
	import { page } from '$app/stores';
	import { srcset } from '$lib/images';
	export let show: boolean = false;
	export let onMediaSelect: Function | undefined;

	let url = `/admin/${$page.params.subdomain}/media`;

	async function getRecent() {
		let response = await fetch(`${url}/images`);
		let data = await response.json();
		return data;
	}

	export let images: Map<string, Image[]> = new Map();

	$: imageset = [...images.values()];

	const extToMimes = {
		'image/jpeg': 'jpg',
		'image/webp': 'webp',
		'image/gif': 'gif',
		'image/png': 'png'
	};

	interface Image {
		file: File;
		width: string;
	}

	async function handleFilesSelect(e: { detail: { acceptedFiles: any } }) {
		const { acceptedFiles } = e.detail;

		acceptedFiles.forEach(async (file: File) => {
			let optimized: any[] = [];
			let imageBitmap = await createImageBitmap(file);
			let quality = 65;
			let prefix = 'image';

			let original = new File([file], `original.${extToMimes[file.type]}`, {
				type: file.type
			});
			optimized.push({ file: original, width: `original_${imageBitmap.width}` });

			let result = await compressImage(file, 1, 100);
			let width = Math.trunc(result.canvasWidth);
			let source = new File([result.compressedImage], `${prefix}_${width}.webp`, {
				type: result.compressedImage.type
			});
			optimized.push({ file: source, width });

			if (imageBitmap.width > 640) {
				let result = await compressImage(file, 640, quality);
				let width = Math.trunc(result.canvasWidth);
				let source = new File([result.compressedImage], `${prefix}_${width}.webp`, {
					type: result.compressedImage.type
				});
				optimized.push({ file: source, width });
			}
			if (imageBitmap.width > 1920) {
				let result = await compressImage(file, 1920, quality);
				let width = Math.trunc(result.canvasWidth);
				let source = new File([result.compressedImage], `${prefix}_${width}.webp`, {
					type: result.compressedImage.type
				});
				optimized.push({ file: source, width });
			}
			if (imageBitmap.width > 2400) {
				let result = await compressImage(file, 2400, quality);
				let width = Math.trunc(result.canvasWidth);
				let source = new File([result.compressedImage], `${prefix}_${width}.webp`, {
					type: result.compressedImage.type
				});
				optimized.push({ file: source, width });
			}
			images.set(file.name, optimized);
		});
	}

	async function compressImage(imgToCompress: File, resizeWidth: number, quality: number) {
		let imageBitmap = await createImageBitmap(imgToCompress);
		// showing the compressed image
		const canvas = document.createElement('canvas');
		const context = canvas.getContext('2d');

		const originalWidth = imageBitmap.width;
		const originalHeight = imageBitmap.height;

		let resizeHeight = (originalHeight * resizeWidth) / originalWidth;

		const canvasWidth = resizeWidth;
		const canvasHeight = resizeHeight;

		canvas.width = canvasWidth;
		canvas.height = canvasHeight;

		if (!context) {
			throw new Error('Error creating canvas context');
		}

		context.drawImage(imageBitmap, 0, 0, canvasWidth, canvasHeight);

		let compressedImage = await getCanvasBlob(canvas, quality);

		return { compressedImage, canvasWidth, canvasHeight };
	}

	function getCanvasBlob(canvas: HTMLCanvasElement, quality: number): Promise<Blob> {
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
				console.log('Uploading', image.file.name);
				formData.append(image.width, image.file);
			});

			let response = await fetch(`/admin/${$page.params.subdomain}/media`, {
				method: 'POST',
				body: formData
			});
			console.log('response', response.status);
		}
	}
</script>

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
					<div class="columns-2 md:columns-3 lg:columns-4">
						{#each imageset as item}
							<img
								class="h-auto max-w-full rounded-lg"
								src={URL.createObjectURL(item.file)}
								alt="thumbnail" />
						{/each}
					</div>
				</div>
				<div class="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
					<button
						on:click={() => uploadFiles()}
						type="button"
						class="inline-flex w-full justify-center rounded-md bg-slate-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-slate-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-slate-600 sm:col-start-2">
						Upload
					</button>
					<button
						on:click={() => (show = false)}
						type="button"
						class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:col-start-1 sm:mt-0">
						Cancel
					</button>
				</div>
				<div>
					<div class="grid grid-cols-2 md:grid-cols-6 gap-4">
						{#await getRecent() then recentImages}
							{#each recentImages as image}
								<div>
									<button
										on:click|preventDefault={() => {
											if (onMediaSelect) {
												onMediaSelect(srcset(image));
											}
											show = false;
										}}>
										<img
											class="h-auto max-h-44 max-w-44 rounded-lg"
											loading="lazy"
											srcset={srcset(image)}
											alt="" />
									</button>
								</div>
							{/each}
						{/await}
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
