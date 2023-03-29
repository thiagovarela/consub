CREATE SCHEMA accounts;
-- This is the main account table. 
-- Most of the other tables should use an account_id to reference this table.
-- The subdomain is used to identify the account in the URL.
CREATE TABLE accounts.accounts (
    id UUID PRIMARY KEY DEFAULT generate_ulid_uuid(),
    name VARCHAR(100) NOT NULL,
    subdomain VARCHAR(100) UNIQUE NOT NULL,    
    feature_flags JSONB NOT NULL DEFAULT '{}',
    origin TEXT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT setup_tgr_updated_at('accounts.accounts');

-- This table is used to store the keys for the account. 
-- The secret_key is used to cryptographically sign tokens.
CREATE TABLE accounts.account_keys (
    id UUID PRIMARY KEY DEFAULT generate_ulid_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    keypair BYTEA NOT NULL,    
    expires_at TIMESTAMP
);
CREATE INDEX account_keys_account_id_idx ON accounts.account_keys (account_id);
SELECT setup_tgr_updated_at('accounts.account_keys');


CREATE TABLE accounts.users (
    id UUID PRIMARY KEY DEFAULT generate_ulid_uuid(),
    account_id UUID NOT NULL REFERENCES accounts.accounts (id),
    email VARCHAR(255) NOT NULL,    
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX user_account_id_idx ON accounts.users (account_id);
CREATE UNIQUE INDEX user_account_id_email_unique ON accounts.users (account_id, email);
SELECT setup_tgr_updated_at('accounts.users');


-- This table is used to allow email/password identity. 
CREATE TABLE accounts.passwords (
    user_id UUID NOT NULL REFERENCES accounts.users (id),    
    hash_password VARCHAR(255) NOT NULL,    
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id)
);
CREATE INDEX passwords_user_id_idx ON accounts.passwords (user_id);
SELECT setup_tgr_updated_at('accounts.passwords');