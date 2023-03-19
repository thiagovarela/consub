/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class DefaultService {

    /**
     * This documentation page.
     * @throws ApiError
     */
    public static getDocs(): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/docs/',
        });
    }

    /**
     * Health check endpoint
     * @returns any plain text
     * @throws ApiError
     */
    public static getHealth(): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/health',
        });
    }

}
