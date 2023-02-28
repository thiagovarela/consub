use aide::{axum::routing::get_with, transform::TransformOperation};
use aide::{
    axum::ApiRouter,
    openapi::{OpenApi, Tag},
    transform::TransformOpenApi,
};
use axum::{debug_handler, extract::State, http::StatusCode, Extension};
use shared::{database_pool, AppState};
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};

use axum_tracing_opentelemetry::opentelemetry_tracing_layer;

use tracing::info;
use tracing_subscriber::layer::SubscriberExt;

use crate::docs::docs_routes;

mod docs;
#[debug_handler]
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
fn health_checkdocs(op: TransformOperation) -> TransformOperation {
    op.description("Health check endpoint")
        .response::<200, String>()
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Consub API")
        .summary("Consub application")
        .description("Open API documentation.")
        .tag(Tag {
            name: "clippings".into(),
            description: Some("Clippings Management".into()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Api-Key".into(),
                description: Some("A key to access resources of your account.".into()),
                extensions: Default::default(),
            },
        )
        .security_scheme(
            "Bearer",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "Authorization".into(),
                description: Some("Authorization Bearer Token.".into()),
                extensions: Default::default(),
            },
        )
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), axum::BoxError> {
    init_tracing()?;

    let db_str =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");
    let address: SocketAddr = "[::0]:8000".parse()?;

    let pool = database_pool(&db_str).await;
    sqlx::migrate!("../migrations").run(&pool).await?;

    let app_state = AppState { db_pool: pool };

    aide::gen::extract_schemas(true);

    let mut api = OpenApi::default();

    let app = ApiRouter::new()
        .nest("/accounts", accounts::routes(app_state.clone()).into())
        .nest("/blogs", blogs::routes(app_state.clone()).into())
        .nest("/clippings", clippings::routes(app_state.clone()).into())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .nest_api_service("/docs", docs_routes(app_state.clone()))
        .api_route(
            "/health",
            get_with(health_check, health_checkdocs).with_state(app_state.clone()),
        )
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)));

    info!("Listening on {}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn init_tracing() -> Result<(), axum::BoxError> {
    use tracing_subscriber::filter::EnvFilter;

    let subscriber = tracing_subscriber::registry();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info,sqlx=debug,tower_http=debug,aide=off")
    }

    let subscriber = subscriber.with(EnvFilter::from_default_env());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()                
        .flatten_event(true);
    let subscriber = subscriber.with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
