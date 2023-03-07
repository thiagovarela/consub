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
    keywords TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
    body JSONB NOT NULL,
    locale VARCHAR(5) NOT NULL,
    category_id UUID NULL REFERENCES blogs.categories (id),
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,
    reading_time_minutes INTEGER,
    translation_of UUID NULL REFERENCES blogs.posts (id), 
    featured_image TEXT, 
    featured_image_alt TEXT,
    featured_image_caption TEXT,
    og_title TEXT,    
    og_description TEXT,
    og_image TEXT,
    published_at TIMESTAMP,    
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);
CREATE INDEX posts_id_idx ON blogs.posts (id);
CREATE INDEX posts_account_id_idx ON blogs.posts (account_id);
CREATE INDEX posts_account_id_locale_idx ON blogs.posts (account_id, locale);
CREATE UNIQUE INDEX posts_slug_locale_account_id_unique ON blogs.posts (slug, locale, account_id);
SELECT setup_tgr_updated_at('blogs.posts');