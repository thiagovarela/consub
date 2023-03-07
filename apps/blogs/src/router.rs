use accounts::User;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post};
use axum::{debug_handler, middleware, Json};
use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::Query as ExtraQuery;
use shared::AppState;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::categories::{CategoryQuery, ChangeCategoryInput, CreateCategoryInput};
use crate::error::Error;
use crate::posts::{ChangePostInput, CreatePostInput, PostQuery};

#[debug_handler]
pub async fn create_post(
    State(pool): State<PgPool>, user: User, Json(body): Json<CreatePostInput>,
) -> Result<impl IntoResponse, Error> {
    body.validate()?;
    let mut conn = pool.acquire().await?;
    let post = crate::posts::create_post(&mut conn, user.account_id, user.id, body).await?;
    Ok((StatusCode::CREATED, Json(post)))
}

#[debug_handler]
pub async fn list_posts(
    State(pool): State<PgPool>, user: User, ExtraQuery(query): ExtraQuery<PostQuery>,
) -> Result<impl IntoResponse, Error> {
    let mut conn = pool.acquire().await?;
    let posts = crate::posts::list_posts(&mut conn, user.account_id, query).await?;
    Ok((StatusCode::CREATED, Json(posts)))
}

#[debug_handler]
pub async fn get_post(
    State(pool): State<PgPool>, user: User, Path(post_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let mut conn = pool.acquire().await?;
    let post = crate::posts::get_post(&mut conn, user.account_id, post_id).await?;
    Ok((StatusCode::CREATED, Json(post)))
}

#[debug_handler]
pub async fn change_post(
    State(pool): State<PgPool>, user: User, Path(post_id): Path<Uuid>,
    Json(body): Json<ChangePostInput>,
) -> Result<impl IntoResponse, Error> {
    let mut conn = pool.acquire().await?;
    let post = crate::posts::change_post(&mut conn, user.account_id, post_id, body).await?;
    Ok((StatusCode::OK, Json(post)))
}

#[debug_handler]
pub async fn create_category(
    State(pool): State<PgPool>, user: User, Json(body): Json<CreateCategoryInput>,
) -> Result<impl IntoResponse, Error> {
    let mut conn = pool.acquire().await?;
    let category = crate::categories::create_category(&mut conn, user.account_id, body).await?;
    Ok((StatusCode::CREATED, Json(category)))
}

#[debug_handler]
pub async fn list_categories(
    State(pool): State<PgPool>, user: User, Query(query): Query<CategoryQuery>,
) -> Result<impl IntoResponse, Error> {
    let mut conn = pool.acquire().await?;
    let categories = crate::categories::list_categories(&mut conn, user.account_id, query).await?;
    Ok((StatusCode::CREATED, Json(categories)))
}

#[debug_handler]
pub async fn get_category(
    State(pool): State<PgPool>, user: User, Path(category_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let mut conn = pool.acquire().await?;
    let category =
        crate::categories::get_category_by_id(&mut conn, category_id, user.account_id).await?;
    Ok((StatusCode::CREATED, Json(category)))
}

#[debug_handler]
pub async fn change_category(
    State(pool): State<PgPool>, user: User, Path(category_id): Path<Uuid>,
    Json(body): Json<ChangeCategoryInput>,
) -> Result<impl IntoResponse, Error> {
    let mut conn = pool.acquire().await?;
    let category =
        crate::categories::change_category(&mut conn, category_id, user.account_id, body).await?;
    Ok((StatusCode::CREATED, Json(category)))
}

#[debug_handler]
pub async fn delete_category(
    State(pool): State<PgPool>, user: User, Path(category_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let mut conn = pool.acquire().await?;
    crate::categories::delete_category(&mut conn, category_id, user.account_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn routes(app_state: AppState) -> aide::axum::ApiRouter {
    let admin = aide::axum::ApiRouter::new()
        .route("/categories", post(create_category))
        .route("/categories", get(list_categories))
        .route("/categories/:category_id", get(get_category))
        .route("/categories/:category_id", patch(change_category))
        .route("/categories/:category_id", delete(delete_category))
        .route("/posts", post(create_post))
        .route("/posts", get(list_posts))
        .route("/posts/:post_id", get(get_post))
        .route("/posts/:post_id", patch(change_post))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            accounts::authorization_layer,
        ));

    aide::axum::ApiRouter::new()
        .nest("/admin", admin)
        .with_state(app_state)
}
