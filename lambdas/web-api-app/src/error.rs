use axum::http::StatusCode;
use thiserror::Error;
use axum::response::IntoResponse;
use crate::error::WebApiAppError::{StorageError, ValidationError};

#[derive(Error, Debug)]
pub enum WebApiAppError {
    #[error("{0}")]
    ValidationError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
}

impl WebApiAppError {
    /// Return the axum error from the thiserror enum
    pub fn return_http_response(&self) -> impl IntoResponse {
        match self {
            ValidationError(reason) => (StatusCode::BAD_REQUEST, reason.to_owned()).into_response(),
            StorageError(reason) => (StatusCode::INTERNAL_SERVER_ERROR, reason.to_owned()).into_response(),
        }
    }
}