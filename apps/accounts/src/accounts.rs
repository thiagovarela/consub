use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::error::{conflict_error, Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub subdomain: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountKey {
    pub id: Uuid,
    pub account_id: Uuid,
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountInput {
    pub name: String,
    pub subdomain: String,
}

pub async fn create_account(
    conn: &mut PgConnection,
    input: CreateAccountInput,
) -> Result<Account, Error> {
    Ok(sqlx::query_as!(
        Account,
        r#"
        INSERT INTO accounts.accounts (name, subdomain)
        VALUES ($1, $2)
        RETURNING *
        "#,
        input.name,
        input.subdomain
    )
    .fetch_one(conn)
    .await
    .map_err(|e| conflict_error(e, "accounts_subdomain_key", "Subdomain already in use"))?)
}

pub async fn create_account_key(
    conn: &mut PgConnection,
    account_id: Uuid,
) -> Result<AccountKey, Error> {
    let key_pair: Keypair = Keypair::generate(&mut OsRng);

    let public = key_pair.public.as_bytes();
    let secret = key_pair.secret.as_bytes();

    Ok(sqlx::query_as!(
        AccountKey,
        r#"
        INSERT INTO accounts.account_keys (account_id, public_key, secret_key)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        account_id,
        public,
        secret
    )
    .fetch_one(conn)
    .await?)
}

pub async fn get_account_by_subdomain(
    conn: &sqlx::PgPool,
    subdomain: String,
) -> Result<Account, sqlx::Error> {
    Ok(sqlx::query_as!(
        Account,
        r#"
        SELECT *
        FROM accounts.accounts
        WHERE subdomain = $1
        "#,
        subdomain
    )
    .fetch_one(conn)
    .await?)
}

pub async fn get_account_by_x_api_key(
    conn: &sqlx::PgPool,
    x_api_key: Uuid,
) -> Result<Account, sqlx::Error> {
    Ok(sqlx::query_as!(
        Account,
        r#"
        SELECT *
        FROM accounts.accounts
        WHERE id = (
            SELECT account_id
            FROM accounts.account_keys
            WHERE id = $1 and (expires_at is null or expires_at > now())
        )
        "#,
        x_api_key
    )
    .fetch_one(conn)
    .await?)
}

pub async fn get_valid_account_key(
    conn: &sqlx::PgPool,
    account_id: Uuid,
) -> Result<AccountKey, Error> {
    let account_key = sqlx::query_as!(
        AccountKey,
        r#"
        SELECT *
        FROM accounts.account_keys
        WHERE account_id = $1 AND (expires_at is null OR expires_at > now())
        ORDER BY expires_at DESC
        "#,
        account_id,
    )
    .fetch_one(conn)
    .await?;

    Ok(account_key)
}

pub async fn get_account_key_by_id(conn: &sqlx::PgPool, id: Uuid) -> Result<AccountKey, Error> {
    let account_key = sqlx::query_as!(
        AccountKey,
        r#"
        SELECT *
        FROM accounts.account_keys
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(conn)
    .await?;

    Ok(account_key)
}
