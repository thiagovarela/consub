/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type CategoryQuery = {
	/**
	 * The pagination cursor to start at.
	 */
	after?: string | null;
	/**
	 * The pagination cursor to end at.
	 */
	before?: string | null;
	locale?: string | null;
	name_starts_with?: string | null;
	/**
	 * The number of items to return. Default is 100.
	 */
	take?: number;
	translation_of?: string | null;
};
