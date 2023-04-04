/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type PostQuery = {
	/**
	 * The pagination cursor to start at.
	 */
	after?: string | null;
	/**
	 * The pagination cursor to end at.
	 */
	before?: string | null;
	category_id?: Array<string>;
	category_slug?: string | null;
	is_featured?: boolean | null;
	locale?: string | null;
	published_at?: string | null;
	slug?: string | null;
	/**
	 * The number of items to return. Default is 100.
	 */
	take?: number;
	translation_of?: string | null;
};
