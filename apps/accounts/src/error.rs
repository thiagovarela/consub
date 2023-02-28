use aide::OperationIo;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use phf::phf_map;
use serde_with::DisplayFromStr;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug, OperationIo)]
pub enum Error {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("{0}")]
    PasswordHash(String),

    #[error("{0}")]
    InvalidAccountID(String),

    #[error("{0}")]
    AccessTokenKeypair(String),

    #[error("{0}")]
    AccessTokenInvalid(String),

    #[error("Failed to verify access token signature")]
    AccessTokenSignature,

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
            Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEntity(_) | UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Conflict(_) => StatusCode::CONFLICT,
            PasswordHash(_) | AccessTokenSignature | AuthPasswordError => StatusCode::UNAUTHORIZED,
            InvalidAccountID(_) | AuthBearer => StatusCode::BAD_REQUEST,
            InvalidAPIKey(_) => StatusCode::FORBIDDEN,
            AccessTokenKeypair(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AccessTokenInvalid(_) => StatusCode::BAD_REQUEST,
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

static UNIQUES: phf::Map<&'static str, &'static str> = phf_map! {
    "accounts_subdomain_key" => "Subdomain is already in use",
    "passwords_account_id_email_unique" => "Email is already in use",
};

pub fn conflict_error(err: sqlx::Error) -> Error {
    match &err {
        sqlx::Error::Database(dbe) => {
            let constraint = dbe.constraint().unwrap_or_default();
            if let Some(message) = UNIQUES.get(constraint) {
                return Error::Conflict(message.to_string());
            }
            Error::Database(err)
        }
        _ => Error::Database(err),
    }
}
