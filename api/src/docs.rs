use std::sync::Arc;

use aide::{
    openapi::OpenApi, axum::{ApiRouter, routing::get, IntoApiResponse},    
};
use axum::{response::IntoResponse, Extension, Json};

use shared::AppState;

pub fn docs_routes() -> ApiRouter<AppState> {
    ApiRouter::new().route("/private/api.json", get(serve_docs))
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}

pub fn public_docs() -> ApiRouter<AppState> {
    ApiRouter::new()
        .route("/api.json", get(serve_docs))        
}
