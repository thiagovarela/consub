use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::AppError;
use sqlx::PgConnection;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageView {
    pub id: i64,
    pub ts: DateTime<Utc>,
    pub account_id: Uuid,
    pub path: String,
    pub headers: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct PageViewInput {
    pub path: String,
    pub headers: serde_json::Value,
}

pub async fn create_page_view(
    conn: &mut PgConnection, account_id: Uuid, input: PageViewInput,
) -> Result<(), AppError> {
    let ts = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO analytics.page_views_raw (ts, account_id, path, headers)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        ts,
        account_id,
        input.path,
        input.headers,
    )
    .fetch_one(conn)
    .await?
    .id;

    Ok(())
}
