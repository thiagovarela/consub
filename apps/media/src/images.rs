use chrono::NaiveDateTime;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use shared::pagination::CursorPagination;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: Uuid,
    pub account_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ImageSet {
    pub id: Uuid,
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

#[derive(Debug, schemars::JsonSchema, Deserialize)]
pub struct ImageQuery {
    pub size: Option<String>,

    #[serde(default, flatten)]
    pub pagination: CursorPagination,
}

pub async fn create_image(
    conn: &mut PgConnection, account_id: Uuid, id: Uuid,
) -> Result<Image, Error> {
    Ok(sqlx::query_as!(
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
    .await?)
}

pub async fn create_image_set(
    conn: &mut PgConnection, image_id: Uuid, size: String, path: String,
) -> Result<ImageSet, Error> {
    Ok(sqlx::query_as!(
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
    .await?)
}

pub async fn get_image(
    conn: &mut PgConnection, account_id: Uuid, id: Uuid,
) -> Result<Image, Error> {
    Ok(sqlx::query_as!(
        Image,
        r#"
        SELECT * FROM media.images
        WHERE account_id = $1 AND id = $2
        "#,
        account_id,
        id,
    )
    .fetch_one(conn)
    .await?)
}

pub async fn get_image_set(
    conn: &mut PgConnection, image_id: Uuid,
) -> Result<Vec<ImageSet>, Error> {
    Ok(sqlx::query_as!(
        ImageSet,
        r#"
        SELECT * FROM media.images_set
        WHERE image_id = $1
        "#,
        image_id
    )
    .fetch_all(conn)
    .await?)
}

pub async fn list_images(
    conn: &mut PgConnection, account_id: Uuid, query: ImageQuery,
) -> Result<Vec<ImageResponse>, Error> {
    let images_set = sqlx::query_as!(
        ImageSet,
        r#"
        with images as (
            SELECT id FROM media.images
            WHERE account_id = $1            
            AND ($3::text IS NULL OR id > $3::uuid)
            ORDER BY id LIMIT $4
        ) select * from media.images_set
            WHERE image_id in (select id from images)
            AND ($2::text IS NULL OR size = $2::text)        
        "#,
        account_id,
        query.size,
        query.pagination.after,
        query.pagination.take
    )
    .fetch_all(conn)
    .await?;

    dbg!(&images_set);

    let mut data_grouped = Vec::new();
    for (key, group) in &images_set.into_iter().group_by(|elt| elt.image_id) {
        data_grouped.push(ImageResponse {
            id: key,
            image_set: group.collect(),
        });
    }
    Ok(data_grouped)
}
