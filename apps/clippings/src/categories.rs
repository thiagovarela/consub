use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

use shared::pagination::CursorPagination;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Category {
    #[schemars(with = "String")]
    pub id: Uuid,
    #[serde(skip)]
    pub account_id: Uuid,
    pub name: String,
    pub slug: String,
    pub locale: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCategoryInput {
    pub name: String,
    pub locale: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChangeCategoryInput {
    pub name: Option<String>,
    pub locale: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, JsonSchema, Deserialize)]
pub struct CategoryQuery {
    #[serde(rename = "name_starts_with")]
    pub name: Option<String>,
    pub locale: Option<String>,
    pub translation_of: Option<Uuid>,

    #[serde(default, flatten)]
    pub pagination: CursorPagination,
}

pub async fn create_category(
    conn: &mut PgConnection, account_id: Uuid, input: CreateCategoryInput,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        r#"
        INSERT INTO clippings.categories (account_id, name, slug, locale)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        account_id,
        &input.name,
        slug::slugify(&input.name),
        input.locale
    )
    .fetch_one(conn)
    .await
}

pub async fn change_category(
    conn: &mut PgConnection, category_id: Uuid, account_id: Uuid, input: ChangeCategoryInput,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        r#"
        UPDATE clippings.categories
        SET name = COALESCE($3, name),
            slug = COALESCE($4, slug),
            locale = COALESCE($5, locale)
        WHERE id = $1 
            AND account_id = $2
        RETURNING *
        "#,
        category_id,
        account_id,
        input.name,
        input.slug.map_or(None, |s| Some(slug::slugify(&s))),
        input.locale
    )
    .fetch_one(conn)
    .await
}

pub async fn delete_category(
    conn: &mut PgConnection, category_id: Uuid, account_id: Uuid,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        r#"
        DELETE FROM clippings.categories
        WHERE id = $1 
        AND account_id = $2
        RETURNING *
        "#,
        category_id,
        account_id
    )
    .fetch_one(conn)
    .await
}

pub async fn get_category_by_id(
    conn: &mut PgConnection, category_id: Uuid, account_id: Uuid,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        r#"
        SELECT * FROM clippings.categories
        WHERE id = $1 
        AND account_id = $2
        "#,
        category_id,
        account_id
    )
    .fetch_one(conn)
    .await
}

pub async fn list_categories(
    conn: &mut PgConnection, account_id: Uuid, query: CategoryQuery,
) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as!(
        Category,
        r#"
        SELECT * FROM clippings.categories
        WHERE account_id = $1
        AND ($2::text IS NULL OR name ~* $2)
        AND ($3::text IS NULL OR locale = $3)        
        AND ($5::text IS NULL OR id > $5::uuid)
        ORDER BY id LIMIT $4
        "#,
        account_id,
        query.name,
        query.locale,
        query.pagination.take,
        query.pagination.after
    )
    .fetch_all(conn)
    .await
}
