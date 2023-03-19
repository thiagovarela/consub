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
        name,
        translationOf,
    }: {
        locale?: string | null,
        name?: string | null,
        translationOf?: string | null,
    }): CancelablePromise<Array<Category>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/clippings/categories',
            query: {
                'locale': locale,
                'name': name,
                'translation_of': translationOf,
            },
        });
    }

    /**
     * Get category
     * @returns Category
     * @throws ApiError
     */
    public static getCategoryById({
        categoryId,
    }: {
        categoryId: string,
    }): CancelablePromise<Category> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/clippings/categories/{category_id}',
            path: {
                'category_id': categoryId,
            },
        });
    }

    /**
     * List clipping items
     * @returns ClippingItem
     * @throws ApiError
     */
    public static listItems({
        after,
        before,
        categoryId,
        isFeatured,
        locale,
        publishedAt,
        slug,
        tag,
        take = 100,
    }: {
        /**
         * The pagination cursor to start at.
         */
        after?: string | null,
        /**
         * The pagination cursor to end at.
         */
        before?: string | null,
        categoryId?: Array<string>,
        isFeatured?: boolean | null,
        locale?: string | null,
        publishedAt?: string | null,
        slug?: string | null,
        tag?: Array<string>,
        /**
         * The number of items to return. Default is 100.
         */
        take?: number,
    }): CancelablePromise<Array<ClippingItem>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/clippings/items',
            query: {
                'after': after,
                'before': before,
                'category_id': categoryId,
                'is_featured': isFeatured,
                'locale': locale,
                'published_at': publishedAt,
                'slug': slug,
                'tag': tag,
                'take': take,
            },
        });
    }

    /**
     * Get clipping item
     * @returns ClippingItem
     * @throws ApiError
     */
    public static getItemById({
        itemId,
    }: {
        itemId: string,
    }): CancelablePromise<ClippingItem> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/clippings/items/{item_id}',
            path: {
                'item_id': itemId,
            },
        });
    }

}
