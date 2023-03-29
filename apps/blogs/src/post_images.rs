use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PostImage {
    pub id: Uuid,
    pub post_id: Uuid,
    pub image_type: String,
    pub media_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreatePostImageInput {
    pub image_type: String,
    pub media_id: Uuid,
}

pub async fn create_post_image(
    conn: &mut PgConnection, post_id: Uuid, input: CreatePostImageInput,
) -> Result<PostImage, sqlx::Error> {
    sqlx::query_as!(
        PostImage,
        r#"
        INSERT INTO blogs.post_images (post_id, image_type, media_id)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        post_id,
        input.image_type,
        input.media_id,
    )
    .fetch_one(conn)
    .await
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChangePostImageInput {
    pub image_type: Option<String>,
}

pub async fn change_post_image(
    conn: &mut PgConnection, post_image_id: Uuid, account_id: Uuid, input: ChangePostImageInput,
) -> Result<PostImage, sqlx::Error> {
    sqlx::query_as!(
        PostImage,
        r#"
        UPDATE blogs.post_images
        SET image_type = COALESCE($3, image_type)
        FROM blogs.posts 
        WHERE blogs.post_images.post_id = $1
        AND blogs.posts.id = blogs.post_images.post_id
        AND blogs.posts.account_id = $2
        RETURNING blogs.post_images.*
        "#,
        post_image_id,
        account_id,
        input.image_type,
    )
    .fetch_one(conn)
    .await
}

pub async fn delete_post_image(
    conn: &mut PgConnection, post_image_id: Uuid, account_id: Uuid,
) -> Result<PostImage, sqlx::Error> {
    sqlx::query_as!(
        PostImage,
        r#"
        DELETE FROM blogs.post_images
        USING blogs.posts
        WHERE blogs.post_images.id = $1
        AND blogs.post_images.post_id = blogs.posts.id        
        AND blogs.posts.account_id = $2
        RETURNING blogs.post_images.*
        "#,
        post_image_id,
        account_id
    )
    .fetch_one(conn)
    .await
}

pub async fn list_post_images(
    conn: &mut PgConnection, account_id: Uuid, post_id: Uuid,
) -> Result<Vec<PostImage>, sqlx::Error> {
    sqlx::query_as!(
        PostImage,
        r#"
        SELECT pi.* FROM blogs.post_images pi
        INNER JOIN blogs.posts p ON p.id = pi.post_id
        WHERE p.account_id = $1 
        AND p.id = $2          
        "#,
        account_id,
        post_id
    )
    .fetch_all(conn)
    .await
}

pub async fn list_post_images_by_post_ids(
    conn: &mut PgConnection, account_id: Uuid, post_ids: Vec<Uuid>,
) -> Result<Vec<PostImage>, sqlx::Error> {
    sqlx::query_as!(
        PostImage,
        r#"
        SELECT pi.* FROM blogs.post_images pi
        INNER JOIN blogs.posts p ON p.id = pi.post_id
        WHERE p.account_id = $1 
        AND p.id = ANY($2)          
        "#,
        account_id,
        &post_ids
    )
    .fetch_all(conn)
    .await
}
