use crate::error::WebApiAppError::{DBError, StorageError, ValidationError};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use lambda_http::tracing::log;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebApiAppError {
    #[error("{0}")]
    #[allow(dead_code)] // TODO: review
    ValidationError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("DB error: {0}")]
    DBError(String),
}

impl WebApiAppError {
    /// Return the axum error from the thiserror enum
    pub fn return_http_response(&self) -> impl IntoResponse {
        match self {
            ValidationError(reason) => (StatusCode::BAD_REQUEST, reason.to_owned()).into_response(),
            StorageError(reason) => {
                log::error!("{}", reason);
                (StatusCode::INTERNAL_SERVER_ERROR, reason.to_owned()).into_response()
            }
            DBError(reason) => {
                log::error!("{}", reason);
                (StatusCode::INTERNAL_SERVER_ERROR, reason.to_owned()).into_response()
            }
        }
    }
}
