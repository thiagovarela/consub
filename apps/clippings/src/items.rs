use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;
use validator::Validate;

use crate::error::{conflict_error, Error};
use shared::pagination::CursorPagination;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClippingItem {
    pub id: Uuid,
    #[serde(skip)]
    pub account_id: Uuid,
    pub created_by_id: Uuid,
    pub title: String,
    pub slug: String,
    pub body_json: serde_json::Value,
    pub body_html: String,
    pub body_text: String,
    pub source: String,
    pub source_url: String,
    pub source_published_at: NaiveDateTime,
    pub locale: String,
    pub is_featured: bool,
    pub category_id: Option<Uuid>,
    pub short_description: Option<String>,
    pub tags: Vec<String>,
    pub reading_time_minutes: Option<i32>,
    pub published_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, serde::Deserialize, Validate, JsonSchema)]
pub struct CreateClippingItemInput {
    pub title: String,
    pub body_json: serde_json::Value,
    pub body_html: String,
    pub body_text: String,
    pub locale: String,
    pub source: String,
    pub source_url: String,
    pub source_published_at: NaiveDateTime,
    pub category_id: Option<Uuid>,
    pub short_description: Option<String>,
    pub is_featured: Option<bool>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub reading_time_minutes: Option<i32>,
    pub published_at: Option<NaiveDateTime>,
}

#[derive(Debug, serde::Deserialize, Validate, JsonSchema)]
pub struct ChangeClippingItemInput {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub body_json: Option<serde_json::Value>,
    pub body_html: Option<String>,
    pub body_text: Option<String>,
    pub locale: Option<String>,
    pub source: Option<String>,
    pub source_url: Option<String>,
    pub source_published_at: Option<NaiveDateTime>,
    pub is_featured: Option<bool>,
    pub short_description: Option<String>,
    pub category_id: Option<Uuid>,
    pub reading_time_minutes: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "::serde_with::rust::double_option::deserialize"
    )]
    pub published_at: Option<Option<NaiveDateTime>>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, serde::Deserialize, Validate, JsonSchema)]
pub struct ClippingItemQuery {
    pub locale: Option<String>,
    #[serde(default, rename = "category_id")]
    pub category_ids: Vec<Uuid>,
    #[serde(default, rename = "tag")]
    pub tags: Vec<Uuid>,
    pub is_featured: Option<bool>,
    pub published_at: Option<NaiveDateTime>,
    pub slug: Option<String>,

    #[serde(default, flatten)]
    pub pagination: CursorPagination,
}

pub async fn create_clipping_item(
    conn: &mut PgConnection, account_id: Uuid, user_id: Uuid, input: CreateClippingItemInput,
) -> Result<ClippingItem, Error> {
    Ok(sqlx::query_as!(
        ClippingItem,
        r#"
        INSERT INTO clippings.items (
            account_id, created_by_id, title, slug, body_json, body_html, body_text, locale,
            short_description, source, source_url, source_published_at, is_featured,            
            category_id, reading_time_minutes, published_at, tags          
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        RETURNING id, account_id, created_by_id, title, slug, body_json, body_html, body_text, locale,
        short_description, source, source_url, source_published_at, is_featured,            
        category_id, reading_time_minutes, published_at, tags, updated_at
        "#,
        account_id,
        user_id,
        &input.title,
        slug::slugify(&input.title),
        input.body_json,
        input.body_html,
        input.body_text,
        input.locale,
        input.short_description,
        input.source,
        input.source_url,
        input.source_published_at,
        input.is_featured,
        input.category_id,
        input.reading_time_minutes,
        input.published_at,
        &input.tags,
    )
    .fetch_one(conn)
    .await?)
}

pub async fn change_clipping_item(
    conn: &mut PgConnection, account_id: Uuid, clipping_item_id: Uuid,
    input: ChangeClippingItemInput,
) -> Result<ClippingItem, Error> {
    let item = get_clipping_item(conn, account_id, clipping_item_id).await?;

    sqlx::query_as!(
        ClippingItem,
        r#"
        UPDATE clippings.items
        SET title = COALESCE($3, title),
            slug = COALESCE($4, slug),
            locale = COALESCE($5, locale),
            body_json = COALESCE($6, body_json),
            is_featured = COALESCE($7, is_featured),
            short_description = COALESCE($8, short_description),
            source = COALESCE($9, source),
            source_url = COALESCE($10, source_url),
            source_published_at = COALESCE($11, source_published_at),            
            category_id = COALESCE($12, category_id),
            reading_time_minutes = COALESCE($13, reading_time_minutes),                        
            published_at = $14,
            tags = COALESCE($15, tags),
            body_html = COALESCE($16, body_html),
            body_text = COALESCE($17, body_text)
        WHERE id = $1
          AND account_id = $2
        RETURNING *
        "#,
        clipping_item_id,
        account_id,
        input.title,
        input.slug.map_or(None, |s| Some(slug::slugify(&s))),
        input.locale,
        input.body_json,
        input.is_featured,
        input.short_description,
        input.source,
        input.source_url,
        input.source_published_at,
        input.category_id,
        input.reading_time_minutes,
        input.published_at.unwrap_or_else(|| item.published_at),
        &input.tags,
        input.body_html,
        input.body_text
    )
    .fetch_one(conn)
    .await
    .map_err(conflict_error)
}

pub async fn list_clipping_items(
    conn: &mut PgConnection, account_id: Uuid, query: ClippingItemQuery,
) -> Result<Vec<ClippingItem>, Error> {
    Ok(sqlx::query_as!(
        ClippingItem,
        r#"SELECT *
        FROM clippings.items
        WHERE account_id = $1 
        AND ($2::text IS NULL OR locale = $2)
        AND (array_length($3::uuid[], 1) IS NULL OR category_id IN (SELECT UNNEST($3::uuid[])))  
        AND ($4::text IS NULL OR id < $4::uuid)
        ORDER BY id LIMIT $5
        "#,
        account_id,
        query.locale,
        &query.category_ids,
        query.pagination.after,
        query.pagination.take
    )
    .fetch_all(conn)
    .await?)
}

pub async fn get_clipping_item(
    conn: &mut PgConnection, account_id: Uuid, id: Uuid,
) -> Result<ClippingItem, Error> {
    Ok(sqlx::query_as!(
        ClippingItem,
        r#"SELECT *
        FROM clippings.items
        WHERE account_id = $1 AND id = $2
        "#,
        account_id,
        id
    )
    .fetch_one(conn)
    .await?)
}

pub async fn public_list_clipping_items(
    conn: &mut PgConnection, account_id: Uuid, query: ClippingItemQuery,
) -> Result<Vec<ClippingItem>, Error> {
    Ok(sqlx::query_as!(
        ClippingItem,
        r#"SELECT *
        FROM clippings.items
        WHERE account_id = $1 AND published_at IS NOT NULL
        AND ($2::text IS NULL OR locale = $2)
        AND (array_length($3::uuid[], 1) IS NULL OR category_id IN (SELECT UNNEST($3::uuid[])))        
        AND ($4::text IS NULL OR slug = $4)
        AND ($5::text IS NULL OR id < $5::uuid)
        ORDER BY published_at, id LIMIT $6
        "#,
        account_id,
        query.locale,
        &query.category_ids,
        query.slug,
        query.pagination.after,
        query.pagination.take
    )
    .fetch_all(conn)
    .await?)
}
