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
	public static getAdminDocs(): CancelablePromise<void> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/admin/docs/'
		});
	}
}
