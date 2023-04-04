use accounts::User;
use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, patch_with, post_with};
use aide::{
    axum::{routing::get_with, IntoApiResponse},
    transform::TransformOperation,
};
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
use crate::items::{
    ChangeClippingItemInput, ClippingItem, ClippingItemQuery, CreateClippingItemInput,
};

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct PathCategory {
    pub category_id: Uuid,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct PathItem {
    pub item_id: Uuid,
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
    op.id("create_category")
        .description("Create a new category")
        .response::<201, Json<Category>>()
        .security_requirement("Bearer")
        .tag("clippings")
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
    op.id("list_categories")
        .description("List categories")
        .response::<200, Json<Vec<Category>>>()
        .security_requirement("Bearer")
        .tag("clippings")
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
    op.id("get_category_by_id")
        .description("Get category")
        .response::<200, Json<Category>>()
        .tag("clippings")
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
    op.id("change_category")
        .description("Change category")
        .response::<200, Json<Category>>()
        .security_requirement("Bearer")
        .tag("clippings")
}

#[debug_handler]
pub async fn delete_category(
    State(pool): State<PgPool>, user: User, Path(path): Path<PathCategory>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    crate::categories::delete_category(&mut conn, path.category_id, user.account_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn delete_category_docs(op: TransformOperation) -> TransformOperation {
    op.id("delete_category")
        .description("Delete category")
        .response::<204, ()>()
        .security_requirement("Bearer")
        .tag("clippings")
}

#[debug_handler]
pub async fn create_clipping_item(
    State(pool): State<PgPool>, user: User, Json(body): Json<CreateClippingItemInput>,
) -> Result<impl IntoApiResponse, AppError> {
    body.validate()?;
    let mut conn = pool.acquire().await?;
    let item =
        crate::items::create_clipping_item(&mut conn, user.account_id, user.id, body).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub fn create_clipping_item_docs(op: TransformOperation) -> TransformOperation {
    op.id("create_item")
        .description("Create a new clipping item")
        .response::<201, Json<ClippingItem>>()
        .security_requirement("Bearer")
        .tag("clippings")
}

#[debug_handler]
pub async fn list_clipping_items(
    State(pool): State<PgPool>, user: User, ExtraQuery(query): ExtraQuery<ClippingItemQuery>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let items = crate::items::list_clipping_items(&mut conn, user.account_id, query).await?;
    Ok((StatusCode::OK, Json(items)))
}
pub fn list_clipping_items_docs(op: TransformOperation) -> TransformOperation {
    op.id("list_items")
        .description("List clipping items")
        .response::<201, Json<Vec<ClippingItem>>>()
        .security_requirement("Bearer")
        .tag("clippings")
}

#[debug_handler]
pub async fn get_clipping_item(
    State(pool): State<PgPool>, user: User, Path(path): Path<PathItem>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let item = crate::items::get_clipping_item(&mut conn, user.account_id, path.item_id).await?;
    Ok((StatusCode::OK, Json(item)))
}

pub fn get_clipping_item_docs(op: TransformOperation) -> TransformOperation {
    op.id("get_item_by_id")
        .description("Get clipping item")
        .response::<200, Json<ClippingItem>>()
        .tag("clippings")
}

#[debug_handler]
pub async fn change_clipping_item(
    State(pool): State<PgPool>, user: User, Path(path): Path<PathItem>,
    Json(body): Json<ChangeClippingItemInput>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let item =
        crate::items::change_clipping_item(&mut conn, user.account_id, path.item_id, body).await?;
    Ok((StatusCode::OK, Json(item)))
}

pub fn change_clipping_item_docs(op: TransformOperation) -> TransformOperation {
    op.id("change_item")
        .description("Change clipping item")
        .response::<200, Json<ClippingItem>>()
        .security_requirement("Bearer")
        .tag("clippings")
}

pub mod public {}

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
        .api_route(
            "/items",
            post_with(create_clipping_item, create_clipping_item_docs),
        )
        .api_route(
            "/items",
            get_with(list_clipping_items, list_clipping_items_docs),
        )
        .api_route(
            "/items/:item_id",
            get_with(get_clipping_item, get_clipping_item_docs),
        )
        .api_route(
            "/items/:item_id",
            patch_with(change_clipping_item, change_clipping_item_docs),
        )        
}
