/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Category } from '../models/Category';
import type { ChangeCategoryInput } from '../models/ChangeCategoryInput';
import type { ChangePostImageInput } from '../models/ChangePostImageInput';
import type { ChangePostInput } from '../models/ChangePostInput';
import type { CreateCategoryInput } from '../models/CreateCategoryInput';
import type { CreatePostImageInput } from '../models/CreatePostImageInput';
import type { CreatePostInput } from '../models/CreatePostInput';
import type { Post } from '../models/Post';
import type { PostImage } from '../models/PostImage';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class BlogsService {
	/**
	 * List post categories
	 * @returns Category
	 * @throws ApiError
	 */
	public static listPostCategories({
		locale,
		name,
		translationOf
	}: {
		locale?: string | null;
		name?: string | null;
		translationOf?: string | null;
	}): CancelablePromise<Array<Category>> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/admin/blogs/categories',
			query: {
				locale: locale,
				name: name,
				translation_of: translationOf
			}
		});
	}

	/**
	 * Create a new post category
	 * @returns Category
	 * @throws ApiError
	 */
	public static createPostCategory({
		requestBody
	}: {
		requestBody: CreateCategoryInput;
	}): CancelablePromise<Category> {
		return __request(OpenAPI, {
			method: 'POST',
			url: '/admin/blogs/categories',
			body: requestBody,
			mediaType: 'application/json'
		});
	}

	/**
	 * Get post category
	 * @returns Category
	 * @throws ApiError
	 */
	public static getPostCategoryById({
		categoryId
	}: {
		categoryId: string;
	}): CancelablePromise<Category> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/admin/blogs/categories/{category_id}',
			path: {
				category_id: categoryId
			}
		});
	}

	/**
	 * Change post category
	 * @returns void
	 * @throws ApiError
	 */
	public static deletePostCategory(): CancelablePromise<void> {
		return __request(OpenAPI, {
			method: 'DELETE',
			url: '/admin/blogs/categories/{category_id}'
		});
	}

	/**
	 * Change post category
	 * @returns Category
	 * @throws ApiError
	 */
	public static changePostCategory({
		categoryId,
		requestBody
	}: {
		categoryId: string;
		requestBody: ChangeCategoryInput;
	}): CancelablePromise<Category> {
		return __request(OpenAPI, {
			method: 'PATCH',
			url: '/admin/blogs/categories/{category_id}',
			path: {
				category_id: categoryId
			},
			body: requestBody,
			mediaType: 'application/json'
		});
	}

	/**
	 * List posts
	 * @returns Post
	 * @throws ApiError
	 */
	public static listPosts({
		categoryId,
		categorySlug,
		isFeatured,
		locale,
		publishedAt,
		slug,
		translationOf
	}: {
		categoryId?: Array<string>;
		categorySlug?: string | null;
		isFeatured?: boolean | null;
		locale?: string | null;
		publishedAt?: string | null;
		slug?: string | null;
		translationOf?: string | null;
	}): CancelablePromise<Array<Post>> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/admin/blogs/posts',
			query: {
				category_id: categoryId,
				category_slug: categorySlug,
				is_featured: isFeatured,
				locale: locale,
				published_at: publishedAt,
				slug: slug,
				translation_of: translationOf
			}
		});
	}

	/**
	 * Create a new post
	 * @returns Post
	 * @throws ApiError
	 */
	public static createPost({
		requestBody
	}: {
		requestBody: CreatePostInput;
	}): CancelablePromise<Post> {
		return __request(OpenAPI, {
			method: 'POST',
			url: '/admin/blogs/posts',
			body: requestBody,
			mediaType: 'application/json'
		});
	}

	/**
	 * Get a single post
	 * @returns Post
	 * @throws ApiError
	 */
	public static getPost({ postId }: { postId: string }): CancelablePromise<Post> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/admin/blogs/posts/{post_id}',
			path: {
				post_id: postId
			}
		});
	}

	/**
	 * Change a post
	 * @returns Post
	 * @throws ApiError
	 */
	public static changePost({
		postId,
		requestBody
	}: {
		postId: string;
		requestBody: ChangePostInput;
	}): CancelablePromise<Post> {
		return __request(OpenAPI, {
			method: 'PATCH',
			url: '/admin/blogs/posts/{post_id}',
			path: {
				post_id: postId
			},
			body: requestBody,
			mediaType: 'application/json'
		});
	}

	/**
	 * List post images
	 * @returns PostImage
	 * @throws ApiError
	 */
	public static listPostImages({
		postId
	}: {
		postId: string;
	}): CancelablePromise<Array<PostImage>> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/admin/blogs/posts/{post_id}/images',
			path: {
				post_id: postId
			}
		});
	}

	/**
	 * Create post image
	 * @returns PostImage
	 * @throws ApiError
	 */
	public static createPostImages({
		postId,
		requestBody
	}: {
		postId: string;
		requestBody: CreatePostImageInput;
	}): CancelablePromise<PostImage> {
		return __request(OpenAPI, {
			method: 'POST',
			url: '/admin/blogs/posts/{post_id}/images',
			path: {
				post_id: postId
			},
			body: requestBody,
			mediaType: 'application/json'
		});
	}

	/**
	 * Delete post image
	 * @returns void
	 * @throws ApiError
	 */
	public static deletePostImage({
		postId,
		postImageId
	}: {
		postId: string;
		postImageId: string;
	}): CancelablePromise<void> {
		return __request(OpenAPI, {
			method: 'DELETE',
			url: '/admin/blogs/posts/{post_id}/images/{post_image_id}',
			path: {
				post_id: postId,
				post_image_id: postImageId
			}
		});
	}

	/**
	 * Change post image
	 * @returns PostImage
	 * @throws ApiError
	 */
	public static changePostImage({
		postId,
		postImageId,
		requestBody
	}: {
		postId: string;
		postImageId: string;
		requestBody: ChangePostImageInput;
	}): CancelablePromise<PostImage> {
		return __request(OpenAPI, {
			method: 'PATCH',
			url: '/admin/blogs/posts/{post_id}/images/{post_image_id}',
			path: {
				post_id: postId,
				post_image_id: postImageId
			},
			body: requestBody,
			mediaType: 'application/json'
		});
	}
}
