import type { ImageSet } from './api';

export function srcset(image: any) {
	let srcset = '';
	let images = image.image_set.sort(
		(a: any, b: any) => sizeFromString(a.size).width - sizeFromString(b.size).width
	);
	let srcsets = images.map(buildSrc);
	srcset = srcsets.join(', ');
	return { srcset, src: buildSrc(images[0]) };
}

export function sizeFromString(size: string): { width: number; height: number } {
	let [width, height] = size.split('x');
	return { width: parseInt(width), height: parseInt(height) };
}

export function buildSrc(image_set: ImageSet) {
	return `${image_set.path}`;
}

export function buildSrcset(image_set: ImageSet) {
	return `${image_set.path} ${sizeFromString(image_set.size).width}w`;
}
