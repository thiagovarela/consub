-- Add migration script here

CREATE SCHEMA clipping;

CREATE TABLE clipping.items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    created_by_id UUID NOT NULL REFERENCES accounts.users (id),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    summary TEXT NOT NULL,
    body TEXT NOT NULL,
    locale VARCHAR(5) NOT NULL,
    source VARCHAR(255) NOT NULL,
    source_url VARCHAR(255) NOT NULL,
    source_published_at DATE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    featured BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);
CREATE INDEX items_id_idx ON clipping.items (id);
CREATE INDEX items_account_id_idx ON clipping.items (account_id);
CREATE INDEX items_account_id_locale_idx ON clipping.items (account_id, locale);
SELECT setup_tgr_updated_at('clipping.items');
