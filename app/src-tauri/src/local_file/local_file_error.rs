//! The local file errors
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ExogenesisEnsembleLocalFileErrors {
    #[error("Reading metadata error: {0}")]
    ReadingMetaDataError(String),
    #[error("File not found: {0}")]
    FileNotFoundError(String),
    #[error("Invalid date time is provided: {0}")]
    InvalidDateError(String),
    #[error("Directory mound failed")]
    DirectoryMoundFailed,
    #[error("Invalid Extension is provided")]
    InvalidExtensionError,
    #[error("File already exists")]
    FileCopyFailed,
}
