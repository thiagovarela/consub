use accounts::User;

use axum::{
    debug_handler,
    extract::{DefaultBodyLimit, Multipart, Query, State},
    http::StatusCode,
    middleware, Json,
};

use aide::axum::routing::{get_with, post_with};
use aide::{
    axum::{ApiRouter, IntoApiResponse},
    transform::TransformOperation,
};

use serde::{Deserialize, Serialize};
use shared::{AppState, OpendalUploader};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::Error,
    images::{create_image, create_image_set, get_image, get_image_set, ImageQuery, ImageSet},
};

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ImageResponse {
    id: Uuid,
    image_set: Vec<ImageSet>,
}

#[debug_handler(state = AppState)]
pub async fn upload_image(
    State(pool): State<PgPool>, State(opendal): State<OpendalUploader>, user: User,
    mut multipart: Multipart,
) -> Result<impl IntoApiResponse, Error> {
    let id = rusty_ulid::generate_ulid_bytes();
    let id = uuid::Uuid::from_slice(&id).unwrap();
    let account_id = user.account_id;

    let mut conn = pool.acquire().await?;
    let image = create_image(&mut conn, account_id, id).await?;
    drop(conn);

    let account_id_ulid = rusty_ulid::Ulid::from(account_id.as_u128())
        .to_string()
        .to_lowercase();
    let image_id_ulid = rusty_ulid::Ulid::from(image.id.as_u128())
        .to_string()
        .to_lowercase();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };
        // TODO: change this later for a simple match on the content type
        let extension = std::path::Path::new(&filename)
            .extension()
            .unwrap()
            .to_string_lossy();

        let path = format!("{account_id_ulid}/{image_id_ulid}/{name}.{extension}");

        let data = field.bytes().await.unwrap();

        opendal.write(&path, data).await.unwrap();
        let mut conn = pool.acquire().await?;
        create_image_set(&mut conn, image.id, name, path).await?;
    }

    let mut conn = pool.acquire().await?;
    let image = get_image(&mut conn, account_id, id).await?;
    let image_set = get_image_set(&mut conn, image.id).await?;

    Ok((
        StatusCode::OK,
        Json(ImageResponse {
            id: image.id,
            image_set,
        }),
    ))
}

pub fn upload_image_docs(op: TransformOperation) -> TransformOperation {
    op.id("upload_image")
        .description("Upload image to media library")
        .response::<201, Json<ImageResponse>>()
        .tag("media")
}

#[debug_handler]
pub async fn list_images(
    State(pool): State<PgPool>, user: User, Query(query): Query<ImageQuery>,
) -> Result<impl IntoApiResponse, Error> {
    let account_id = user.account_id;
    let mut conn = pool.acquire().await?;
    let images = crate::images::list_images(&mut conn, account_id, query).await?;

    Ok((StatusCode::OK, Json(images)))
}

pub fn list_image_docs(op: TransformOperation) -> TransformOperation {
    op.id("list_images")
        .description("List images uploaded to the media library")
        .response::<200, Json<Vec<ImageResponse>>>()
        .tag("media")
}

pub fn routes(app_state: AppState) -> ApiRouter {
    let admin = ApiRouter::new()
        .api_route("/images", post_with(upload_image, upload_image_docs))
        .api_route("/images", get_with(list_images, list_image_docs))
        .layer(DefaultBodyLimit::max(50 * 1000 * 1000))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            accounts::authorization_layer,
        ));

    ApiRouter::new().nest("/admin", admin).with_state(app_state)
}
