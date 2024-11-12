//! This mod has s3-related functions.

use std::time::Duration;
use aws_sdk_s3::operation::list_objects::ListObjectsOutput;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;
use aws_sdk_s3::presigning::{PresigningConfig};
use lambda_http::tracing::error;
use time_file_name::FilePath;
use crate::error::WebApiAppError;
use crate::routes::return_types::return_data_types::{
    DaysVideos, MonthsVideos, VideoObjects, YearsVideos,
};
use crate::static_values::clients::s3_client;
use crate::static_values::lambda_environment_values::standard_bucked_name;

/// The expiring time for the s3 pre-signed URL
static PRE_SIGN_EXPIRING_TIME: Duration = Duration::from_secs(5 * 60);

/// Read the years that exist items in the s3 bucket.
pub async fn get_years() -> Result<YearsVideos, WebApiAppError> {
    
    let result = s3_client()
        .await
        .list_objects_v2()
        .bucket(standard_bucked_name())
        .delimiter("/")
        .send()
        .await;
    
    let output = match result {
        Ok(out) => out,
            Err(e) => {
                error!("Failed to get objects: {}", e);
                return Err(WebApiAppError::StorageError("Get years failed".to_string()))
            }
    };
    
    let years: Vec<String> = retrieve_prefixes(&output);

    Ok(YearsVideos { years })
}

/// Read the month that existing items are narrowed down by year in the s3 bucket.
pub async fn get_months(years: usize) -> Result<MonthsVideos, WebApiAppError> {
    
    let result = s3_client()
        .await
        .list_objects_v2()
        .bucket(standard_bucked_name())
        .prefix(format!("{}/", years))
        .delimiter("/")
        .send()
        .await;
    
    let output = match result {
        Ok(out) => out,
            Err(e) => {
                error!("Failed to get objects: {}", e);
                return Err(WebApiAppError::StorageError("Get months failed".to_string()))
            }
    };
   
    let removed_delimiter: Vec<String> = retrieve_prefixes(&output);
    let months: Vec<String> = removed_delimiter.iter().map(|st| st.split("/").collect::<Vec<&str>>()[1].to_string()).collect();

    Ok(MonthsVideos { months })
}

/// Read the days that existing items are narrowed down by year and month in the s3 bucket.
pub async fn get_days(years: usize, months: usize) -> Result<DaysVideos, WebApiAppError> {
    
        let result = s3_client()
        .await
        .list_objects_v2()
        .bucket(standard_bucked_name())
        .prefix(format!("{}/{}/", years, months))
        .delimiter("/")
        .send()
        .await;
    
    let output = match result {
        Ok(out) => out,
            Err(e) => {
                error!("Failed to get objects: {}", e);
                return Err(WebApiAppError::StorageError("Get days failed".to_string()))
            }
    };
   
    let removed_delimiter: Vec<String> = retrieve_prefixes(&output);
    let days: Vec<String> = removed_delimiter.iter().map(|st| st.split("/").collect::<Vec<&str>>()[2].to_string()).collect();

    
    Ok(DaysVideos { days })
}

/// Read the objects that existing items are narrowed down by year, month and day in the s3 bucket.
pub async fn get_objects(_year: usize, _month: usize, _day: usize) -> Result<VideoObjects, String> {
    Ok(VideoObjects { objects: vec![] })
}

/// get a date time as an argument and return the [s3 pre-signed URL](https://docs.aws.amazon.com/AmazonS3/latest/userguide/ShareObjectPreSignedURL.html)
/// The expiring time is 3600 sec
/// The date time in the argument must be ISO
pub async fn generate_pre_signed_url_for_video(date_time: &str, extension: &str) -> Result<String, WebApiAppError> {
    let config = match PresigningConfig::expires_in(PRE_SIGN_EXPIRING_TIME) {
        Ok(config) => config,
        Err(_) => return Err(WebApiAppError::ValidationError("Too long expiring is provided".to_string())),
    };

    let s3_client = s3_client().await;

    let file_path = match FilePath::new().generate_file_path(date_time, extension) {
        Ok(file_path) => file_path,
        Err(e) => return Err(WebApiAppError::ValidationError(e))
    };

    get_pre_signed_url(s3_client, config, file_path.as_str()).await
}

/// Calling the s3 bucket to get the pre-signed URL.
/// [see](https://docs.aws.amazon.com/AmazonS3/latest/API/s3_example_s3_Scenario_PresignedUrl_section.html)
/// [errors](https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html)
async fn get_pre_signed_url(client: &aws_sdk_s3::Client, config: PresigningConfig, file_path: &str) -> Result<String, WebApiAppError> {
    let pre_signed_request_result = client
        .put_object()
        .bucket(standard_bucked_name())
        .key(file_path)
        .presigned(config)
        .await;

    match pre_signed_request_result {
        Ok(result) => Ok(result.uri().into()),
        Err(e) => {
            error!("{}", e.to_string());
            Err(WebApiAppError::StorageError("Get pre-signed url failed".to_string()))
        }
    }
}

/// gets a s3 list object output and returns prefixes
fn retrieve_prefixes(output: &ListObjectsV2Output) -> Vec<String> {
    let prefixes = output.common_prefixes();
    
    if prefixes.len() == 0 {
        return vec![];
    }
    
    let mut result: Vec<String> = vec![];
    
    for prefix in prefixes {
        if let Some(prefix_value) = prefix.prefix() {
            result.push(remove_delimiter(prefix_value).to_string());    
        };
    };
    
    result
}

/// remove "/" from the string
fn remove_delimiter(prefix: &str) -> &str {
    if &prefix[prefix.len() - 1..] != "/" {
        return prefix
    }
    &prefix[..prefix.len() - 1] 
}

#[cfg(test)]
mod test_remove_delimiter {
    use super::*;
    
    #[test]
    fn with_delimiter() {
        // Arrange
        let prefix = "1984/4/4/";
        
        // Act
        let result = remove_delimiter(prefix);
        
        // Assert
        assert_eq!(result, "1984/4/4");
    }
    
    
    #[test]
    fn without_delimiter() {
        // Arrange
        let prefix = "1984/4/4";
        
        // Act
        let result = remove_delimiter(prefix);
        
        // Assert
        assert_eq!(result, "1984/4/4");
    }
}