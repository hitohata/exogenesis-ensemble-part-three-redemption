//! To run tests, it required to run docker compose beforehand

#[cfg(feature = "db")]
pub mod dynamodb;
pub(crate) mod environment_values;
#[cfg(feature = "standard-storage")]
pub mod s3;
