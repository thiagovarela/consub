use addr::parse_domain_name;
use aide::OperationIo;
use axum::{
    extract::{FromRef, FromRequestParts, Host},
    http::request::Parts,
};

use axum_auth::AuthBearer;
use serde::Serialize;

use sqlx::PgPool;

use crate::{
    authentication::{get_claims_from_bearer_token, ConsubClaims},
    error::Error,
    users::get_user_by_id,
    Account, User,
};
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
    type Rejection = Error;

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

        let Host(hostname) = Host::from_request_parts(parts, state)
            .await
            .map_err(|_| Error::InvalidAccountID("Unable to parse the hostname".into()))?;
        let domain = parse_domain_name(&hostname)
            .map_err(|_| Error::InvalidAccountID("Provided domain is not valid".into()))?;
        let prefix = match domain.prefix() {
            Some(prefix) => prefix,
            None => {
                return Err(Error::InvalidAccountID(
                    "Provided domain is not valid".into(),
                ))
            }
        };
        // Tries to fetch using the subdomain.
        let pool = PgPool::from_ref(state);
        if let Ok(account) = get_account_by_subdomain(&pool, prefix.to_string()).await {
            return Ok(AccountID(account.id));
        }

        Err(Error::InvalidAccountID("Account ID was not found".into()))
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for User
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

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
    type Rejection = Error;

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

        Err(Error::InvalidAPIKey("Invalid X-API-KEY header".into()))
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for ConsubClaims
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AuthBearer(token) = AuthBearer::from_request_parts(parts, state)
            .await
            .map_err(|_| Error::AuthBearer)?;
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
    type Rejection = Error;

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
        Err(Error::InvalidAccountID("Account was not found".into()))
    }
}
