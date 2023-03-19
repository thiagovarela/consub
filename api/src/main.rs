use aide::{axum::routing::get_with, transform::TransformOperation};
use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::{debug_handler, extract::State, http::StatusCode, Extension};
use shared::{database_pool, AppState, OpendalUploader};
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};

use opendal::{services, Operator};
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
    dotenv::dotenv().ok();
    init_tracing()?;

    let db_str =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");
    let address: SocketAddr = "[::0]:8000".parse()?;

    let pool = database_pool(&db_str).await;
    sqlx::migrate!("../migrations").run(&pool).await?;

    let opendal = opendal_operator()?;

    let app_state = AppState {
        db_pool: pool,
        opendal: OpendalUploader(Arc::new(opendal)),
    };

    aide::gen::extract_schemas(true);

    let mut api = OpenApi::default();

    // TODO: set up rate limiting, cors, etc.

    let app = ApiRouter::new()
        .nest("/accounts", accounts::routes(app_state.clone()).into())
        .nest("/blogs", blogs::routes(app_state.clone()).into())
        .nest("/clippings", clippings::routes(app_state.clone()).into())
        .nest("/media", media::routes(app_state.clone()).into())
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
    let subscriber = subscriber.with(EnvFilter::from_default_env());

    #[cfg(not(debug_assertions))]
    let fmt_layer = tracing_subscriber::fmt::layer().json().flatten_event(true);
    #[cfg(debug_assertions)]
    let fmt_layer = tracing_subscriber::fmt::layer();

    let subscriber = subscriber.with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn opendal_operator() -> Result<opendal::Operator, opendal::Error> {
    // let mut builder = services::Fs::default();
    // builder.root("./tmp");

    let mut builder = services::S3::default();
    builder.bucket("consub-media");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(op)
}
