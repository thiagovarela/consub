use aide::OperationIo;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::{conflict_error, Error},
    users::CreateUserWithPasswordInput,
};

/// Account represents a single account.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub subdomain: String,    
    #[schemars(with = "AccountFeatureFlags")]
    pub feature_flags: sqlx::types::Json<AccountFeatureFlags>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AccountFeatureFlags {
    #[serde(default)]
    pub clipping_enabled: bool,
}

/// AccountKey represents a single key for an account.
/// We use this to authenticate requests to the API.
/// An account can have multiple keys.
#[derive(Debug, Clone, Serialize)]
pub struct AccountKey {
    pub id: Uuid,
    #[serde(skip)]
    pub account_id: Uuid,
    #[serde(skip)]
    pub keypair: Vec<u8>,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountInput {
    pub name: String,
    pub subdomain: String,
}

static SUBDOMAIN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z\-]+$").unwrap());

pub async fn create_account(
    conn: &mut PgConnection, input: CreateAccountInput,
) -> Result<Account, Error> {
    sqlx::query_as!(
        Account,
        r#"
        INSERT INTO accounts.accounts (name, subdomain)
        VALUES ($1, $2)
        RETURNING id, name, subdomain, 
        feature_flags as "feature_flags!: sqlx::types::Json<AccountFeatureFlags>",
        created_at, updated_at
        "#,
        input.name,
        input.subdomain
    )
    .fetch_one(conn)
    .await
    .map_err(conflict_error)
}

pub async fn create_account_key(
    conn: &mut PgConnection, account_id: Uuid,
) -> Result<AccountKey, Error> {
    let keypair: Keypair = Keypair::generate(&mut OsRng);
    Ok(sqlx::query_as!(
        AccountKey,
        r#"
        INSERT INTO accounts.account_keys (account_id, keypair)
        VALUES ($1, $2)
        RETURNING id, account_id, keypair, expires_at
        "#,
        account_id,
        &keypair.to_bytes()
    )
    .fetch_one(conn)
    .await?)
}

/// CreatePublicAccountInput is used to create a new account with a user.
/// This is used when a user signs up for a new account.
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePublicAccountInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub name: String,
    #[validate(regex = "SUBDOMAIN_REGEX")]
    pub subdomain: String,
}

pub async fn create_account_with_user(
    pool: &sqlx::PgPool, input: CreatePublicAccountInput,
) -> Result<Account, Error> {
    let account_input = CreateAccountInput {
        name: input.name,
        subdomain: input.subdomain,
    };

    let mut tx = pool.begin().await?;

    let account = crate::accounts::create_account(&mut tx, account_input).await?;
    crate::accounts::create_account_key(&mut tx, account.id).await?;

    let user_input = CreateUserWithPasswordInput {
        account_id: account.id,
        email: input.email,
        password: input.password,
    };

    crate::users::create_user_with_password(&mut tx, user_input).await?;

    tx.commit().await?;

    Ok(account)
}

pub async fn get_account_by_subdomain(
    conn: &sqlx::PgPool, subdomain: String,
) -> Result<Account, sqlx::Error> {
    sqlx::query_as!(
        Account,
        r#"
        SELECT id, name, subdomain, 
        feature_flags as "feature_flags!: sqlx::types::Json<AccountFeatureFlags>",
        created_at, updated_at
        FROM accounts.accounts
        WHERE subdomain = $1
        "#,
        subdomain
    )
    .fetch_one(conn)
    .await
}

pub async fn get_account_by_x_api_key(
    conn: &sqlx::PgPool, x_api_key: Uuid,
) -> Result<Account, sqlx::Error> {
    sqlx::query_as!(
        Account,
        r#"
        SELECT id, name, subdomain,
        feature_flags as "feature_flags!: sqlx::types::Json<AccountFeatureFlags>",
        created_at, updated_at
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
    .await
}

pub async fn get_valid_account_key(
    conn: &sqlx::PgPool, account_id: Uuid,
) -> Result<AccountKey, sqlx::Error> {
    sqlx::query_as!(
        AccountKey,
        r#"
        SELECT id, account_id, keypair, expires_at
        FROM accounts.account_keys
        WHERE account_id = $1
         AND (expires_at is null OR expires_at > now())
        ORDER BY expires_at DESC
        "#,
        account_id,
    )
    .fetch_one(conn)
    .await
}

pub async fn get_account_key_by_id(
    conn: &sqlx::PgPool, id: Uuid,
) -> Result<AccountKey, sqlx::Error> {
    sqlx::query_as!(
        AccountKey,
        r#"
        SELECT id, account_id, keypair, expires_at
        FROM accounts.account_keys        
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(conn)
    .await
}

pub async fn get_account_keys(
    conn: &sqlx::PgPool, account_id: Uuid,
) -> Result<Vec<AccountKey>, sqlx::Error> {
    sqlx::query_as!(
        AccountKey,
        r#"
        SELECT id, account_id, keypair, expires_at
        FROM accounts.account_keys
        WHERE account_id = $1
        ORDER BY expires_at DESC
        "#,
        account_id,
    )
    .fetch_all(conn)
    .await
}
