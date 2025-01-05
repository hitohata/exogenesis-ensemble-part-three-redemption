use thiserror::Error;

/// The errors
#[derive(Error, Debug, PartialEq)]
pub enum ExogenesisError {
    #[allow(dead_code)] // TODO: must be removed
    #[error("Reading error: {0}")]
    FileReadError(String),
    #[allow(dead_code)] // TODO: must be removed
    #[error("Writing error: {0}")]
    FileWriteError(String),
    #[error("Couldn't get a read lock at {0}")]
    ReadLockFailed(String),
    #[error("Couldn't get a write lock at {0}")]
    WriteLockFailed(String),
}
