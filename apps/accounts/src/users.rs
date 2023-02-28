use aide::OperationIo;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::error::{conflict_error, Error};

use crate::passwords::hash_password;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct User {
    pub id: Uuid,
    #[serde(skip)]
    pub account_id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct CreateUserWithPasswordInput {
    pub account_id: Uuid,
    pub email: String,
    pub password: String,
}

pub async fn create_user_with_password(
    conn: &mut PgConnection, input: CreateUserWithPasswordInput,
) -> Result<User, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO accounts.users (account_id, email)
        VALUES ($1, $2)
        RETURNING id, account_id, email, created_at, updated_at
        "#,
        input.account_id,
        input.email.to_lowercase(),
    )
    .fetch_one(&mut *conn)
    .await
    .map_err(conflict_error)?;

    sqlx::query!(
        r#"
        INSERT INTO accounts.passwords (user_id, hash_password)
        VALUES ($1, $2)
        "#,
        user.id,
        hash_password(input.password)?,
    )
    .execute(&mut *conn)
    .await
    .map_err(conflict_error)?;

    Ok(user)
}

pub async fn get_user_by_id(conn: &sqlx::PgPool, id: Uuid) -> Result<User, Error> {
    Ok(sqlx::query_as!(
        User,
        r#"
        SELECT id, account_id, email, created_at, updated_at
        FROM accounts.users
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(conn)
    .await?)
}
