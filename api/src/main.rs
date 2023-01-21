use axum::{extract::State, http::StatusCode};
use shared::{database_pool, AppState};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub async fn health_check(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'OK'")
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't connect to database".to_string(),
            )
        })
}

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let trace_format = tracing_subscriber::fmt::format()
        .with_target(false) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .compact();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().event_format(trace_format))
        .init();

    let db_str =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");
    let address: SocketAddr = "[::0]:8000".parse()?;

    let pool = database_pool(&db_str).await;
    sqlx::migrate!("../migrations").run(&pool).await?;

    let app_state = AppState { db_pool: pool };

    let app = axum::Router::new()
        .nest("/accounts", accounts::routes(app_state.clone()))
        .nest("/blogs", blogs::routes(app_state))
        .layer(TraceLayer::new_for_http());

    info!("Listening on {}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
