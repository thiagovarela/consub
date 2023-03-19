export function srcset(image: any) {
	let srcset = '';
	let images = image.image_set
		.filter((image: { size: number }) => image.size < 2400)
		.sort((a: any, b: any) => a.size - b.size);
	images.forEach((image_set: any) => {
		srcset += `https://media.consub.io/${image_set.path} ${image_set.size}w,`;
	});
	return srcset;
}
