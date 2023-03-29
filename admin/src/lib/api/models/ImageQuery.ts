/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type ImageQuery = {
    /**
     * The pagination cursor to start at.
     */
    after?: string | null;
    /**
     * The pagination cursor to end at.
     */
    before?: string | null;
    image_id?: Array<string>;
    size?: string | null;
    /**
     * The number of items to return. Default is 100.
     */
    take?: number;
};

