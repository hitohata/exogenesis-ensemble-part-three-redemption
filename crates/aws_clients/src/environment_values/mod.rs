/// The lambda's environment values

pub mod lambda_environment_values {
    use std::env::var;
    use std::sync::OnceLock;

    /// The standard bucket name.
    pub fn standard_bucked_name() -> &'static str {
        static STANDARD_BUCKET: OnceLock<String> = OnceLock::new();
        STANDARD_BUCKET.get_or_init(|| var("STANDARD_BUCKET_NAME").unwrap())
    }

    /// DynamoDB table name
    #[allow(dead_code)] // TODO: Delete
    pub fn table_name() -> &'static str {
        static TABLE_NAME: OnceLock<String> = OnceLock::new();
        TABLE_NAME.get_or_init(|| var("TABLE_NAME").unwrap())
    }
}

/// The AWS Clients
pub mod clients {
    use tokio::sync::OnceCell;

    static S3_CLIENT: OnceCell<aws_sdk_s3::Client> = OnceCell::const_new();
    static DYNAMODB_CLIENT: OnceCell<aws_sdk_dynamodb::Client> = OnceCell::const_new();

    /// The s3 client
    pub async fn s3_client() -> &'static aws_sdk_s3::Client {
        S3_CLIENT
            .get_or_init(|| async {
                let config = aws_config::load_from_env().await;
                aws_sdk_s3::Client::new(&config)
            })
            .await
    }

    /// The DynamoDB client
    #[allow(dead_code)] // TODO: fix
    pub async fn dynamodb_client() -> &'static aws_sdk_dynamodb::Client {
        DYNAMODB_CLIENT
            .get_or_init(|| async {
                let config = aws_config::load_from_env().await;
                aws_sdk_dynamodb::Client::new(&config)
            })
            .await
    }

    #[cfg(test)]
    /// test client
    /// when this function is called at the first time, the
    pub async fn test_dynamo_client() -> &'static aws_sdk_dynamodb::Client {
        use aws_config::BehaviorVersion;
        use aws_config::Region;
        use aws_sdk_dynamodb::config::Credentials;

        DYNAMODB_CLIENT
            .get_or_init(|| async {
                let config = aws_config::defaults(BehaviorVersion::latest())
                    .endpoint_url("http://localhost:8000")
                    .region(Some(Region::new("us-west-2")))
                    .credentials_provider(Credentials::new("key", "secret", None, None, "test"))
                    .load()
                    .await;
                aws_sdk_dynamodb::Client::new(&config)
            })
            .await
    }
}
