/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type CreateClippingItemInput = {
	body_html: string;
	body_json: any;
	body_text: string;
	category_id?: string | null;
	is_featured?: boolean | null;
	locale: string;
	published_at?: string | null;
	reading_time_minutes?: number | null;
	short_description?: string | null;
	source: string;
	source_published_at: string;
	source_url: string;
	tags?: Array<string>;
	title: string;
};
