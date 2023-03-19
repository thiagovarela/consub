/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type ClippingItemQuery = {
    /**
     * The pagination cursor to start at.
     */
    after?: string | null;
    /**
     * The pagination cursor to end at.
     */
    before?: string | null;
    category_id?: Array<string>;
    is_featured?: boolean | null;
    locale?: string | null;
    published_at?: string | null;
    slug?: string | null;
    tag?: Array<string>;
    /**
     * The number of items to return. Default is 100.
     */
    take?: number;
};

