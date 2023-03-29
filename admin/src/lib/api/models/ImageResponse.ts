/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ImageSet } from './ImageSet';

export type ImageResponse = {
    alt?: string | null;
    caption?: string | null;
    created_at: string;
    id: string;
    image_set: Array<ImageSet>;
    updated_at: string;
};

