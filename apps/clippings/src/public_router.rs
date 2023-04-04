use accounts::Account;
use aide::{
    axum::{routing::get_with, IntoApiResponse, ApiRouter},
    transform::TransformOperation,
};
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    Json,
};
use axum_extra::extract::Query;
use shared::{AppError, AppState};
use sqlx::PgPool;

use crate::{
    categories::{Category, CategoryQuery},
    items::{ClippingItem, ClippingItemQuery},
    router::{PathCategory, PathItem},
};

pub async fn list_clipping_items(
    State(pool): State<PgPool>, account: Account, Query(query): Query<ClippingItemQuery>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let items = crate::items::public_list_clipping_items(&mut conn, account.id, query).await?;
    Ok((StatusCode::OK, Json(items)))
}

pub fn list_clipping_items_docs(op: TransformOperation) -> TransformOperation {
    op.id("list_items")
        .description("List clipping items")
        .response::<200, Json<Vec<ClippingItem>>>()
        .tag("clippings")
}

pub async fn list_categories(
    State(pool): State<PgPool>, account: Account, Query(query): Query<CategoryQuery>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let categories = crate::categories::list_categories(&mut conn, account.id, query).await?;
    Ok((StatusCode::OK, Json(categories)))
}

pub fn list_categories_docs(op: TransformOperation) -> TransformOperation {
    op.id("list_categories")
        .description("List Categories")
        .response::<200, Json<Vec<Category>>>()
        .tag("clippings")
}

pub async fn get_category(
    State(pool): State<PgPool>, account: Account, Path(path): Path<PathCategory>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let category =
        crate::categories::get_category_by_id(&mut conn, path.category_id, account.id).await?;
    Ok((StatusCode::OK, Json(category)))
}

pub fn get_category_docs(op: TransformOperation) -> TransformOperation {
    op.id("get_category_by_id")
        .description("Get category")
        .response::<200, Json<Category>>()
        .tag("clippings")
}

pub async fn get_clipping_item(
    State(pool): State<PgPool>, account: Account, Path(path): Path<PathItem>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    let item = crate::items::get_clipping_item(&mut conn, account.id, path.item_id).await?;
    Ok((StatusCode::OK, Json(item)))
}

pub fn get_clipping_item_docs(op: TransformOperation) -> TransformOperation {
    op.id("get_item_by_id")
        .description("Get clipping item")
        .response::<200, Json<ClippingItem>>()
        .tag("clippings")
}

pub fn public_routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/categories",
            get_with(list_categories, list_categories_docs),
        )
        .api_route(
            "/categories/:category_id",
            get_with(get_category, get_category_docs),
        )
        .api_route(
            "/items",
            get_with(list_clipping_items, list_clipping_items_docs),
        )
        .api_route(
            "/items/:item_id",
            get_with(get_clipping_item, get_clipping_item_docs),
        )        
}
