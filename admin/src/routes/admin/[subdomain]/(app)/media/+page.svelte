<script lang="ts">
	import { srcset } from '$lib/images';
	import BasePage from '$lib/ui/BasePage.svelte';
	import MediaUpload from '$lib/ui/MediaUpload.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	let imageUploadModal = false;
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
		<div class="grid grid-cols-2 md:grid-cols-6 gap-4">
			{#each data.images as image}
				<div>
					<img
						class="h-auto max-h-44 max-w-44 rounded-lg"
						loading="lazy"
						srcset={srcset(image)}
						alt="" />
				</div>
			{/each}
		</div>
	</span>
</BasePage>
<MediaUpload bind:show={imageUploadModal} />
