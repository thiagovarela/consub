use axum::extract::FromRef;
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};
use std::ops::Deref;
use std::sync::Arc;

pub mod pagination;

pub async fn database_pool(db_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(15)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                conn.execute("SET application_name = 'consub'; SET search_path = 'public';")
                    .await?;
                Ok(())
            })
        })
        .connect(db_url)
        .await
        .expect("Can't connect to database")
}

#[derive(Debug, Clone)]
pub struct OpendalUploader(pub Arc<opendal::Operator>);

impl Deref for OpendalUploader {
    type Target = Arc<opendal::Operator>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub opendal: OpendalUploader,
}

pub mod testing {
    use std::net::TcpListener;

    use tracing_subscriber::EnvFilter;

    use crate::OpendalUploader;

    pub fn test_opendal_uploader() -> OpendalUploader {
        let builder = opendal::services::Memory::default();
        let op: opendal::Operator = opendal::Operator::new(builder).unwrap().finish();
        let opendal = crate::OpendalUploader(std::sync::Arc::new(op));
        opendal
    }

    pub async fn test_app(routes: aide::axum::ApiRouter) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bind to address");
        let port = listener.local_addr().unwrap().port();
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_max_level(tracing::Level::INFO)
            .compact()
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap_or_default();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(routes.into_make_service())
                .await
                .expect("failed to start")
        });

        format!("http://127.0.0.1:{port}")
    }
}
