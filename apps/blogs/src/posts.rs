use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;
use validator::Validate;

use crate::error::{conflict_error, Error};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Post {
    pub id: Uuid,
    pub account_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub body_json: serde_json::Value,
    pub body_html: String,
    pub body_text: String,
    pub locale: String,
    pub is_featured: bool,
    pub keywords: Vec<String>,
    pub short_description: Option<String>,
    pub category_id: Option<Uuid>,
    pub reading_time_minutes: Option<i32>,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, serde::Deserialize, Validate, JsonSchema)]
pub struct CreatePostInput {
    pub title: String,
    pub body_json: serde_json::Value,
    pub body_html: String,
    pub body_text: String,
    pub locale: String,
    pub is_featured: bool,
    pub short_description: Option<String>,
    pub category_id: Option<Uuid>,
    pub reading_time_minutes: Option<i32>,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,
    #[serde(default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, serde::Deserialize, Validate, JsonSchema)]
pub struct ChangePostInput {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub body_json: Option<serde_json::Value>,
    pub body_html: Option<String>,
    pub body_text: Option<String>,
    pub locale: Option<String>,
    pub is_featured: Option<bool>,
    pub short_description: Option<String>,
    pub category_id: Option<Uuid>,
    pub reading_time_minutes: Option<i32>,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,
    #[serde(default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, serde::Deserialize, Validate, JsonSchema)]
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
            account_id, author_id, title, slug, body_json, body_html, body_text, locale, is_featured,
            short_description,
            category_id, reading_time_minutes,
            translation_of, published_at,
            keywords
        )
        VALUES ($1, $2, $3, $4, $5,
             $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING id, account_id, author_id, title, slug, body_json, body_html, body_text, locale, is_featured,
        short_description,
        category_id, reading_time_minutes,
        translation_of, published_at,
        keywords, updated_at
        "#,
        account_id,
        author_id,
        &input.title,
        slug::slugify(&input.title),
        input.body_json,
        input.body_html,
        input.body_text,
        input.locale,
        input.is_featured,
        input.short_description,
        input.category_id,
        input.reading_time_minutes,
        input.translation_of,
        input.published_at,
        &input.keywords,
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
            body_json = COALESCE($6, body_json),
            body_html = COALESCE($7, body_html),
            body_text = COALESCE($8, body_text),
            is_featured = COALESCE($9, is_featured),
            short_description = COALESCE($10, short_description),
            category_id = COALESCE($11, category_id),
            reading_time_minutes = COALESCE($12, reading_time_minutes),            
            translation_of = COALESCE($13, translation_of),
            published_at = COALESCE($14, published_at),
            keywords = COALESCE($15, keywords)
        WHERE id = $1 
            AND account_id = $2
        RETURNING *
        "#,
        post_id,
        account_id,
        input.title,
        input.slug.map_or(None, |s| Some(slug::slugify(&s))),
        input.locale,
        input.body_json,
        input.body_html,
        input.body_text,
        input.is_featured,
        input.short_description,
        input.category_id,
        input.reading_time_minutes,
        input.translation_of,
        input.published_at,
        &input.keywords,
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
        SELECT id, account_id, author_id, title, slug, body_json, body_html, body_text, locale, is_featured,
        short_description,
        category_id, reading_time_minutes,
        translation_of, published_at,
        keywords, updated_at FROM blogs.posts
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
        SELECT id, account_id, author_id, title, slug, body_json, body_html, body_text, locale, is_featured,
        short_description,
        category_id, reading_time_minutes,
        translation_of, published_at,
        keywords, updated_at
        FROM blogs.posts
        WHERE account_id = $1 AND id = $2
        "#,
        account_id,
        id
    )
    .fetch_one(conn)
    .await?)
}
