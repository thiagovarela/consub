use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_with::DisplayFromStr;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("{0}")]
    PasswordError(String),

    #[error("JWT error")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),

    #[error("{0}")]
    InvalidAccountID(String),

    #[error("Authorization header missing")]
    AuthBearer,

    #[error("Failed to authenticate user")]
    AuthPasswordError,

    #[error("{0}")]
    InvalidAPIKey(String),

    #[error("validation error in request body")]
    InvalidEntity(#[from] ValidationErrors),

    #[error("{0}")]
    UnprocessableEntity(String),

    #[error("{0}")]
    Conflict(String),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEntity(_) | UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Conflict(_) => StatusCode::CONFLICT,
            PasswordError(_) | InvalidToken(_) | AuthPasswordError => StatusCode::UNAUTHORIZED,
            InvalidAccountID(_) => StatusCode::BAD_REQUEST,
            AuthBearer => StatusCode::BAD_REQUEST,
            InvalidAPIKey(_) => StatusCode::FORBIDDEN,
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

pub fn conflict_error(err: sqlx::Error, constraint: &str, message: &str) -> Error {
    match err {
        sqlx::Error::Database(dbe) if dbe.constraint() == Some(constraint) => {
            Error::Conflict(message.into())
        }
        _ => Error::DatabaseError(err.into()),
    }
}
