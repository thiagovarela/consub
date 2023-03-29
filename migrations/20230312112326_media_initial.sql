CREATE SCHEMA media;

CREATE TABLE media.images (
    id UUID PRIMARY KEY DEFAULT generate_ulid_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    alt TEXT NULL,
    caption TEXT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX images_id_idx ON media.images (id);
CREATE INDEX images_account_id_idx ON media.images (account_id);
SELECT setup_tgr_updated_at('media.images');

CREATE TABLE media.images_set (
    id UUID PRIMARY KEY DEFAULT generate_ulid_uuid(),
    image_id UUID NOT NULL REFERENCES media.images (id),
    size TEXT NOT NULL,
    path TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX images_set_id_idx ON media.images_set (id);
CREATE INDEX images_set_image_id_idx ON media.images_set (image_id);
SELECT setup_tgr_updated_at('media.images_set');