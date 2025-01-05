//! S3 client
//! to see the data check the storage/ready.d

use std::env::var;
use std::sync::OnceLock;
use tokio::sync::OnceCell;

/// read data from environment value
pub(crate) fn standard_bucked_name() -> &'static str {
    static STANDARD_BUCKET: OnceLock<String> = OnceLock::new();
    STANDARD_BUCKET.get_or_init(|| {
        var("STANDARD_BUCKET_NAME").unwrap_or_else(|_| {
            if cfg!(test) {
                "test-bucket".to_string()
            } else {
                log::error!("{}", String::from("Bucket is not found"));
                panic!("variable not found")
            }
        })
    })
}

#[cfg(test)]
fn bucket_url() -> &'static str {
    static HOST_NAME: OnceLock<String> = OnceLock::new();
    HOST_NAME.get_or_init(|| {
        if let Ok(host) = var("HOST_NAME") {
            format!("http://{host}")
        } else {
            "http://localhost:4566".to_string()
        }
    })
}

static S3_CLIENT: OnceCell<aws_sdk_s3::Client> = OnceCell::const_new();

/// The s3 client
#[cfg(not(test))]
pub async fn s3_client() -> &'static aws_sdk_s3::Client {
    S3_CLIENT
        .get_or_init(|| async {
            let config = aws_config::load_from_env().await;
            aws_sdk_s3::Client::new(&config)
        })
        .await
}

#[cfg(test)]
pub async fn s3_client() -> &'static aws_sdk_s3::Client {
    use aws_config::BehaviorVersion;
    use aws_config::Region;
    use aws_sdk_s3::config::Credentials;

    S3_CLIENT
        .get_or_init(|| async {
            let config = aws_config::defaults(BehaviorVersion::latest())
                .endpoint_url(bucket_url())
                .region(Some(Region::new("us-west-2")))
                .credentials_provider(Credentials::new("key", "secret", None, None, "test"))
                .load()
                .await;

            let mut config_builder = aws_sdk_s3::config::Builder::from(&config);
            config_builder.set_force_path_style(Some(true));
            let client = config_builder.build();

            aws_sdk_s3::Client::from_conf(client)
        })
        .await
}
