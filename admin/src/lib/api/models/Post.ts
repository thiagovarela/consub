/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type Post = {
    account_id: string;
    author_id: string;
    body_html: string;
    body_json: any;
    body_text: string;
    category_id?: string | null;
    id: string;
    is_featured: boolean;
    locale: string;
    meta_description?: string | null;
    meta_keywords?: string | null;
    meta_title?: string | null;
    published_at?: string | null;
    short_description?: string | null;
    slug: string;
    title: string;
    translation_of?: string | null;
    updated_at: string;
};

