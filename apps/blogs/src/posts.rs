use chrono::NaiveDateTime;
use estimated_read_time::{Options, ReadTime};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{error::Error, router::CreatePostInput};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub account_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub locale: String,
    pub category_id: Option<Uuid>,
    pub featured: bool,
    pub estimated_reading_time: i32,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn create_post(
    conn: &mut PgConnection,
    account_id: Uuid,
    author_id: Uuid,
    input: CreatePostInput,
) -> Result<Post, Error> {
    let time_taken: ReadTime = estimated_read_time::text(&input.body, &Options::new());
    Ok(sqlx::query_as!(
        Post,
        r#"
        INSERT INTO blogs.posts (account_id, author_id, title, slug, body, locale, category_id, featured, estimated_reading_time, translation_of, published_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING *
        "#,
        account_id,
        author_id,
        &input.title,
        slug::slugify(&input.title),
        input.body,
        input.locale,
        input.category_id,
        input.featured,
        time_taken.seconds() as i32,
        input.translation_of,
        input.published_at
    )
    .fetch_one(conn)
    .await?)
}

pub async fn list_posts(
    conn: &mut PgConnection,
    account_id: Uuid,
    locale: Option<String>,
) -> Result<Vec<Post>, Error> {
    Ok(sqlx::query_as!(
        Post,
        r#"
        SELECT * FROM blogs.posts
        WHERE account_id = $1 AND locale = coalesce($2, locale)
        "#,
        account_id,
        locale
    )
    .fetch_all(conn)
    .await?)
}

pub async fn get_post(conn: &mut PgConnection, account_id: Uuid, id: Uuid) -> Result<Post, Error> {
    Ok(sqlx::query_as!(
        Post,
        r#"
        SELECT * FROM blogs.posts
        WHERE account_id = $1 AND id = $2
        "#,
        account_id,
        id
    )
    .fetch_one(conn)
    .await?)
}
