use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::error::{conflict_error, Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare::typeshare]
pub struct Category {
    pub id: Uuid,
    #[typeshare(skip)]
    pub account_id: Uuid,
    pub name: String,
    pub slug: String,
    pub locale: String,
    pub translation_of: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
#[typeshare::typeshare]
pub struct CreateCategoryInput {
    pub name: String,
    pub locale: String,
    pub translation_of: Option<Uuid>,
}

pub async fn create_category(
    conn: &mut PgConnection, account_id: Uuid, input: CreateCategoryInput,
) -> Result<Category, Error> {
    sqlx::query_as!(
        Category,
        r#"
        INSERT INTO blogs.categories (account_id, name, slug, locale, translation_of)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        account_id,
        &input.name,
        slug::slugify(&input.name),
        input.locale,
        input.translation_of
    )
    .fetch_one(conn)
    .await
    .map_err(conflict_error)
}

#[derive(Debug, Deserialize)]
#[typeshare::typeshare]
pub struct ChangeCategoryInput {
    pub name: Option<String>,
    pub locale: Option<String>,
    pub slug: Option<String>,
    pub translation_of: Option<Uuid>,
}

pub async fn change_category(
    conn: &mut PgConnection, category_id: Uuid, account_id: Uuid, input: ChangeCategoryInput,
) -> Result<Category, Error> {
    sqlx::query_as!(
        Category,
        r#"
        UPDATE blogs.categories
        SET name = COALESCE($3, name),
            slug = COALESCE($4, slug),
            locale = COALESCE($5, locale),
            translation_of = COALESCE($6, translation_of)
        WHERE id = $1 
            AND account_id = $2
        RETURNING *
        "#,
        category_id,
        account_id,
        input.name,
        input.slug.map_or(None, |s| Some(slug::slugify(&s))),
        input.locale,
        input.translation_of
    )
    .fetch_one(conn)
    .await
    .map_err(conflict_error)
}

pub async fn delete_category(
    conn: &mut PgConnection, category_id: Uuid, account_id: Uuid,
) -> Result<Category, Error> {
    Ok(sqlx::query_as!(
        Category,
        r#"
        DELETE FROM blogs.categories
        WHERE id = $1 
        AND account_id = $2
        RETURNING *
        "#,
        category_id,
        account_id
    )
    .fetch_one(conn)
    .await?)
}

pub async fn get_category_by_id(
    conn: &mut PgConnection, category_id: Uuid, account_id: Uuid,
) -> Result<Category, Error> {
    Ok(sqlx::query_as!(
        Category,
        r#"
        SELECT * FROM blogs.categories
        WHERE id = $1 
        AND account_id = $2
        "#,
        category_id,
        account_id
    )
    .fetch_one(conn)
    .await?)
}

#[derive(Debug, serde::Deserialize)]
#[typeshare::typeshare]
pub struct CategoryQuery {
    pub name: Option<String>,
    pub locale: Option<String>,
    pub translation_of: Option<Uuid>,
}

pub async fn list_categories(
    conn: &mut PgConnection, account_id: Uuid, query: CategoryQuery,
) -> Result<Vec<Category>, Error> {
    Ok(sqlx::query_as!(
        Category,
        r#"
        SELECT * FROM blogs.categories
        WHERE account_id = $1
        AND ($2::text IS NULL OR name ~* $2)
        AND ($3::text IS NULL OR locale = $3)
        AND ($4::uuid IS NULL OR translation_of = $4)
        "#,
        account_id,
        query.name,
        query.locale,
        query.translation_of
    )
    .fetch_all(conn)
    .await?)
}
