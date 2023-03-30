use accounts::Account;
use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
};
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    Json,
};
use shared::{AppError, AppState};
use sqlx::PgPool;

use crate::router::{ImageResponse, PathImage};

pub async fn get_image(
    State(pool): State<PgPool>, account: Account, Path(path): Path<PathImage>,
) -> Result<impl IntoApiResponse, AppError> {
    let mut conn = pool.acquire().await?;

    let image = crate::images::get_image(&mut conn, account.id, path.image_id).await?;
    let image_set = crate::images::get_image_set(&mut conn, image.id).await?;

    Ok((StatusCode::OK, Json(ImageResponse { image, image_set })))
}

pub fn get_image_docs(op: TransformOperation) -> TransformOperation {
    op.id("get_image")
        .description("Get the image in the media library")
        .response::<200, Json<ImageResponse>>()
        .tag("media")
}

pub fn public_routes(app_state: AppState) -> aide::axum::ApiRouter {
    ApiRouter::new()
        .api_route("/images/:image_id", get_with(get_image, get_image_docs))
        .with_state(app_state)
}
