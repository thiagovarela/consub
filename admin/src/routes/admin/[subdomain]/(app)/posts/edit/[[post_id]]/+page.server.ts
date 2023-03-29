import { error, fail } from '@sveltejs/kit';
import type { RequestEvent } from './$types';
import type { PageServerLoad } from './$types';
import { parse } from '$lib/locales';
import { BlogsService, MediaService, type ChangePostInput, type CreatePostInput } from '$lib/api';
import dayjs from 'dayjs';

export const load = (async ({ params, request }: RequestEvent) => {
	let categories = BlogsService.listPostCategories({});
	let recentlyUploaded = MediaService.listImages({ take: 10 });
	let languages = parse(request.headers.get('accept-language')).map(
		(l: { code: any; region: any }) => `${l.code}-${l.region}`
	);

	let base = {
		categories,
		languages,
		recentlyUploaded,
		pageTitle: 'Post'
	};

	if (params.post_id) {
		let post = await BlogsService.getPost({ postId: params.post_id });
		let postImages = await BlogsService.listPostImages({ postId: params.post_id });
		let mediaIds = postImages.map((image) => image.media_id);
		let images = await MediaService.listImages({ imageId: mediaIds });

		let featuredImage = postImages.find((image) => image.image_type === 'featured');
		let featuredImageMedia = images.find((image) => image.id === featuredImage?.media_id);
		let openGraphImage = postImages.find((image) => image.image_type === 'og');
		let openGraphImageMedia = images.find((image) => image.id === openGraphImage?.media_id);
		let twitterImage = postImages.find((image) => image.image_type === 'twitter');
		let twitterImageMedia = images.find((image) => image.id === twitterImage?.media_id);
		return {
			...base,
			featuredImage,
			featuredImageMedia,
			openGraphImage,
			openGraphImageMedia,
			twitterImage,
			twitterImageMedia,
			images,
			post
		};
	}
	return base;
}) satisfies PageServerLoad;

export const actions = {
	create: async ({ request }: RequestEvent) => {
		const values = await request.formData();
		const title = values.get('title')?.toString();
		let body_json = values.get('body_json')?.toString();
		const body_html = values.get('body_html')?.toString();
		const body_text = values.get('body_text')?.toString();
		const locale = values.get('locale')?.toString();
		const is_featured = values.get('is_featured')?.toString();
		const short_description = values.get('short_description')?.toString();
		const category_id = values.get('category_id')?.toString();
		const meta_title = values.get('meta_title')?.toString();
		const meta_description = values.get('meta_description')?.toString();
		const meta_keywords = values.get('meta_keywords')?.toString();

		if (!title || !body_json || !body_html || !body_text || !locale) {
			return fail(400, { message: 'Required fields are missing!' });
		}
		body_json = JSON.parse(body_json);

		let input = {
			title,
			body_json,
			body_html,
			body_text,
			locale,
			short_description,
			is_featured: is_featured ? true : false,
			category_id,
			meta_title,
			meta_description,
			meta_keywords
		} satisfies CreatePostInput;

		let post = await BlogsService.createPost({ requestBody: input });

		const media_id = values.get('featured_image_id')?.toString();
		if (media_id) {
			await BlogsService.createPostImages({
				postId: post.id,
				requestBody: { media_id, image_type: 'featured' }
			});
		}

		return {
			post,
			success: true,
			message: 'Post successfuly created'
		};
	},

	update: async ({ request, params }: RequestEvent) => {
		const values = await request.formData();
		console.log(values);
		const title = values.get('title')?.toString();
		let body_json = values.get('body_json')?.toString();
		const body_html = values.get('body_html')?.toString();
		const body_text = values.get('body_text')?.toString();
		const short_description = values.get('short_description')?.toString();
		const is_featured = values.get('is_featured')?.toString();
		const published_at = values.get('published_at')?.toString();
		const category_id = values.get('category_id')?.toString();
		const meta_title = values.get('meta_title')?.toString();
		const meta_description = values.get('meta_description')?.toString();
		const meta_keywords = values.get('meta_keywords')?.toString();

		if (!params.post_id) {
			throw error(400, 'Title and content are required');
		}

		let postId: string = params.post_id;

		body_json = body_json ? JSON.parse(body_json) : undefined;

		let input = {
			title,
			body_json,
			body_html,
			body_text,
			short_description,
			is_featured: is_featured ? true : false,
			published_at: published_at
				? dayjs(published_at).format('YYYY-MM-DDTHH:mm:ss')
				: undefined,
			category_id,
			meta_title,
			meta_description,
			meta_keywords
		} satisfies ChangePostInput;

		let post = await BlogsService.changePost({
			postId: params.post_id,
			requestBody: input
		});

		let featured_post_image_id = values.get('featured_post_image_id')?.toString();
		const media_id = values.get('featured_image_id')?.toString();
		if (featured_post_image_id && media_id) {
			// update
		} else if (media_id) {
			await BlogsService.createPostImages({
				postId,
				requestBody: { media_id, image_type: 'featured' }
			});
		}

		return {
			post,
			success: true,
			message: 'Post successfuly updated.'
		};
	},
	publish: async ({ request, params }: RequestEvent) => {
		const values = await request.formData();
		const published_at = values.get('published_at')?.toString();

		if (!params.post_id || !published_at) {
			throw error(400, 'post_id and published_at are required.');
		}

		let input = {
			published_at: dayjs(published_at).format('YYYY-MM-DDTHH:mm:ss')
		} satisfies ChangePostInput;

		let post = await BlogsService.changePost({
			postId: params.post_id,
			requestBody: input
		});

		return {
			post,
			success: true,
			message: 'Post successfuly published.'
		};
	},
	unpublish: async ({ locals, params }: RequestEvent) => {
		if (!params.post_id) {
			throw error(400, 'post_id is required');
		}

		let input = {
			published_at: null
		} satisfies ChangePostInput;

		let post = await BlogsService.changePost({
			postId: params.post_id,
			requestBody: input
		});

		return {
			post,
			success: true,
			message: 'Post successfuly unpublished.'
		};
	}
};
