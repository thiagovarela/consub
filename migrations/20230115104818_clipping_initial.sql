-- Add migration script here

CREATE SCHEMA clippings;

CREATE TABLE clippings.categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    slug TEXT NOT NULL,    
    locale VARCHAR(5) NOT NULL,    
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX categories_id_idx ON clippings.categories (id);
CREATE INDEX categories_account_id_idx ON clippings.categories (account_id);
CREATE INDEX categories_account_id_locale_idx ON clippings.categories (account_id, locale);
CREATE UNIQUE INDEX categories_slug_locale_account_id_unique ON clippings.categories (slug, locale, account_id);
SELECT setup_tgr_updated_at('clippings.categories');

CREATE TABLE clippings.items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    created_by_id UUID NOT NULL REFERENCES accounts.users (id),
    title TEXT NOT NULL,
    slug TEXT NOT NULL,    
    body JSONB NOT NULL,    
    locale VARCHAR(5) NOT NULL,
    short_description TEXT,
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,
    tags TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
    source TEXT NOT NULL,
    source_url TEXT NOT NULL,
    source_published_at TIMESTAMP NOT NULL,    
    category_id UUID REFERENCES clippings.categories (id),
    reading_time_minutes INTEGER,
    published_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);
CREATE INDEX items_id_idx ON clippings.items (id);
CREATE INDEX items_account_id_idx ON clippings.items (account_id);
CREATE INDEX items_account_id_locale_idx ON clippings.items (account_id, locale);
SELECT setup_tgr_updated_at('clippings.items');
