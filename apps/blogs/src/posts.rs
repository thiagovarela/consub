use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;
use validator::Validate;

use crate::error::{conflict_error, Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare::typeshare]
pub struct Post {
    pub id: Uuid,
    #[typeshare(skip)]
    pub account_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub body: serde_json::Value,
    pub locale: String,
    pub is_featured: bool,
    pub keywords: Vec<String>,
    pub short_description: Option<String>,
    pub featured_image: Option<String>,
    pub featured_image_alt: Option<String>,
    pub featured_image_caption: Option<String>,
    pub category_id: Option<Uuid>,
    pub reading_time_minutes: Option<i32>,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, serde::Deserialize, Validate)]
#[typeshare::typeshare]
pub struct CreatePostInput {
    pub title: String,
    pub body: serde_json::Value,
    pub locale: String,
    pub is_featured: bool,
    pub short_description: Option<String>,
    pub featured_image: Option<String>,
    pub featured_image_alt: Option<String>,
    pub featured_image_caption: Option<String>,
    pub category_id: Option<Uuid>,
    pub reading_time_minutes: Option<i32>,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
}

#[derive(Debug, serde::Deserialize, Validate)]
#[typeshare::typeshare]
pub struct ChangePostInput {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub body: Option<serde_json::Value>,
    pub locale: Option<String>,
    pub is_featured: Option<bool>,
    pub short_description: Option<String>,
    pub featured_image: Option<String>,
    pub featured_image_alt: Option<String>,
    pub featured_image_caption: Option<String>,
    pub category_id: Option<Uuid>,
    pub reading_time_minutes: Option<i32>,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
}

#[derive(Debug, serde::Deserialize, Validate)]
#[typeshare::typeshare]
pub struct PostQuery {
    pub locale: Option<String>,
    #[serde(default, rename = "category_id")]
    pub category_ids: Vec<Uuid>,
    pub is_featured: Option<bool>,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,
}

pub async fn create_post(
    conn: &mut PgConnection, account_id: Uuid, author_id: Uuid, input: CreatePostInput,
) -> Result<Post, Error> {
    Ok(sqlx::query_as!(
        Post,
        r#"
        INSERT INTO blogs.posts (
            account_id, author_id, title, slug, body, locale, is_featured,
            short_description, featured_image, featured_image_alt, featured_image_caption,
            category_id, reading_time_minutes,
            translation_of, published_at,
            keywords, og_title, og_description, og_image            
        )
        VALUES ($1, $2, $3, $4, $5,
             $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
        RETURNING id, account_id, author_id, title, slug, body, locale, is_featured,
        short_description, featured_image, featured_image_alt, featured_image_caption,
        category_id, reading_time_minutes,
        translation_of, published_at,
        keywords, og_title, og_description, og_image, created_at, updated_at
        "#,
        account_id,
        author_id,
        &input.title,
        slug::slugify(&input.title),
        input.body,
        input.locale,
        input.is_featured,
        input.short_description,
        input.featured_image,
        input.featured_image_alt,
        input.featured_image_caption,
        input.category_id,
        input.reading_time_minutes,
        input.translation_of,
        input.published_at,
        &input.keywords,
        input.og_title,
        input.og_description,
        input.og_image
    )
    .fetch_one(conn)
    .await?)
}

pub async fn change_post(
    conn: &mut PgConnection, account_id: Uuid, post_id: Uuid, input: ChangePostInput,
) -> Result<Post, Error> {
    sqlx::query_as!(
        Post,
        r#"
        UPDATE blogs.posts
        SET title = COALESCE($3, title),
            slug = COALESCE($4, slug),
            locale = COALESCE($5, locale),
            body = COALESCE($6, body),
            is_featured = COALESCE($7, is_featured),
            short_description = COALESCE($8, short_description),
            featured_image = COALESCE($9, featured_image),
            featured_image_alt = COALESCE($10, featured_image_alt),
            featured_image_caption = COALESCE($11, featured_image_caption),
            category_id = COALESCE($12, category_id),
            reading_time_minutes = COALESCE($13, reading_time_minutes),            
            translation_of = COALESCE($14, translation_of),
            published_at = COALESCE($15, published_at),
            keywords = COALESCE($16, keywords),
            og_title = COALESCE($17, og_title),
            og_description = COALESCE($18, og_description),
            og_image = COALESCE($19, og_image)
        WHERE id = $1 
            AND account_id = $2
        RETURNING *
        "#,
        post_id,
        account_id,
        input.title,
        input.slug.map_or(None, |s| Some(slug::slugify(&s))),
        input.locale,
        input.body,
        input.is_featured,
        input.short_description,
        input.featured_image,
        input.featured_image_alt,
        input.featured_image_caption,
        input.category_id,
        input.reading_time_minutes,
        input.translation_of,
        input.published_at,
        &input.keywords,
        input.og_title,
        input.og_description,
        input.og_image
    )
    .fetch_one(conn)
    .await
    .map_err(conflict_error)
}

pub async fn list_posts(
    conn: &mut PgConnection, account_id: Uuid, query: PostQuery,
) -> Result<Vec<Post>, Error> {
    Ok(sqlx::query_as!(
        Post,
        r#"
        SELECT id, account_id, author_id, title, slug, body, locale, is_featured,
        short_description, featured_image, featured_image_alt, featured_image_caption,
        category_id, reading_time_minutes,
        translation_of, published_at,
        keywords, og_title, og_description, og_image, created_at, updated_at FROM blogs.posts
        WHERE account_id = $1 
        AND ($2::text IS NULL OR locale = $2)
        AND (array_length($3::uuid[], 1) IS NULL OR category_id IN (SELECT UNNEST($3::uuid[])))
        "#,
        account_id,
        query.locale,
        &query.category_ids
    )
    .fetch_all(conn)
    .await?)
}

pub async fn get_post(conn: &mut PgConnection, account_id: Uuid, id: Uuid) -> Result<Post, Error> {
    Ok(sqlx::query_as!(
        Post,
        r#"
        SELECT id, account_id, author_id, title, slug, body, locale, is_featured,
        short_description, featured_image, featured_image_alt, featured_image_caption,
        category_id, reading_time_minutes,
        translation_of, published_at,
        keywords, og_title, og_description, og_image, created_at, updated_at
        FROM blogs.posts
        WHERE account_id = $1 AND id = $2
        "#,
        account_id,
        id
    )
    .fetch_one(conn)
    .await?)
}
