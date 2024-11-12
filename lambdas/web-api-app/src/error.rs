use crate::error::WebApiAppError::{StorageError, ValidationError};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebApiAppError {
    #[error("{0}")]
    #[allow(dead_code)] // TODO: review
    ValidationError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
}

impl WebApiAppError {
    /// Return the axum error from the thiserror enum
    pub fn return_http_response(&self) -> impl IntoResponse {
        match self {
            ValidationError(reason) => (StatusCode::BAD_REQUEST, reason.to_owned()).into_response(),
            StorageError(reason) => {
                (StatusCode::INTERNAL_SERVER_ERROR, reason.to_owned()).into_response()
            }
        }
    }
}
