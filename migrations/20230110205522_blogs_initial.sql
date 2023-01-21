CREATE SCHEMA blogs;

CREATE TABLE blogs.categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,    
    locale VARCHAR(5) NOT NULL,
    translation_of UUID NULL REFERENCES blogs.categories (id),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX categories_id_idx ON blogs.categories (id);
CREATE INDEX categories_account_id_idx ON blogs.categories (account_id);
CREATE INDEX categories_account_id_locale_idx ON blogs.categories (account_id, locale);
CREATE UNIQUE INDEX categories_slug_account_id_idx ON blogs.categories (slug, account_id);
SELECT setup_tgr_updated_at('blogs.categories');


CREATE TABLE blogs.posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    author_id UUID NOT NULL REFERENCES accounts.users (id),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    locale VARCHAR(5) NOT NULL,
    category_id UUID NULL REFERENCES blogs.categories (id),
    featured BOOLEAN NOT NULL DEFAULT FALSE,
    estimated_reading_time INTEGER NOT NULL,
    translation_of UUID NULL REFERENCES blogs.posts (id),  
    published_at TIMESTAMP,  
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP    
);
CREATE INDEX posts_id_idx ON blogs.posts (id);
CREATE INDEX posts_account_id_idx ON blogs.posts (account_id);
CREATE INDEX posts_account_id_locale_idx ON blogs.posts (account_id, locale);
CREATE UNIQUE INDEX posts_slug_account_id_idx ON blogs.posts (slug, account_id);
SELECT setup_tgr_updated_at('blogs.posts');