<script lang="ts">
	import type { ImageSet } from '$lib/api';
	import { buildSrc, buildSrcset, sizeFromString } from '$lib/images';

	export let imageSet: ImageSet[];

	let srcset = '';
	let images = imageSet.sort(
		(a: any, b: any) => sizeFromString(a.size).width - sizeFromString(b.size).width
	);
	let srcsets = images.map(buildSrcset);
	srcset = srcsets.join(', ');
</script>

<img
	loading="lazy"
	class={$$props.class}
	{srcset}
	src={buildSrc(imageSet[0])}
	alt="todo: add alt" />
