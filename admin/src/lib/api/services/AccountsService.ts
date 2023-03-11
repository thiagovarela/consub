/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AccessToken } from '../models/AccessToken';
import type { CreateUserAccessTokenWithPassword } from '../models/CreateUserAccessTokenWithPassword';
import type { User } from '../models/User';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class AccountsService {
	/**
	 * Get an access token for a user with a password.
	 * @returns AccessToken
	 * @throws ApiError
	 */
	public static getAccessTokenWithPassword({
		requestBody
	}: {
		requestBody: CreateUserAccessTokenWithPassword;
	}): CancelablePromise<AccessToken> {
		return __request(OpenAPI, {
			method: 'POST',
			url: '/accounts/users/access-tokens/passwords',
			body: requestBody,
			mediaType: 'application/json'
		});
	}

	/**
	 * Get the profile of the current user.
	 * @returns User
	 * @throws ApiError
	 */
	public static getUserProfile(): CancelablePromise<User> {
		return __request(OpenAPI, {
			method: 'GET',
			url: '/accounts/users/profiles'
		});
	}
}
