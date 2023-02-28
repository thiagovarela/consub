/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Category } from '../models/Category';
import type { ClippingItem } from '../models/ClippingItem';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class PublicClippingsService {
	/**
	 * List Categories
	 * @returns Category
	 * @throws ApiError
	 */
	public static listCategories({
		locale,
		nameStartsWith,
		translationOf
	}: {
		locale?: string | null;
		nameStartsWith?: string | null;
		translationOf?: string | null;
	}): CancelablePromise<Array<Category>> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/clippings/categories',
			query: {
				locale: locale,
				name_starts_with: nameStartsWith,
				translation_of: translationOf
			}
		});
	}

	/**
	 * Get category
	 * @returns Category
	 * @throws ApiError
	 */
	public static getCategoryById({
		categoryId
	}: {
		categoryId: string;
	}): CancelablePromise<Category> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/clippings/categories/{category_id}',
			path: {
				category_id: categoryId
			}
		});
	}

	/**
	 * List clipping items
	 * @returns ClippingItem
	 * @throws ApiError
	 */
	public static listItems({
		categoryId,
		isFeatured,
		locale,
		publishedAt,
		tag
	}: {
		categoryId?: Array<string>;
		isFeatured?: boolean | null;
		locale?: string | null;
		publishedAt?: string | null;
		tag?: Array<string>;
	}): CancelablePromise<Array<ClippingItem>> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/clippings/items',
			query: {
				category_id: categoryId,
				is_featured: isFeatured,
				locale: locale,
				published_at: publishedAt,
				tag: tag
			}
		});
	}

	/**
	 * Get clipping item
	 * @returns ClippingItem
	 * @throws ApiError
	 */
	public static getItemById({ itemId }: { itemId: string }): CancelablePromise<ClippingItem> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/clippings/items/{item_id}',
			path: {
				item_id: itemId
			}
		});
	}
}
