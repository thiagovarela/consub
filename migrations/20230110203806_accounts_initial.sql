CREATE SCHEMA accounts;
-- This is the main account table. 
-- Most of the other tables should use an account_id to reference this table.
-- The subdomain is used to identify the account in the URL.
CREATE TABLE accounts.accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    subdomain VARCHAR(100) UNIQUE NOT NULL,    
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT setup_tgr_updated_at('accounts.accounts');

-- This table is used to store the keys for the account. 
-- The secret_key is used to cryptographically sign tokens.
CREATE TABLE accounts.account_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    public_key BYTEA NOT NULL,
    secret_key BYTEA NOT NULL,
    expires_at TIMESTAMP
);
CREATE INDEX account_keys_account_id_idx ON accounts.account_keys (account_id);
SELECT setup_tgr_updated_at('accounts.account_keys');


CREATE TABLE accounts.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX user_account_id_idx ON accounts.users (account_id);
SELECT setup_tgr_updated_at('accounts.users');

-- This table is used to allow email/password identity. 
-- The account id is denormalized to allow uniqueness of email per account.
CREATE TABLE accounts.passwords (
    user_id UUID NOT NULL REFERENCES accounts.users (id),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    email VARCHAR(255) NOT NULL,
    hash_password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, account_id)
);
CREATE INDEX passwords_user_id_idx ON accounts.passwords (user_id);
CREATE UNIQUE INDEX passwords_account_id_email_idx ON accounts.passwords (account_id, email);
SELECT setup_tgr_updated_at('accounts.passwords');