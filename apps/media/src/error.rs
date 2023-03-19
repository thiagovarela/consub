use aide::OperationIo;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_with::DisplayFromStr;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug, OperationIo)]
pub enum Error {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("validation error in request body")]
    InvalidEntity(#[from] ValidationErrors),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[serde_with::serde_as]
        #[serde_with::skip_serializing_none]
        #[derive(serde::Serialize)]
        struct ErrorResponse<'a> {
            // Serialize the `Display` output as the error message
            #[serde_as(as = "DisplayFromStr")]
            message: &'a Error,

            errors: Option<&'a ValidationErrors>,
        }

        let errors = match &self {
            Error::InvalidEntity(errors) => Some(errors),
            _ => None,
        };

        tracing::error!("API error: {:?}", self);

        (
            self.status_code(),
            Json(ErrorResponse {
                message: &self,
                errors,
            }),
        )
            .into_response()
    }
}
