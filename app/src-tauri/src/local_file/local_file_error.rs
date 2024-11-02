//! The local file errors
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ExogenesisEnsembleLocalFileErrors {
    #[error("Reading metadata error: {0}")]
    ReadingMetaDataError(String),
    #[error("File not found: {0}")]
    FileNotFoundError(String),
    #[error("Directory mount failed")]
    DirectoryMountFailed,
    #[error("File already exists")]
    FileCopyFailed,
    #[error("Incorrect extension is provided")]
    ExtensionParseFailed,
    #[error("{0}")]
    FileError(String),
}
