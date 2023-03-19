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
    keywords: Array<string>;
    locale: string;
    published_at?: string | null;
    reading_time_minutes?: number | null;
    short_description?: string | null;
    slug: string;
    title: string;
    translation_of?: string | null;
    updated_at: string;
};

