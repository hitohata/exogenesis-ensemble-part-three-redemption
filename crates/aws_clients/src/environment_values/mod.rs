/// The lambda's environment values
pub mod lambda_environment_values {
    use std::env::var;
    use std::sync::OnceLock;

    /// The standard bucket name.
    pub fn standard_bucked_name() -> &'static str {
        static STANDARD_BUCKET: OnceLock<String> = OnceLock::new();
        STANDARD_BUCKET.get_or_init(|| var("STANDARD_BUCKET_NAME").unwrap())
    }
}

/// The AWS Clients
pub mod clients {
    use tokio::sync::OnceCell;

    static S3_CLIENT: OnceCell<aws_sdk_s3::Client> = OnceCell::const_new();

    /// The s3 client
    pub async fn s3_client() -> &'static aws_sdk_s3::Client {
        S3_CLIENT
            .get_or_init(|| async {
                let config = aws_config::load_from_env().await;
                aws_sdk_s3::Client::new(&config)
            })
            .await
    }
}
