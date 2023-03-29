use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shared::pagination::CursorPagination;
use sqlx::PgConnection;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Post {
    pub id: Uuid,
    #[serde(skip)]
    pub account_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub body_json: serde_json::Value,
    pub body_html: String,
    pub body_text: String,
    pub locale: String,
    pub is_featured: bool,
    pub short_description: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub category_id: Option<Uuid>,
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
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub category_id: Option<Uuid>,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,
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
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub category_id: Option<Uuid>,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,
}

#[derive(Debug, serde::Deserialize, Validate, JsonSchema)]
pub struct PostQuery {
    pub slug: Option<String>,
    pub locale: Option<String>,
    #[serde(default, rename = "category_id")]
    pub category_ids: Vec<Uuid>,
    pub category_slug: Option<String>,
    pub is_featured: Option<bool>,
    pub translation_of: Option<Uuid>,
    pub published_at: Option<NaiveDateTime>,

    #[serde(default, flatten)]
    pub pagination: CursorPagination,
}

pub async fn create_post(
    conn: &mut PgConnection, account_id: Uuid, author_id: Uuid, input: CreatePostInput,
) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        INSERT INTO blogs.posts (
            account_id, author_id, title, slug, body_json, body_html, body_text, locale, is_featured,
            short_description, meta_title, meta_description, meta_keywords,       
            category_id,
            translation_of, published_at                
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        RETURNING id, account_id, author_id, title, slug, body_json, body_html, body_text, locale, is_featured,
        short_description, meta_title, meta_description, meta_keywords,
        category_id, translation_of, published_at, updated_at
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
        input.meta_title,
        input.meta_description,
        input.meta_keywords,
        input.category_id,
        input.translation_of,
        input.published_at,
    )
    .fetch_one(conn)
    .await
}

pub async fn change_post(
    conn: &mut PgConnection, account_id: Uuid, post_id: Uuid, input: ChangePostInput,
) -> Result<Post, sqlx::Error> {
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
            meta_title = COALESCE($11, meta_title),
            meta_description = COALESCE($12, meta_description),
            meta_keywords = COALESCE($13, meta_keywords),
            category_id = COALESCE($14, category_id),            
            translation_of = COALESCE($15, translation_of),
            published_at = COALESCE($16, published_at)            
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
        input.meta_title,
        input.meta_description,
        input.meta_keywords,
        input.category_id,
        input.translation_of,
        input.published_at,
    )
    .fetch_one(conn)
    .await
}

pub async fn list_posts(
    conn: &mut PgConnection, account_id: Uuid, query: PostQuery,
) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        SELECT id, account_id, author_id, title, slug, body_json, body_html, body_text, locale, is_featured,
        short_description, meta_title, meta_description, meta_keywords,
        category_id,
        translation_of, published_at,
        updated_at FROM blogs.posts
        WHERE account_id = $1 
        AND ($2::text IS NULL OR locale = $2)
        AND (array_length($3::uuid[], 1) IS NULL OR category_id IN (SELECT UNNEST($3::uuid[])))
        "#,
        account_id,
        query.locale,
        &query.category_ids
    )
    .fetch_all(conn)
    .await
}

pub async fn get_post(
    conn: &mut PgConnection, account_id: Uuid, id: Uuid,
) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        SELECT id, account_id, author_id, title, slug, body_json, body_html, body_text, locale, is_featured,
        short_description, meta_title, meta_description, meta_keywords,
        category_id, 
        translation_of, published_at,
        updated_at
        FROM blogs.posts
        WHERE account_id = $1 AND id = $2
        "#,
        account_id,
        id
    )
    .fetch_one(conn)
    .await
}

pub async fn public_list_posts(
    conn: &mut PgConnection, account_id: Uuid, query: PostQuery,
) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        SELECT bp.id, bp.account_id, bp.author_id, bp.title, bp.slug, bp.body_json, bp.body_html, bp.body_text, bp.locale, bp.is_featured,
        bp.short_description, bp.meta_title, bp.meta_description, bp.meta_keywords,
        bp.category_id, bp.translation_of, bp.published_at, bp.updated_at 
        FROM blogs.posts bp
        LEFT OUTER JOIN blogs.categories bc ON bc.id = bp.category_id
        WHERE bp.account_id = $1 
        AND ($2::text IS NULL OR bp.slug = $2)
        AND ($3::text IS NULL OR bp.locale = $3)
        AND (array_length($4::uuid[], 1) IS NULL OR bp.category_id IN (SELECT UNNEST($4::uuid[])))
        AND ($5::text IS NULL OR bc.slug = $5)
        AND bp.published_at IS NOT NULL and bp.published_at <= NOW()
        AND ($6::text IS NULL OR bp.id > $6::uuid)
        ORDER BY bp.published_at DESC, bp.id DESC
        LIMIT $7
        "#,
        account_id,
        query.slug,
        query.locale,
        &query.category_ids,
        query.category_slug,
        query.pagination.after,
        query.pagination.take
    )
    .fetch_all(conn)
    .await
}
