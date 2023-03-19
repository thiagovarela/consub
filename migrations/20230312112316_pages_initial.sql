CREATE SCHEMA pages;

CREATE TABLE pages.pages (
    id UUID PRIMARY KEY DEFAULT generate_ulid_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    author_id UUID NOT NULL REFERENCES accounts.users (id),
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    short_description TEXT NULL,
    keywords TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
    body_json JSONB NOT NULL,
    body_html TEXT NOT NULL,
    body_text TEXT NOT NULL,
    locale VARCHAR(5) NOT NULL,            
    translation_of UUID NULL REFERENCES pages.pages (id),
    published_at TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);
CREATE INDEX pages_id_idx ON pages.pages (id);
CREATE INDEX pages_account_id_idx ON pages.pages (account_id);
CREATE INDEX pages_account_id_locale_idx ON pages.pages (account_id, locale);
CREATE UNIQUE INDEX pages_slug_locale_account_id_unique ON pages.pages (slug, locale, account_id);
SELECT setup_tgr_updated_at('pages.pages');

CREATE TABLE pages.page_images (
    id UUID PRIMARY KEY DEFAULT generate_ulid_uuid(),
    page_id UUID NOT NULL REFERENCES pages.pages (id),
    image_type VARCHAR(255) NOT NULL,
    media_id UUID NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);
CREATE INDEX page_images_id_idx ON pages.page_images (id);
CREATE INDEX page_images_page_id_media_id_idx ON pages.page_images (page_id, media_id);
SELECT setup_tgr_updated_at('pages.page_images');