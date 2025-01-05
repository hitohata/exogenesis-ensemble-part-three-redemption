use std::env::var;
use std::sync::OnceLock;
use tokio::sync::OnceCell;

static DYNAMODB_CLIENT: OnceCell<aws_sdk_dynamodb::Client> = OnceCell::const_new();

#[cfg(not(test))]
pub fn table_name() -> &'static str {
    static TABLE_NAME: OnceLock<String> = OnceLock::new();
    TABLE_NAME.get_or_init(|| var("TABLE_NAME").unwrap())
}

#[cfg(test)]
fn dynamo_db_url() -> &'static str {
    static HOST_NAME: OnceLock<String> = OnceLock::new();
    HOST_NAME.get_or_init(|| {
        if let Ok(host) = var("DYNAMO_HOST") {
            format!("http://{host}")
        } else {
            "http://localhost:8000".to_string()
        }
    })
}

/// The DynamoDB client
#[allow(dead_code)] // TODO: fix
#[cfg(all(not(test), feature = "db"))]
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
pub async fn dynamodb_client() -> &'static aws_sdk_dynamodb::Client {
    use aws_config::BehaviorVersion;
    use aws_config::Region;
    use aws_sdk_dynamodb::config::Credentials;

    DYNAMODB_CLIENT
        .get_or_init(|| async {
            let config = aws_config::defaults(BehaviorVersion::latest())
                .endpoint_url(dynamo_db_url())
                .region(Some(Region::new("us-west-2")))
                .credentials_provider(Credentials::new("key", "secret", None, None, "test"))
                .load()
                .await;
            aws_sdk_dynamodb::Client::new(&config)
        })
        .await
}
