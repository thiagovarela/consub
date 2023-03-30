use crate::passwords::verify_password;
use anyhow::anyhow;
use axum::{http::Request, middleware::Next, response::Response};
use chrono::{DateTime, Utc};
use ed25519_compact::KeyPair;
use jwt_compact::{alg::Ed25519, prelude::*, Algorithm};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shared::AppError;
use uuid::Uuid;

use crate::accounts::{get_account_key_by_id, get_valid_account_key};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsubClaims {
    pub user_id: Uuid,
    pub account_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

type EdSigningKey = <Ed25519 as Algorithm>::SigningKey;
type EdVerifyingKey = <Ed25519 as Algorithm>::VerifyingKey;

pub fn create_access_token(
    user_id: Uuid, account_id: Uuid, account_key_id: Uuid, secret_key: Vec<u8>,
) -> Result<(String, DateTime<Utc>), anyhow::Error> {
    let header = Header::default().with_key_id(account_key_id.to_string());
    let key = EdSigningKey::from_slice(&secret_key).unwrap();

    let claims = ConsubClaims {
        user_id,
        account_id,
    };

    let time_options = TimeOptions::default();
    let claims = Claims::new(claims)
        .set_not_before(Utc::now())
        .set_duration(&time_options, chrono::Duration::days(7));

    let token = Ed25519::with_specific_name()
        .token(header, &claims, &key)
        .unwrap();

    Ok((token, claims.expiration.unwrap()))
}

pub async fn authenticate_user_with_password(
    conn: &sqlx::PgPool, account_id: Uuid, email: String, password: String,
) -> Result<AccessToken, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT u.id as user_id, u.account_id, p.hash_password
        FROM accounts.passwords p
        INNER JOIN accounts.users u ON (u.id = p.user_id)
        WHERE u.email = $1 and u.account_id = $2
        "#,
        email,
        account_id,
    )
    .fetch_one(conn)
    .await?;

    verify_password(password, row.hash_password)?;

    let account_key = get_valid_account_key(conn, account_id).await?;

    let (token, expires_at) =
        create_access_token(row.user_id, account_id, account_key.id, account_key.keypair)?;

    Ok(AccessToken { token, expires_at })
}

pub async fn get_claims_from_bearer_token(
    conn: &sqlx::PgPool, token: String,
) -> Result<ConsubClaims, AppError> {
    let token = UntrustedToken::new(&token)?;
    let account_key_id = token
        .header()
        .key_id
        .as_deref()
        .ok_or(AppError::BadRequest(
            "Unable to retrieve the account key id from the access token".into(),
        ))?;

    let account_key_id = Uuid::parse_str(account_key_id)
        .map_err(|_| AppError::BadRequest("Access key id is not in the expected format".into()))?;

    let account_key = get_account_key_by_id(conn, account_key_id).await?;
    let keypair = KeyPair::from_slice(&account_key.keypair).map_err(|_| {
        anyhow!(format!(
            "Unable to retrieve a valid keypair for the account key id {account_key_id}"
        ))
    })?;
    let key = EdVerifyingKey::from_slice(keypair.pk.as_ref()).map_err(|_| {
        anyhow!(format!(
            "Unable to retrieve a valid public key for the account key id {account_key_id}"
        ))
    })?;

    let token: Token<ConsubClaims> = Ed25519::with_specific_name()
        .validate_integrity(&token, &key)
        .map_err(|_| {
            AppError::Unauthorized("Unable to validate the integrity of the access token".into())
        })?;

    let claims = &token.claims().custom;
    Ok(claims.to_owned())
}

pub async fn authorization_layer<B>(
    _claims: ConsubClaims, request: Request<B>, next: Next<B>,
) -> Result<Response, AppError> {
    let response = next.run(request).await;
    Ok(response)
}
