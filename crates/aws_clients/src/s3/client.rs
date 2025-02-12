use crate::s3::environment_value::{s3_client, standard_bucked_name};
use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;
use aws_sdk_s3::presigning::PresigningConfig;
use shared::traits::GetFileListTrait;
use std::future::Future;
use std::time::Duration;
use time_file_name::file_path::FilePath;

/// The expiring time for the s3 pre-signed URL
static PRE_SIGN_EXPIRING_TIME: Duration = Duration::from_secs(5 * 60);

/// The client for the standard bucket
pub struct StandardS3Client {
    pub(crate) client: &'static aws_sdk_s3::Client,
}

impl StandardS3Client {
    pub async fn new() -> Self {
        Self {
            client: s3_client().await,
        }
    }

    /// check if a key provided exists
    pub async fn exists(&self, key: impl Into<String>) -> Result<bool, String> {
        let result = self
            .client
            .head_object()
            .bucket(standard_bucked_name())
            .key(key.into())
            .send()
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => match &e.code() {
                None => Err(e.to_string()),
                Some(code) => {
                    if *code == "NotFound" {
                        Ok(false)
                    } else {
                        Err(e.to_string())
                    }
                }
            },
        }
    }

    /// remove an object
    pub async fn remove_object(&self, key: impl Into<&str>) -> Result<(), String> {
        let result = self
            .client
            .delete_object()
            .bucket(standard_bucked_name())
            .key(key.into())
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait StandardS3ClientTrait {
    fn generate_pre_signed_url_for_video(
        date_time: &str,
        extension: &str,
    ) -> impl Future<Output = Result<String, String>> + Send;
}

impl GetFileListTrait for StandardS3Client {
    async fn get_years(&self) -> Result<Vec<String>, String> {
        let result = self
            .client
            .list_objects_v2()
            .bucket(standard_bucked_name())
            .delimiter("/")
            .send()
            .await;

        let output = match result {
            Ok(out) => out,
            Err(e) => return Err(format!("Failed to get objects: {}", e)),
        };

        Ok(retrieve_prefixes(&output))
    }

    async fn get_months(&self, years: usize) -> Result<Vec<String>, String> {
        let result = self
            .client
            .list_objects_v2()
            .bucket(standard_bucked_name())
            .prefix(format!("{years}/"))
            .delimiter("/")
            .send()
            .await;

        let output = match result {
            Ok(out) => out,
            Err(e) => {
                return Err(format!("Failed to get objects: {}", e));
            }
        };

        let removed_delimiter: Vec<String> = retrieve_prefixes(&output);
        let months: Vec<String> = removed_delimiter
            .iter()
            .map(|st| st.split("/").collect::<Vec<&str>>()[1].to_string())
            .collect();

        Ok(months)
    }

    async fn get_days(&self, year: usize, month: usize) -> Result<Vec<String>, String> {
        let result = self
            .client
            .list_objects_v2()
            .bucket(standard_bucked_name())
            .prefix(format!("{year}/{}/", zero_adder(month)))
            .delimiter("/")
            .send()
            .await;

        let output = match result {
            Ok(out) => out,
            Err(e) => {
                return Err(format!("Failed to get objects: {}", e));
            }
        };

        let removed_delimiter: Vec<String> = retrieve_prefixes(&output);
        let days: Vec<String> = removed_delimiter
            .iter()
            .map(|st| st.split("/").collect::<Vec<&str>>()[2].to_string())
            .collect();

        Ok(days)
    }

    async fn get_objects(
        &self,
        year: usize,
        month: usize,
        day: usize,
    ) -> Result<Vec<String>, String> {
        let result = self
            .client
            .list_objects_v2()
            .bucket(standard_bucked_name())
            .prefix(format!("{year}/{}/{}", zero_adder(month), zero_adder(day)))
            .send()
            .await;

        let output = match result {
            Ok(out) => out,
            Err(e) => {
                return Err(format!("Failed to get objects: {}", e));
            }
        };

        let saved_objects = output.contents();

        let mut objects: Vec<String> = Vec::new();

        for object in saved_objects {
            if let Some(key) = &object.key {
                let key_vec = key.split('/').collect::<Vec<&str>>();
                if key_vec.len() == 4 {
                    objects.push(key_vec[3].to_owned())
                }
            }
        }

        Ok(objects)
    }
}

impl StandardS3ClientTrait for StandardS3Client {
    /// get a date time as an argument and return the [s3 pre-signed URL](https://docs.aws.amazon.com/AmazonS3/latest/userguide/ShareObjectPreSignedURL.html)
    /// The expiring time is 3600 sec
    /// The date time in the argument must be ISO
    async fn generate_pre_signed_url_for_video(
        date_time: &str,
        extension: &str,
    ) -> Result<String, String> {
        let config = match PresigningConfig::expires_in(PRE_SIGN_EXPIRING_TIME) {
            Ok(config) => config,
            Err(_) => return Err("Too long expiring is provided".to_string()),
        };

        let s3_client = s3_client().await;

        let file_path = match FilePath::new().generate_file_path(date_time, extension) {
            Ok(file_path) => file_path,
            Err(e) => return Err(e),
        };

        get_pre_signed_url(s3_client, config, file_path.as_str()).await
    }
}

/// Calling the s3 bucket to get the pre-signed URL.
/// [see](https://docs.aws.amazon.com/AmazonS3/latest/API/s3_example_s3_Scenario_PresignedUrl_section.html)
/// [errors](https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html)
async fn get_pre_signed_url(
    client: &aws_sdk_s3::Client,
    config: PresigningConfig,
    file_path: &str,
) -> Result<String, String> {
    let pre_signed_request_result = client
        .put_object()
        .bucket(standard_bucked_name())
        .key(file_path)
        .presigned(config)
        .await;

    match pre_signed_request_result {
        Ok(result) => Ok(result.uri().into()),
        Err(e) => Err(format!("{}", e.to_string())),
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
    }

    result
}

/// remove "/" from the string
fn remove_delimiter(prefix: &str) -> &str {
    if &prefix[prefix.len() - 1..] != "/" {
        return prefix;
    }
    &prefix[..prefix.len() - 1]
}

/// Add "0" if the number is less than 9.
/// The prefix of month and day is 2 digits.
/// ```rust
/// # use aws_clients::s3::client::zero_adder;
/// assert_eq!(zero_adder(1), String::from("01"));
/// assert_eq!(zero_adder(10), String::from("10"));
/// ```
pub fn zero_adder(num: usize) -> String {
    if num < 9 {
        format!("0{}", num)
    } else {
        format!("{num}")
    }
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

#[cfg(test)]
mod client_test {
    use super::*;

    mod test_exists {
        use super::*;

        #[tokio::test]
        async fn test_exists() {
            let result = StandardS3Client::new()
                .await
                .exists("1984/04/04/1984-04-04-12-34-50.MOV")
                .await
                .unwrap();

            assert_eq!(result, true);
        }

        #[tokio::test]
        async fn test_not_exists() {
            let result = StandardS3Client::new()
                .await
                .exists("no-key")
                .await
                .unwrap();

            assert_eq!(result, false);
        }
    }

    mod test_get_years {
        use super::*;

        #[tokio::test]
        async fn test_get_years() {
            // Assert
            let result = StandardS3Client::new().await.get_years().await.unwrap();

            assert_eq!(result, ["1984", "1985"])
        }
    }

    mod test_get_months {
        use super::*;

        #[tokio::test]
        async fn test_get_months() {
            // Assert
            let result = StandardS3Client::new()
                .await
                .get_months(1984)
                .await
                .unwrap();

            assert_eq!(result, ["04", "05"])
        }
    }

    mod test_get_days {
        use super::*;

        #[tokio::test]
        async fn test_get_days() {
            // Assert
            let result = StandardS3Client::new()
                .await
                .get_days(1984, 4)
                .await
                .unwrap();

            assert_eq!(result, ["04", "05"])
        }
    }

    mod test_get_objects {
        use super::*;

        #[tokio::test]
        async fn test_get_objects() {
            // Assert
            let result = StandardS3Client::new()
                .await
                .get_objects(1984, 4, 4)
                .await
                .unwrap();

            assert_eq!(
                result,
                ["1984-04-04-12-34-50.MOV", "1984-04-04-12-34-51.MOV"]
            )
        }
    }

    mod test_remove_object {
        use super::*;
        use crate::s3::test_utils::put_test_object;

        #[tokio::test]
        async fn test_remove_object() {
            // Arrange
            let key_name = "key";
            let _ = put_test_object(key_name).await;
            let client = StandardS3Client::new();

            // Act
            let result = client.await.remove_object(key_name).await;

            // Assert
            assert!(result.is_ok());
        }
    }
}
