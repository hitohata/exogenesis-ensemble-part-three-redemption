use thiserror::Error;

/// The errors
#[derive(Error, Debug)]
pub enum ExogenesisError {
    #[error("Reading error: {0}")]
    FileReadError(String),
    #[error("Writing error: {0}")]
    FileWriteError(String),
}