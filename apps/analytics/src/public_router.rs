use accounts::Account;
use aide::{
    axum::{routing::post_with, IntoApiResponse, ApiRouter},
    transform::TransformOperation,
};
use axum::http::StatusCode;
use axum::{extract::State, Json};
use shared::{AppError, AppState};
use sqlx::PgPool;

use crate::page_views::PageViewInput;

pub async fn create_page_view(
    State(pool): State<PgPool>, account: Account, Json(input): Json<PageViewInput>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;
    crate::page_views::create_page_view(&mut conn, account.id, input).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn create_page_view_docs(op: TransformOperation) -> TransformOperation {
    op.id("create_page_view")
        .description("Add a page view")
        .response::<204, ()>()
        .tag("analytics")
}

pub fn public_routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/page_view",
            post_with(create_page_view, create_page_view_docs),
        )        
}
