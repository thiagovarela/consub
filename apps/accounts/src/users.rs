use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::error::{conflict_error, Error};

use crate::passwords::hash_password;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub account_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct CreateUserWithPasswordInput {
    pub account_id: Uuid,
    pub email: String,
    pub password: String,
}

pub async fn create_user_with_password(
    conn: &mut PgConnection,
    input: CreateUserWithPasswordInput,
) -> Result<User, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO accounts.users (account_id)
        VALUES ($1)
        RETURNING id, account_id, created_at, updated_at
        "#,
        input.account_id,
    )
    .fetch_one(&mut *conn)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO accounts.passwords (user_id, account_id, email, hash_password)
        VALUES ($1, $2, $3, $4)
        "#,
        user.id,
        user.account_id,
        input.email,
        hash_password(input.password)?,
    )
    .execute(&mut *conn)
    .await
    .map_err(|e| conflict_error(e, "passwords_account_id_email_idx", "E-mail already in use"))?;

    Ok(user)
}

pub async fn get_user_by_id(conn: &sqlx::PgPool, id: Uuid) -> Result<User, Error> {
    Ok(sqlx::query_as!(
        User,
        r#"
        SELECT id, account_id, created_at, updated_at
        FROM accounts.users
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(conn)
    .await?)
}
