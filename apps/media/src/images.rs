use chrono::NaiveDateTime;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use shared::pagination::CursorPagination;
use sqlx::PgConnection;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Image {
    pub id: Uuid,
    #[serde(skip)]
    pub account_id: Uuid,
    pub alt: Option<String>,
    pub caption: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ImageSet {
    pub id: Uuid,
    #[serde(skip)]
    pub image_id: Uuid,
    pub size: String,
    pub path: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ImageResponse {
    id: Uuid,
    image_set: Vec<ImageSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CreateImageInput {
    pub id: Uuid,
    pub alt: Option<String>,
    pub caption: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ChangeImageInput {
    pub alt: Option<String>,
    pub caption: Option<String>,
}

#[derive(Debug, schemars::JsonSchema, Deserialize)]
pub struct ImageQuery {
    #[serde(default, rename = "image_id")]
    pub ids: Vec<Uuid>,
    pub size: Option<String>,

    #[serde(default, flatten)]
    pub pagination: CursorPagination,
}

pub async fn create_image(
    conn: &mut PgConnection, account_id: Uuid, id: Uuid,
) -> Result<Image, sqlx::Error> {
    sqlx::query_as!(
        Image,
        r#"
        INSERT INTO media.images (id, account_id)
        VALUES ($1, $2)
        RETURNING *
        "#,
        id,
        account_id,
    )
    .fetch_one(conn)
    .await
}

pub async fn get_image(
    conn: &mut PgConnection, account_id: Uuid, id: Uuid,
) -> Result<Image, sqlx::Error> {
    sqlx::query_as!(
        Image,
        r#"
        SELECT * FROM media.images
        WHERE account_id = $1 AND id = $2
        "#,
        account_id,
        id,
    )
    .fetch_one(conn)
    .await
}

pub async fn change_image(
    conn: &mut PgConnection, account_id: Uuid, id: Uuid, input: ChangeImageInput,
) -> Result<Image, sqlx::Error> {
    sqlx::query_as!(
        Image,
        r#"
        UPDATE media.images
        SET alt = COALESCE($3, alt),
            caption = COALESCE($4, caption)            
        WHERE account_id = $1 AND id = $2
        RETURNING *
        "#,
        account_id,
        id,
        input.alt,
        input.caption,
    )
    .fetch_one(conn)
    .await
}

pub async fn create_image_set(
    conn: &mut PgConnection, image_id: Uuid, size: String, path: String,
) -> Result<ImageSet, sqlx::Error> {
    sqlx::query_as!(
        ImageSet,
        r#"
        INSERT INTO media.images_set (image_id, size, path)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        image_id,
        size,
        path,
    )
    .fetch_one(conn)
    .await
}

pub async fn get_image_set(
    conn: &mut PgConnection, image_id: Uuid,
) -> Result<Vec<ImageSet>, sqlx::Error> {
    sqlx::query_as!(
        ImageSet,
        r#"
        SELECT * FROM media.images_set
        WHERE image_id = $1
        "#,
        image_id
    )
    .fetch_all(conn)
    .await
}

pub async fn list_images(
    conn: &mut PgConnection, account_id: Uuid, query: ImageQuery,
) -> Result<Vec<ImageResponse>, sqlx::Error> {
    let images_set = sqlx::query_as!(
        ImageSet,
        r#"
        WITH images as (
            SELECT id FROM media.images
            WHERE account_id = $1            
            AND ($3::text IS NULL OR id > $3::uuid)
            AND (array_length($5::uuid[], 1) IS NULL OR id IN (SELECT UNNEST($5::uuid[])))  
            ORDER BY id LIMIT $4
        ) SELECT * from media.images_set
            WHERE image_id in (SELECT id FROM images)
            AND ($2::text IS NULL OR size = $2::text)       
            ORDER BY image_id DESC
        "#,
        account_id,
        query.size,
        query.pagination.after,
        query.pagination.take,
        &query.ids
    )
    .fetch_all(conn)
    .await?;

    let mut data_grouped = Vec::new();
    for (key, group) in &images_set.into_iter().group_by(|elt| elt.image_id) {
        data_grouped.push(ImageResponse {
            id: key,
            image_set: group.collect(),
        });
    }
    Ok(data_grouped)
}
