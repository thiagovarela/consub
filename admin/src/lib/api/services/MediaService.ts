/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ImageResponse } from '../models/ImageResponse';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class MediaService {

    /**
     * List images uploaded to the media library
     * @returns ImageResponse
     * @throws ApiError
     */
    public static listImages({
        after,
        before,
        imageId,
        size,
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
        imageId?: Array<string>,
        size?: string | null,
        /**
         * The number of items to return. Default is 100.
         */
        take?: number,
    }): CancelablePromise<Array<ImageResponse>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/media/images',
            query: {
                'after': after,
                'before': before,
                'image_id': imageId,
                'size': size,
                'take': take,
            },
        });
    }

    /**
     * Upload image to media library
     * @returns ImageResponse
     * @throws ApiError
     */
    public static uploadImage({
        formData,
    }: {
        /**
         * multipart form data
         */
        formData: any[],
    }): CancelablePromise<ImageResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/media/images',
            formData: formData,
            mediaType: 'multipart/form-data',
        });
    }

}
