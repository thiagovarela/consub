use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{debug_handler, Json};
use axum::{extract::State, response::IntoResponse, Router};
use regex::Regex;
use shared::AppState;
use sqlx::PgPool;
use validator::Validate;

use crate::accounts::CreateAccountInput;
use crate::error::Error;
use crate::extractors::AccountID;
use crate::users::CreateUserWithPasswordInput;
use crate::User;
use once_cell::sync::Lazy;

static SUBDOMAIN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z\-]+$").unwrap());

#[derive(Debug, serde::Deserialize, Validate)]
pub struct CreatePublicAccountInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub name: String,
    #[validate(regex = "SUBDOMAIN_REGEX")]
    pub subdomain: String,
}

#[debug_handler]
#[tracing::instrument(skip(pool, body))]
pub async fn create_account(
    State(pool): State<PgPool>,
    Json(body): Json<CreatePublicAccountInput>,
) -> Result<impl IntoResponse, Error> {
    body.validate()?;

    let account_input = CreateAccountInput {
        name: body.name,
        subdomain: body.subdomain,
    };

    let mut tx = pool.begin().await?;

    let account = crate::accounts::create_account(&mut tx, account_input).await?;
    crate::accounts::create_account_key(&mut tx, account.id).await?;

    let user_input = CreateUserWithPasswordInput {
        account_id: account.id,
        email: body.email,
        password: body.password,
    };

    crate::users::create_user_with_password(&mut tx, user_input).await?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(account)))
}

#[derive(Debug, serde::Deserialize, Validate)]
pub struct CreateUserAccessTokenWithPassword {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[debug_handler]
pub async fn create_user_access_token_with_password(
    State(pool): State<PgPool>,
    AccountID(account_id): AccountID,
    Json(body): Json<CreateUserAccessTokenWithPassword>,
) -> Result<impl IntoResponse, Error> {
    body.validate()?;
    let access_token = crate::authentication::authenticate_user_with_password(
        &pool,
        account_id,
        body.email,
        body.password,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(access_token)))
}

#[debug_handler]
pub async fn user_profile(
    State(_pool): State<PgPool>,
    user: User,
) -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, Json(user)))
}

pub fn routes(app_state: AppState) -> Router {
    let protected = Router::new().route("/users/profile", get(user_profile));

    let public = Router::new().route("/", post(create_account)).route(
        "/users/access-tokens/passwords",
        post(create_user_access_token_with_password),
    );

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(app_state)
}
