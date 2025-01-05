//! To run tests, it required to run docker compose beforehand

#[cfg(feature = "db")]
pub mod dynamodb;
#[cfg(feature = "standard-storage")]
pub mod s3;
