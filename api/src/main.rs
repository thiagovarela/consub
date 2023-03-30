use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::routing::get;
use axum::{debug_handler, extract::State, http::StatusCode, Extension};
use shared::{database_pool, AppState, OpendalUploader};
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};

use opendal::{services, Operator};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;

use crate::docs::{docs_routes, public_docs};

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

fn public_api_docs(api: TransformOpenApi) -> TransformOpenApi {
    // let mut oa = api.inner_mut();
    // Iterates over all paths and check if the extension x-public is set to true.
    // If so, the path is removed from the OpenAPI document.
    // oa.paths = oa.paths.clone().filter(|path| {
    //     path.extensions.get("x-public").map(|ext| ext == "true").unwrap_or(false)
    // });
    // oa.components = oa.components.clone().filter(|component| {
    //     component.extensions.get("x-public").map(|ext| ext == "true").unwrap_or(false)
    // });

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
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), axum::BoxError> {
    dotenv::dotenv().ok();
    init_tracing()?;

    let db_str =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");

    let pool = database_pool(&db_str).await;
    sqlx::migrate!("../migrations").run(&pool).await?;

    let opendal = opendal_operator()?;

    let app_state = AppState {
        db_pool: pool,
        opendal: OpendalUploader(Arc::new(opendal)),
    };

    aide::gen::extract_schemas(true);

    let mut api = OpenApi::default();
    let mut public_api = OpenApi::default();

    let admin = ApiRouter::new()
        .nest("/accounts", accounts::routes(app_state.clone()))
        .nest("/blogs", blogs::routes(app_state.clone()))
        .nest("/clippings", clippings::routes(app_state.clone()))
        .nest("/media", media::routes(app_state.clone()));

    // Prefixes all paths with /admin.
    let admin = ApiRouter::new()
        .nest("/admin", admin)
        .nest_api_service("/admin/docs", docs_routes(app_state.clone()))
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)));

    let public = ApiRouter::new()
        .nest("/blogs", blogs::public_routes(app_state.clone()))
        .nest("/media", media::public_routes(app_state.clone()))
        .nest("/clippings", clippings::public_routes(app_state.clone()))
        .nest_api_service("/docs", public_docs(app_state.clone()))
        .finish_api_with(&mut public_api, public_api_docs)
        .layer(Extension(Arc::new(public_api)));

    // TODO: set up rate limiting, cors, etc.
    let app = ApiRouter::new()
        .merge(admin)
        .merge(public)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .route("/health", get(health_check).with_state(app_state.clone()));

    let address: SocketAddr = "[::0]:8000".parse()?;
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
