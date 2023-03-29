<script lang="ts">
	import type { ImageResponse } from '$lib/api';
	import BasePage from '$lib/ui/BasePage.svelte';
	import ImageSet from '$lib/ui/images/ImageSet.svelte';
	import MediaUpload from '$lib/ui/MediaUpload.svelte';
	import Modal from '$lib/ui/Modal.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	let imageUploadModal = false;
	let imageModal = false;
	let selectedImage: ImageResponse | undefined;
</script>

<BasePage>
	<span slot="header-left">
		<h1 class="text-xl font-semibold text-gray-900">Media Library</h1>
	</span>
	<span slot="header-actions">
		<button
			on:click={() => (imageUploadModal = true)}
			class="border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
			Upload
		</button>
	</span>
	<span slot="main">
		<div class="grid grid-cols-1 md:grid-cols-6 gap-4">
			{#each data.images as image}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					data-id={image.id}
					on:click={({ currentTarget: { dataset } }) => {
						selectedImage = data.images.find((i) => i.id === dataset.id);
						imageModal = true;
					}}>
					<ImageSet
						class="h-auto md:max-h-44 md:max-w-44 rounded-lg"
						imageSet={image.image_set} />
				</div>
			{/each}
		</div>
	</span>
</BasePage>
<MediaUpload bind:show={imageUploadModal} />

{#if imageModal && selectedImage}
	<Modal bind:show={imageModal}>
		<span slot="header"><h1>Image</h1></span>
		{''}
		<ImageSet class="h-auto rounded-lg" imageSet={selectedImage.image_set} />
	</Modal>
{/if}
