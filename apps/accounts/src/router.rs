use crate::accounts::CreatePublicAccountInput;
use crate::authentication::AccessToken;

use crate::extractors::AccountID;
use crate::User;
use aide::axum::routing::{get_with, post_with};
use aide::axum::{IntoApiResponse, ApiRouter};
use aide::transform::TransformOperation;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{debug_handler, Json};
use axum::{extract::State, response::IntoResponse};
use schemars::JsonSchema;
use shared::{AppError, AppState};
use sqlx::PgPool;
use validator::Validate;

#[debug_handler]
pub async fn create_account(
    State(pool): State<PgPool>, Json(body): Json<CreatePublicAccountInput>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;
    let account = crate::accounts::create_account_with_user(&pool, body).await?;
    Ok((StatusCode::CREATED, Json(account)))
}

#[derive(Debug, serde::Deserialize, Validate, JsonSchema)]
pub struct CreateUserAccessTokenWithPassword {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

pub async fn create_user_access_token_with_password(
    State(pool): State<PgPool>, AccountID(account_id): AccountID,
    Json(body): Json<CreateUserAccessTokenWithPassword>,
) -> Result<impl IntoApiResponse, AppError> {
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

pub fn create_user_access_token_with_password_docs(op: TransformOperation) -> TransformOperation {
    op.id("get_access_token_with_password")
        .description("Get an access token for a user with a password.")
        .response::<200, Json<AccessToken>>()
        .security_requirement("ApiKey")
        .tag("accounts")
}

pub async fn user_profile(
    State(_pool): State<PgPool>, user: User,
) -> Result<impl IntoApiResponse, AppError> {
    Ok((StatusCode::OK, Json(user)))
}

fn user_profile_docs(op: TransformOperation) -> TransformOperation {
    op.id("get_user_profile")
        .description("Get the profile of the current user.")
        .response::<200, Json<User>>()
        .security_requirement("ApiKey")
        .tag("accounts")
}

pub async fn list_account_keys(
    State(pool): State<PgPool>, user: User,
) -> Result<impl IntoResponse, AppError> {
    let keys = crate::accounts::get_account_keys(&pool, user.account_id).await?;
    Ok((StatusCode::OK, Json(keys)))
}

pub fn routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .route("/", post(create_account))
        .route("/account-keys", get(list_account_keys))
        .api_route(
            "/users/access-tokens/passwords",
            post_with(
                create_user_access_token_with_password,
                create_user_access_token_with_password_docs,
            ),
        )
        .api_route("/users/profiles", get_with(user_profile, user_profile_docs))        
}
