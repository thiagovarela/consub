use std::sync::Arc;

use aide::{
    axum::{
        routing::{get, get_with},
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
    redoc::Redoc,
};
use axum::{response::IntoResponse, Extension, Json};

use shared::AppState;

pub fn docs_routes(state: AppState) -> ApiRouter {
    

    ApiRouter::new()
        .api_route_with(
            "/",
            get_with(
                Redoc::new("/docs/private/api.json")
                    .with_title("Consub API Documentation")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        .route("/private/api.json", get(serve_docs))
        .with_state(state)
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}

pub fn public_docs(state: AppState) -> ApiRouter {
    

    ApiRouter::new()
        .route("/api.json", get(serve_docs))
        .with_state(state)
}
