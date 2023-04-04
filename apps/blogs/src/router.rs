use accounts::User;
use aide::axum::routing::{delete_with, get_with, patch_with, post_with};
use aide::axum::{IntoApiResponse, ApiRouter};
use aide::transform::TransformOperation;
use axum::extract::State;
use axum::extract::{Path, Query};
use axum::http::StatusCode;

use axum::{debug_handler, Json};
use axum_extra::extract::Query as ExtraQuery;
use schemars::JsonSchema;
use shared::{AppError, AppState};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::categories::{Category, CategoryQuery, ChangeCategoryInput, CreateCategoryInput};
use crate::post_images::{ChangePostImageInput, CreatePostImageInput, PostImage};
use crate::posts::{ChangePostInput, CreatePostInput, Post, PostQuery};

/// routes for posts

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct PathPost {
    pub post_id: Uuid,
}

#[debug_handler]
pub async fn create_post(
    State(pool): State<PgPool>, user: User, Json(body): Json<CreatePostInput>,
) -> Result<impl IntoApiResponse, AppError> {
    body.validate()?;
    let mut conn = pool.acquire().await?;
    let post = crate::posts::create_post(&mut conn, user.account_id, user.id, body).await?;
    Ok((StatusCode::CREATED, Json(post)))
}

pub fn create_post_docs(op: TransformOperation) -> TransformOperation {
    op.id("create_post")
        .description("Create a new post")
        .response::<201, Json<Post>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

#[debug_handler]
pub async fn list_posts(
    State(pool): State<PgPool>, user: User, ExtraQuery(query): ExtraQuery<PostQuery>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let posts = crate::posts::list_posts(&mut conn, user.account_id, query).await?;
    Ok((StatusCode::CREATED, Json(posts)))
}

pub fn list_posts_docs(op: TransformOperation) -> TransformOperation {
    op.id("list_posts")
        .description("List posts")
        .response::<200, Json<Vec<Post>>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

#[debug_handler]
pub async fn get_post(
    State(pool): State<PgPool>, user: User, Path(path_post): Path<PathPost>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let post = crate::posts::get_post(&mut conn, user.account_id, path_post.post_id).await?;
    Ok((StatusCode::CREATED, Json(post)))
}

pub fn get_post_docs(op: TransformOperation) -> TransformOperation {
    op.id("get_post")
        .description("Get a single post")
        .response::<200, Json<Post>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

#[debug_handler]
pub async fn change_post(
    State(pool): State<PgPool>, user: User, Path(path_post): Path<PathPost>,
    Json(body): Json<ChangePostInput>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let post =
        crate::posts::change_post(&mut conn, user.account_id, path_post.post_id, body).await?;
    Ok((StatusCode::OK, Json(post)))
}

pub fn change_post_docs(op: TransformOperation) -> TransformOperation {
    op.id("change_post")
        .description("Change a post")
        .response::<200, Json<Post>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

/// routes for categories

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct PathCategory {
    pub category_id: Uuid,
}

#[debug_handler]
pub async fn create_category(
    State(pool): State<PgPool>, user: User, Json(body): Json<CreateCategoryInput>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let category = crate::categories::create_category(&mut conn, user.account_id, body).await?;
    Ok((StatusCode::CREATED, Json(category)))
}

pub fn create_category_docs(op: TransformOperation) -> TransformOperation {
    op.id("create_post_category")
        .description("Create a new post category")
        .response::<201, Json<Category>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

#[debug_handler]
pub async fn list_categories(
    State(pool): State<PgPool>, user: User, Query(query): Query<CategoryQuery>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let categories = crate::categories::list_categories(&mut conn, user.account_id, query).await?;
    Ok((StatusCode::OK, Json(categories)))
}

pub fn list_categories_docs(op: TransformOperation) -> TransformOperation {
    op.id("list_post_categories")
        .description("List post categories")
        .response::<201, Json<Vec<Category>>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

#[debug_handler]
pub async fn get_category(
    State(pool): State<PgPool>, user: User, Path(path): Path<PathCategory>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let category =
        crate::categories::get_category_by_id(&mut conn, path.category_id, user.account_id).await?;
    Ok((StatusCode::OK, Json(category)))
}

pub fn get_category_docs(op: TransformOperation) -> TransformOperation {
    op.id("get_post_category_by_id")
        .description("Get post category")
        .response::<201, Json<Category>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

#[debug_handler]
pub async fn change_category(
    State(pool): State<PgPool>, user: User, Path(path): Path<PathCategory>,
    Json(body): Json<ChangeCategoryInput>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let category =
        crate::categories::change_category(&mut conn, path.category_id, user.account_id, body)
            .await?;
    Ok((StatusCode::OK, Json(category)))
}

pub fn change_category_docs(op: TransformOperation) -> TransformOperation {
    op.id("change_post_category")
        .description("Change post category")
        .response::<200, Json<Category>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

#[debug_handler]
pub async fn delete_category(
    State(pool): State<PgPool>, user: User, Path(category_id): Path<Uuid>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    crate::categories::delete_category(&mut conn, category_id, user.account_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn delete_category_docs(op: TransformOperation) -> TransformOperation {
    op.id("delete_post_category")
        .description("Change post category")
        .response::<204, ()>()
        .security_requirement("Bearer")
        .tag("blogs")
}

/// routes for post images

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct PathPostImage {
    pub post_id: Uuid,
    pub post_image_id: Uuid,
}

#[debug_handler]
pub async fn create_post_image(
    State(pool): State<PgPool>, _user: User, Path(path_post): Path<PathPost>,
    Json(body): Json<CreatePostImageInput>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    // TODO: check if user is allowed to create post image
    let image = crate::post_images::create_post_image(&mut conn, path_post.post_id, body).await?;
    Ok((StatusCode::CREATED, Json(image)))
}

pub fn create_post_image_docs(op: TransformOperation) -> TransformOperation {
    op.id("create_post_images")
        .description("Create post image")
        .response::<201, Json<PostImage>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

#[debug_handler]
pub async fn list_post_images(
    State(pool): State<PgPool>, user: User, Path(path_post): Path<PathPost>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let images =
        crate::post_images::list_post_images(&mut conn, user.account_id, path_post.post_id).await?;
    Ok((StatusCode::CREATED, Json(images)))
}

pub fn list_post_images_docs(op: TransformOperation) -> TransformOperation {
    op.id("list_post_images")
        .description("List post images")
        .response::<200, Json<Vec<PostImage>>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

#[debug_handler]
pub async fn change_post_image(
    State(pool): State<PgPool>, user: User, Path(post_image_path): Path<PathPostImage>,
    Json(body): Json<ChangePostImageInput>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let post_image = crate::post_images::change_post_image(
        &mut conn,
        post_image_path.post_image_id,
        user.account_id,
        body,
    )
    .await?;
    Ok((StatusCode::OK, Json(post_image)))
}

pub fn change_post_image_docs(op: TransformOperation) -> TransformOperation {
    op.id("change_post_image")
        .description("Change post image")
        .response::<200, Json<PostImage>>()
        .security_requirement("Bearer")
        .tag("blogs")
}

#[debug_handler]
pub async fn delete_post_image(
    State(pool): State<PgPool>, user: User, Path(post_image_path): Path<PathPostImage>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    crate::post_images::delete_post_image(
        &mut conn,
        post_image_path.post_image_id,
        user.account_id,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn delete_post_image_docs(op: TransformOperation) -> TransformOperation {
    op.id("delete_post_image")
        .description("Delete post image")
        .response::<204, ()>()
        .security_requirement("Bearer")
        .tag("blogs")
}

pub fn routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/categories",
            post_with(create_category, create_category_docs),
        )
        .api_route(
            "/categories",
            get_with(list_categories, list_categories_docs),
        )
        .api_route(
            "/categories/:category_id",
            get_with(get_category, get_category_docs),
        )
        .api_route(
            "/categories/:category_id",
            patch_with(change_category, change_category_docs),
        )
        .api_route(
            "/categories/:category_id",
            delete_with(delete_category, delete_category_docs),
        )
        .api_route("/posts", post_with(create_post, create_post_docs))
        .api_route("/posts", get_with(list_posts, list_posts_docs))
        .api_route("/posts/:post_id", get_with(get_post, get_post_docs))
        .api_route("/posts/:post_id", patch_with(change_post, change_post_docs))
        .api_route(
            "/posts/:post_id/images",
            get_with(list_post_images, list_post_images_docs),
        )
        .api_route(
            "/posts/:post_id/images",
            post_with(create_post_image, create_post_image_docs),
        )
        .api_route(
            "/posts/:post_id/images/:post_image_id",
            patch_with(change_post_image, change_post_image_docs),
        )
        .api_route(
            "/posts/:post_id/images/:post_image_id",
            delete_with(delete_post_image, delete_post_image_docs),
        )
        .into()
}
