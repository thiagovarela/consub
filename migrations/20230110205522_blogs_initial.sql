CREATE SCHEMA blogs;

CREATE TABLE blogs.categories (
    id UUID PRIMARY KEY DEFAULT generate_ulid_uuid(),
    name TEXT NOT NULL,
    slug TEXT NOT NULL,    
    locale VARCHAR(5) NOT NULL,
    translation_of UUID NULL REFERENCES blogs.categories (id),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),    
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX categories_id_idx ON blogs.categories (id);
CREATE INDEX categories_account_id_idx ON blogs.categories (account_id);
CREATE INDEX categories_account_id_locale_idx ON blogs.categories (account_id, locale);
CREATE UNIQUE INDEX categories_slug_locale_account_id_unique ON blogs.categories (slug, locale, account_id);
SELECT setup_tgr_updated_at('blogs.categories');


CREATE TABLE blogs.posts (
    id UUID PRIMARY KEY DEFAULT generate_ulid_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    author_id UUID NOT NULL REFERENCES accounts.users (id),
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    short_description TEXT NULL,
    body_json JSONB NOT NULL,
    body_html TEXT NOT NULL,
    body_text TEXT NOT NULL,
    locale VARCHAR(5) NOT NULL,
    meta_title TEXT NULL,
    meta_keywords TEXT NULL,
    meta_description TEXT NULL,
    category_id UUID NULL REFERENCES blogs.categories (id),
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,    
    translation_of UUID NULL REFERENCES blogs.posts (id),     
    published_at TIMESTAMP,    
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);
CREATE INDEX posts_id_idx ON blogs.posts (id);
CREATE INDEX posts_account_id_idx ON blogs.posts (account_id);
CREATE INDEX posts_account_id_locale_idx ON blogs.posts (account_id, locale);
CREATE UNIQUE INDEX posts_slug_locale_account_id_unique ON blogs.posts (slug, locale, account_id);
SELECT setup_tgr_updated_at('blogs.posts');

CREATE TABLE blogs.post_images (
    id UUID PRIMARY KEY DEFAULT generate_ulid_uuid(),
    post_id UUID NOT NULL REFERENCES blogs.posts (id),
    image_type VARCHAR(255) NOT NULL,
    media_id UUID NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);
CREATE INDEX post_images_id_idx ON blogs.post_images (id);
CREATE INDEX post_images_post_id_media_id_idx ON blogs.post_images (post_id, media_id);
SELECT setup_tgr_updated_at('blogs.post_images');