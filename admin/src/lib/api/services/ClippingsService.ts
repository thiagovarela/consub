/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Category } from '../models/Category';
import type { ChangeCategoryInput } from '../models/ChangeCategoryInput';
import type { ChangeClippingItemInput } from '../models/ChangeClippingItemInput';
import type { ClippingItem } from '../models/ClippingItem';
import type { CreateCategoryInput } from '../models/CreateCategoryInput';
import type { CreateClippingItemInput } from '../models/CreateClippingItemInput';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class ClippingsService {

    /**
     * List categories
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
            url: '/admin/clippings/categories',
            query: {
                'locale': locale,
                'name': name,
                'translation_of': translationOf,
            },
        });
    }

    /**
     * Create a new category
     * @returns Category
     * @throws ApiError
     */
    public static createCategory({
        requestBody,
    }: {
        requestBody: CreateCategoryInput,
    }): CancelablePromise<Category> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/clippings/categories',
            body: requestBody,
            mediaType: 'application/json',
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
            url: '/admin/clippings/categories/{category_id}',
            path: {
                'category_id': categoryId,
            },
        });
    }

    /**
     * Delete category
     * @returns void
     * @throws ApiError
     */
    public static deleteCategory({
        categoryId,
    }: {
        categoryId: string,
    }): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/admin/clippings/categories/{category_id}',
            path: {
                'category_id': categoryId,
            },
        });
    }

    /**
     * Change category
     * @returns Category
     * @throws ApiError
     */
    public static changeCategory({
        categoryId,
        requestBody,
    }: {
        categoryId: string,
        requestBody: ChangeCategoryInput,
    }): CancelablePromise<Category> {
        return __request(OpenAPI, {
            method: 'PATCH',
            url: '/admin/clippings/categories/{category_id}',
            path: {
                'category_id': categoryId,
            },
            body: requestBody,
            mediaType: 'application/json',
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
            url: '/admin/clippings/items',
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
     * Create a new clipping item
     * @returns ClippingItem
     * @throws ApiError
     */
    public static createItem({
        requestBody,
    }: {
        requestBody: CreateClippingItemInput,
    }): CancelablePromise<ClippingItem> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/clippings/items',
            body: requestBody,
            mediaType: 'application/json',
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
            url: '/admin/clippings/items/{item_id}',
            path: {
                'item_id': itemId,
            },
        });
    }

    /**
     * Change clipping item
     * @returns ClippingItem
     * @throws ApiError
     */
    public static changeItem({
        itemId,
        requestBody,
    }: {
        itemId: string,
        requestBody: ChangeClippingItemInput,
    }): CancelablePromise<ClippingItem> {
        return __request(OpenAPI, {
            method: 'PATCH',
            url: '/admin/clippings/items/{item_id}',
            path: {
                'item_id': itemId,
            },
            body: requestBody,
            mediaType: 'application/json',
        });
    }

}
