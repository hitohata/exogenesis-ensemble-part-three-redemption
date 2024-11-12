pub mod client {
    use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;
    use crate::environment_values::clients::s3_client;
    use crate::environment_values::lambda_environment_values::standard_bucked_name;

    /// The client for the standard bucket
    pub struct StandardS3Client {}

    impl StandardS3Client {
        pub async fn get_years() -> Result<Vec<String>, String> {
            let result = s3_client()
                .await
                .list_objects_v2()
                .bucket(standard_bucked_name())
                .delimiter("/")
                .send()
                .await;

            let output = match result {
                Ok(out) => out,
                Err(e) => return Err(format!("Failed to get objects: {}", e))
            };

            Ok(retrieve_prefixes(&output))
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
}