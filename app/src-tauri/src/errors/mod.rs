use thiserror::Error;

/// The errors
#[derive(Error, Debug)]
pub enum ExogenesisError {
    #[error("Reading error: {:?}")]
    FileReadError(String),
    #[error("Writing error: {:?}")]
    FileWriteError(String),
}