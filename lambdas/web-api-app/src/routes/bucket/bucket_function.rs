//! This mod has s3-related functions.

use crate::error::WebApiAppError;
use crate::routes::return_types::return_data_types::{
    DaysVideos, MonthsVideos, VideoObjects, YearsVideos,
};
use aws_clients::s3::client::{StandardS3Client, StandardS3ClientTrait};
use lambda_http::tracing::error;
use shared::traits::GetFileListTrait;

/// Read the years that exist items in the s3 bucket.
pub async fn get_years() -> Result<YearsVideos, WebApiAppError> {
    match StandardS3Client::new().await.get_years().await {
        Ok(years) => Ok(YearsVideos { years }),
        Err(e) => {
            error!(e);
            Err(WebApiAppError::StorageError("Get years failed".to_string()))
        }
    }
}

/// Read the month that existing items are narrowed down by year in the s3 bucket.
pub async fn get_months(year: usize) -> Result<MonthsVideos, WebApiAppError> {
    match StandardS3Client::new().await.get_months(year).await {
        Ok(months) => Ok(MonthsVideos { months }),
        Err(e) => {
            error!(e);
            Err(WebApiAppError::StorageError(
                "Get months failed".to_string(),
            ))
        }
    }
}

/// Read the days that existing items are narrowed down by year and month in the s3 bucket.
pub async fn get_days(years: usize, months: usize) -> Result<DaysVideos, WebApiAppError> {
    match StandardS3Client::new().await.get_days(years, months).await {
        Ok(days) => Ok(DaysVideos { days }),
        Err(e) => {
            error!(e);
            Err(WebApiAppError::StorageError("Get days failed".to_string()))
        }
    }
}

/// Read the objects that existing items are narrowed down by year, month and day in the s3 bucket.
pub async fn get_objects(
    year: usize,
    month: usize,
    day: usize,
) -> Result<VideoObjects, WebApiAppError> {
    match StandardS3Client::new()
        .await
        .get_objects(year, month, day)
        .await
    {
        Ok(objects) => Ok(VideoObjects { objects }),
        Err(e) => {
            error!(e);
            Err(WebApiAppError::StorageError(
                "Get objects failed".to_string(),
            ))
        }
    }
}

pub async fn generate_pre_signed_url_for_upload(
    date_time: &str,
    extension: &str,
) -> Result<String, WebApiAppError> {
    match StandardS3Client::generate_pre_signed_url_for_video(date_time, extension).await {
        Ok(url) => Ok(url),
        Err(e) => {
            error!("{}", e);
            Err(WebApiAppError::StorageError(
                "generate_pre_signed_url_for_video failed".to_string(),
            ))
        }
    }
}
