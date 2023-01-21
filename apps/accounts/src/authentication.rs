use crate::passwords::verify_password;
use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    accounts::{get_account_key_by_id, get_valid_account_key},
    error::Error,
    users::{get_user_by_id, User},
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: Uuid,
    account_id: Uuid,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub access_key_id: Uuid,
}

pub fn create_access_token(
    user_id: Uuid,
    account_id: Uuid,
    secret_key: Vec<u8>,
) -> Result<(String, DateTime<Utc>), Error> {
    let key = &EncodingKey::from_secret(&secret_key);

    // TODO: Make expiry configurable
    let expiry_date = chrono::Utc::now() + chrono::Duration::days(7);

    let claims = Claims {
        user_id,
        account_id,
        exp: expiry_date.timestamp() as usize,
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, key)?;

    Ok((token, expiry_date))
}

fn verify_access_token(token: &str, secret_key: Vec<u8>) -> Result<Claims, Error> {
    let claims = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&secret_key),
        &Validation::default(),
    )?
    .claims;

    Ok(claims)
}

pub async fn authenticate_user_with_password(
    conn: &sqlx::PgPool,
    account_id: Uuid,
    email: String,
    password: String,
) -> Result<AccessToken, Error> {
    let row = sqlx::query!(
        r#"
        SELECT user_id, account_id, hash_password
        FROM accounts.passwords
        WHERE email = $1 and account_id = $2
        "#,
        email,
        account_id,
    )
    .fetch_one(conn)
    .await
    .map_err(|_| Error::AuthPasswordError)?;

    verify_password(password, row.hash_password)?;

    let account_key = get_valid_account_key(&conn, account_id).await?;

    let (token, expires_at) = create_access_token(row.user_id, account_id, account_key.secret_key)?;

    Ok(AccessToken {
        token,
        expires_at,
        access_key_id: account_key.id,
    })
}

pub async fn get_user_from_bearer_token(
    conn: &sqlx::PgPool,
    token: String,
    account_key_id: Uuid,
    account_id: Uuid,
) -> Result<User, Error> {
    let account_key = get_account_key_by_id(conn, account_key_id).await?;
    let user_claims = verify_access_token(token.as_str(), account_key.secret_key)?;
    if user_claims.account_id != account_id {
        return Err(Error::InvalidAccountID(
            "Provided account ID does not match token account ID".into(),
        ));
    }
    let user = get_user_by_id(conn, user_claims.user_id).await?;
    Ok(user)
}
