use accounts::User;

use axum::{
    debug_handler,
    extract::{DefaultBodyLimit, Multipart, Path, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::Query;

use aide::axum::{routing::{get_with, post_with}, ApiRouter};
use aide::{
    axum::{IntoApiResponse},
    transform::TransformOperation,
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use shared::{AppError, AppState, OpendalUploader};
use sqlx::PgPool;
use uuid::Uuid;

use crate::images::{
    create_image, create_image_set, get_image, get_image_set, ChangeImageInput, Image, ImageQuery,
    ImageSet,
};

static IMAGES_CDN_PATH: Lazy<String> = Lazy::new(|| {
    std::env::var("CDN_PATH")
        .expect("CDN_PATH environment variable is not set. Please set it to the CDN path.")
});

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ImageResponse {
    #[serde(flatten)]
    pub image: Image,
    pub image_set: Vec<ImageSet>,
}

#[debug_handler(state = AppState)]
pub async fn upload_image(
    State(pool): State<PgPool>, State(opendal): State<OpendalUploader>, user: User,
    mut multipart: Multipart,
) -> Result<impl IntoApiResponse, AppError> {
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

        let extension = match field.content_type().unwrap() {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            "image/gif" => "gif",
            "image/webp" => "webp",
            _ => continue,
        };

        let cdn_path = &*IMAGES_CDN_PATH;
        let file_path = format!("{account_id_ulid}/{image_id_ulid}/{name}.{extension}");
        let path = format!("{cdn_path}/{file_path}");

        let data = field.bytes().await.unwrap();

        opendal.write(&file_path, data).await.unwrap();
        let mut conn = pool.acquire().await?;
        create_image_set(&mut conn, image.id, name, path).await?;
    }

    let mut conn = pool.acquire().await?;
    let image = get_image(&mut conn, account_id, id).await?;
    let image_set = get_image_set(&mut conn, image.id).await?;

    Ok((StatusCode::OK, Json(ImageResponse { image, image_set })))
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
) -> Result<impl IntoApiResponse, AppError> {
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

#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct PathImage {
    pub image_id: Uuid,
}

#[debug_handler]
pub async fn change_image(
    State(pool): State<PgPool>, user: User, Path(path): Path<PathImage>,
    Json(body): Json<ChangeImageInput>,
) -> Result<impl IntoApiResponse, AppError> {
    let account_id = user.account_id;
    let mut conn = pool.acquire().await?;
    let image = crate::images::change_image(&mut conn, account_id, path.image_id, body).await?;

    Ok((StatusCode::OK, Json(image)))
}

pub fn change_image_docs(op: TransformOperation) -> TransformOperation {
    op.id("change_image")
        .description("Change the image in the media library")
        .response::<200, Json<Image>>()
        .tag("media")
}

pub fn routes() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/images", post_with(upload_image, upload_image_docs))
        .api_route("/images", get_with(list_images, list_image_docs))
        .layer(DefaultBodyLimit::max(50 * 1000 * 1000))        
}
