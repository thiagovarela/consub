use addr::parse_domain_name;
use aide::OperationIo;
use axum::{
    extract::{FromRef, FromRequestParts, Host},
    http::request::Parts,
};

use axum_auth::AuthBearer;
use serde::Serialize;

use shared::AppError;
use sqlx::PgPool;

use crate::{
    authentication::{get_claims_from_bearer_token, ConsubClaims},
    users::get_user_by_id,
    Account, User,
};
use anyhow::anyhow;
use uuid::Uuid;

use crate::accounts::{get_account_by_subdomain, get_account_by_x_api_key};

#[derive(Debug, Clone)]
pub struct APIKey(pub Uuid);

#[derive(Debug, Serialize, OperationIo)]
pub struct AccountID(pub Uuid);

#[axum::async_trait]
impl<S> FromRequestParts<S> for AccountID
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    /// Extracts the account ID from the request.
    /// The x-api-key takes precedence to fetch the account ID.    
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Tries to fetch by X-API-KEY header.
        if let Ok(APIKey(x_api_key)) = APIKey::from_request_parts(parts, state).await {
            let pool = PgPool::from_ref(state);
            if let Ok(account) = get_account_by_x_api_key(&pool, x_api_key).await {
                return Ok(AccountID(account.id));
            }
        }

        let Host(hostname) = Host::from_request_parts(parts, state).await?;

        let domain = parse_domain_name(&hostname)
            .map_err(|_| AppError::BadRequest("Invalid domain name".into()))?;
        let prefix = match domain.prefix() {
            Some(prefix) => prefix,
            None => return Err(AppError::BadRequest("Invalid domain name".into())),
        };
        // Tries to fetch using the subdomain.
        let pool = PgPool::from_ref(state);
        if let Ok(account) = get_account_by_subdomain(&pool, prefix.to_string()).await {
            return Ok(AccountID(account.id));
        }

        Err(AppError::BadRequest("AccountID is not valid".into()))
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for User
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let claims = ConsubClaims::from_request_parts(parts, state).await?;
        let pool = PgPool::from_ref(state);
        let user = get_user_by_id(&pool, claims.user_id).await?;
        Ok(user)
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for APIKey
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        const X_API_KEY_HEADER_KEY: &str = "X-Api-Key";
        if let Some(api_key) = parts
            .headers
            .get(X_API_KEY_HEADER_KEY)
            .and_then(|api_key| api_key.to_str().ok())
            .and_then(|api_key| Uuid::parse_str(api_key).ok())
        {
            return Ok(APIKey(api_key));
        }

        Err(anyhow!("API Key is not valid").into())
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for ConsubClaims
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AuthBearer(token) = AuthBearer::from_request_parts(parts, state)
            .await
            .map_err(|_| anyhow!("Unable to get authorization bearer token"))?;
        let pool = PgPool::from_ref(state);
        let claims = get_claims_from_bearer_token(&pool, token).await?;
        Ok(claims)
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Account
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    /// Extracts the account from the x-api-key.
    /// The x-api-key takes precedence to fetch the account ID.    
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Tries to fetch by X-API-KEY header.
        if let Ok(APIKey(x_api_key)) = APIKey::from_request_parts(parts, state).await {
            let pool = PgPool::from_ref(state);
            if let Ok(account) = get_account_by_x_api_key(&pool, x_api_key).await {
                return Ok(account);
            }
        }
        Err(anyhow!("Account is not valid").into())
    }
}
