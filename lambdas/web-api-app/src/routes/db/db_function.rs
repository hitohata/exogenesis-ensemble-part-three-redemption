//! DB related functions

use crate::error::WebApiAppError;
use crate::routes::return_types::return_data_types::{
    DaysVideos, MonthsVideos, VideoObjects, YearsVideos,
};
use aws_clients::dynamodb::client::{DynamoClientTrait, DynamoDbClient};
use aws_clients::s3::client::StandardS3Client;
use lambda_http::tracing::error;
use shared::traits::GetFileListTrait;

/// get years that stored in the DB
pub async fn get_years() -> Result<YearsVideos, WebApiAppError> {
    match DynamoDbClient::new().await.get_years().await {
        Ok(years) => Ok(YearsVideos { years }),
        Err(e) => Err(WebApiAppError::DBError(e)),
    }
}

/// get months that stored in the DB
pub async fn get_months(year: usize) -> Result<MonthsVideos, WebApiAppError> {
    match DynamoDbClient::new().await.get_months(year).await {
        Ok(months) => Ok(MonthsVideos { months }),
        Err(e) => Err(WebApiAppError::DBError(e)),
    }
}

/// get days that stored in the DB
pub async fn get_days(year: usize, month: usize) -> Result<DaysVideos, WebApiAppError> {
    match DynamoDbClient::new().await.get_days(year, month).await {
        Ok(days) => Ok(DaysVideos { days }),
        Err(e) => Err(WebApiAppError::DBError(e)),
    }
}

/// get objects that stored in the DB
pub async fn get_objects(
    year: usize,
    month: usize,
    day: usize,
) -> Result<VideoObjects, WebApiAppError> {
    match DynamoDbClient::new()
        .await
        .get_objects(year, month, day)
        .await
    {
        Ok(objects) => Ok(VideoObjects { objects }),
        Err(e) => Err(WebApiAppError::StorageError(
            "Get objects failed".to_string(),
        )),
    }
}
