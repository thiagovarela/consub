use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::error::{conflict_error, Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub slug: String,
    pub locale: String,
    pub translation_of: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryInput {
    pub name: String,
    pub locale: String,
    pub translation_of: Option<Uuid>,
}

pub async fn create_category(
    conn: &mut PgConnection,
    account_id: Uuid,
    input: CreateCategoryInput,
) -> Result<Category, Error> {
    Ok(sqlx::query_as!(
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
    .map_err(|e| conflict_error(e, "categories_slug_account_id_idx", "Slug already in use"))?)
}

#[derive(Debug, Deserialize)]
pub struct ChangeCategoryInput {
    pub name: Option<String>,
    pub locale: Option<String>,
    pub slug: Option<String>,
    pub translation_of: Option<Uuid>,
}

pub async fn change_category(
    conn: &mut PgConnection,
    category_id: Uuid,
    account_id: Uuid,
    input: ChangeCategoryInput,
) -> Result<Category, Error> {
    Ok(sqlx::query_as!(
        Category,
        r#"
        UPDATE blogs.categories
        SET name = coalesce($3, name),
            slug = coalesce($4, slug),
            locale = coalesce($5, locale),
            translation_of = coalesce($6, translation_of)
        WHERE id = $1 and account_id = $2
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
    .map_err(|e| conflict_error(e, "categories_slug_account_id_idx", "Slug already in use"))?)
}

pub async fn remove_category(
    conn: &mut PgConnection,
    category_id: Uuid,
    account_id: Uuid,
) -> Result<Category, Error> {
    Ok(sqlx::query_as!(
        Category,
        r#"
        DELETE FROM blogs.categories
        WHERE id = $1 and account_id = $2
        RETURNING *
        "#,
        category_id,
        account_id
    )
    .fetch_one(conn)
    .await?)
}

pub async fn get_category_by_id(
    conn: &mut PgConnection,
    category_id: Uuid,
    account_id: Uuid,
) -> Result<Category, Error> {
    Ok(sqlx::query_as!(
        Category,
        r#"
        SELECT * FROM blogs.categories
        WHERE id = $1 and account_id = $2
        "#,
        category_id,
        account_id
    )
    .fetch_one(conn)
    .await?)
}

pub async fn list_categories(
    conn: &mut PgConnection,
    account_id: Uuid,
) -> Result<Vec<Category>, Error> {
    Ok(sqlx::query_as!(
        Category,
        r#"
        SELECT * FROM blogs.categories
        WHERE account_id = $1
        "#,
        account_id
    )
    .fetch_all(conn)
    .await?)
}
