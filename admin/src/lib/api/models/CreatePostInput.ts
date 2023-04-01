/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type CreatePostInput = {
	body_html: string;
	body_json: any;
	body_text: string;
	category_id?: string | null;
	is_featured: boolean;
	locale: string;
	meta_description?: string | null;
	meta_keywords?: string | null;
	meta_title?: string | null;
	published_at?: string | null;
	short_description?: string | null;
	title: string;
	translation_of?: string | null;
};
