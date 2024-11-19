pub mod entities;
pub mod client {
    use crate::dynamodb::entities::collection::CollectionItem;
    use crate::environment_values::clients::dynamodb_client;
    use crate::environment_values::lambda_environment_values::table_name;
    use aws_sdk_dynamodb::types::AttributeValue;
    use shared::traits::GetFileListTrait;
    use std::future::Future;
    use std::time::{SystemTime, UNIX_EPOCH};

    pub struct DynamoDbClient {}

    pub trait DynamoClientTrait: GetFileListTrait {
        /// put a new collection item
        fn put_collection_item(
            collection: &CollectionItem,
        ) -> impl Future<Output = Result<bool, String>> + Send;
        // put zipping time
        /// time is mill sec
        fn put_unzipping_item(
            key_name: &str,
            time: Option<u128>,
        ) -> impl Future<Output = Result<bool, String>> + Send;
        /// put unzipped item
        /// time is mill sec
        fn put_unzipped_item(
            key_name: &str,
            time: Option<u128>,
        ) -> impl Future<Output = Result<bool, String>> + Send;
    }

    impl GetFileListTrait for DynamoDbClient {
        async fn get_years() -> Result<Vec<String>, String> {
            Ok(vec![])
        }

        async fn get_month(_years: usize) -> Result<Vec<String>, String> {
            todo!()
        }

        async fn get_days(_year: usize, _month: usize) -> Result<Vec<String>, String> {
            todo!()
        }

        async fn get_objects(
            _year: usize,
            _month: usize,
            _day: usize,
        ) -> Result<Vec<String>, String> {
            todo!()
        }
    }

    impl DynamoClientTrait for DynamoDbClient {
        async fn put_collection_item(collection: &CollectionItem) -> Result<bool, String> {
            let request = dynamodb_client()
                .await
                .put_item()
                .table_name(table_name())
                .item("PK", AttributeValue::S(collection.year.to_string()))
                .item("SK", AttributeValue::N(collection.unix_time.to_string()))
                .item("IsUnzipped", AttributeValue::Bool(collection.is_unzipped))
                .item("Vault", AttributeValue::S(collection.vault.to_string()))
                .item(
                    "KeyName",
                    AttributeValue::S(collection.key_name.to_string()),
                );

            if let Err(e) = request.send().await {
                return Err(e.to_string());
            }

            Ok(true)
        }

        async fn put_unzipping_item(key_name: &str, time: Option<u128>) -> Result<bool, String> {
            let now = get_now(time)?;

            let request = dynamodb_client()
                .await
                .put_item()
                .table_name(table_name())
                .item("PK", AttributeValue::S("Unzipping".to_string()))
                .item("SK", AttributeValue::N(now.to_string()))
                .item("KeyName", AttributeValue::S(key_name.to_string()));

            if let Err(e) = request.send().await {
                return Err(e.to_string());
            }

            Ok(true)
        }

        async fn put_unzipped_item(key_name: &str, time: Option<u128>) -> Result<bool, String> {
            let now = get_now(time)?;

            let request = dynamodb_client()
                .await
                .put_item()
                .table_name(table_name())
                .item("PK", AttributeValue::S("Unzipped".to_string()))
                .item("SK", AttributeValue::N(now.to_string()))
                .item("KeyName", AttributeValue::S(key_name.to_string()));

            if let Err(e) = request.send().await {
                return Err(e.to_string());
            }

            Ok(true)
        }
    }

    /// this is a helper function.
    /// if there is argument, this function returns it.
    /// If not, this function gets system time.
    fn get_now(time: Option<u128>) -> Result<u128, String> {
        match time {
            Some(now) => Ok(now),
            None => match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(now) => Ok(now.as_millis()),
                Err(_) => Err("Failed to get current time".to_string()),
            },
        }
    }
}

#[cfg(test)]
mod test_util;
