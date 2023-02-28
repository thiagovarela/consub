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

    #[error("validation error in request body")]
    InvalidEntity(#[from] ValidationErrors),

    #[error("{0}")]
    Conflict(String),

    #[error("{0}")]
    ForeignKeyError(String),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Conflict(_) => StatusCode::CONFLICT,
            ForeignKeyError(_) => StatusCode::BAD_REQUEST,
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
    "categories_slug_locale_account_id_unique" => "Category slug for locale is already in use",
    "item_slug_locale_account_id_unique" => "Post slug for locale is already in use",
};
static FOREIGN_KEYS: phf::Map<&'static str, &'static str> = phf_map! {
    "categories_translation_of_fkey" => "Category translation_of must be a valid category",
};

pub fn conflict_error(err: sqlx::Error) -> Error {
    match &err {
        sqlx::Error::Database(dbe) => {
            let constraint = dbe.constraint().unwrap_or_default();
            if let Some(message) = UNIQUES.get(constraint) {
                return Error::Conflict(message.to_string());
            }
            if let Some(message) = FOREIGN_KEYS.get(constraint) {
                return Error::ForeignKeyError(message.to_string());
            }
            Error::Database(err)
        }
        _ => Error::Database(err),
    }
}
