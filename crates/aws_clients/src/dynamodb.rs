pub mod entities;
pub mod client {
    use crate::dynamodb::entities::collection::CollectionItem;
    use crate::environment_values::dynamo::{dynamodb_client, table_name};
    use aws_sdk_dynamodb::types::AttributeValue;
    use shared::traits::GetFileListTrait;
    use std::future::Future;
    use std::time::{SystemTime, UNIX_EPOCH};

    pub struct DynamoDbClient {
        pub(crate) client: &'static aws_sdk_dynamodb::Client,
    }

    impl DynamoDbClient {
        pub async fn new() -> Self {
            Self {
                client: dynamodb_client().await,
            }
        }
    }

    pub trait DynamoClientTrait: GetFileListTrait {
        /// put a new collection item
        fn put_collection_item(
            &self,
            collection: &CollectionItem,
        ) -> impl Future<Output = Result<bool, String>> + Send;
        // put zipping time
        /// time is mill sec
        fn put_unzipping_item(
            &self,
            key_name: &str,
            time: Option<u128>,
        ) -> impl Future<Output = Result<bool, String>> + Send;
        /// put unzipped item
        /// time is mill sec
        fn put_unzipped_item(
            &self,
            key_name: &str,
            time: Option<u128>,
        ) -> impl Future<Output = Result<bool, String>> + Send;
    }

    impl GetFileListTrait for DynamoDbClient {
        async fn get_years(&self) -> Result<Vec<String>, String> {
            get_date_list("root").await
        }

        async fn get_month(&self, year: usize) -> Result<Vec<String>, String> {
            get_date_list(format!("{year}").as_str()).await
        }

        async fn get_days(&self, year: usize, month: usize) -> Result<Vec<String>, String> {
            get_date_list(format!("{year}-{month}").as_str()).await
        }

        async fn get_objects(
            &self,
            year: usize,
            month: usize,
            day: usize,
        ) -> Result<Vec<String>, String> {
            get_date_list(format!("{year}-{month}-{day}").as_str()).await
        }
    }

    impl DynamoClientTrait for DynamoDbClient {
        async fn put_collection_item(&self, collection: &CollectionItem) -> Result<bool, String> {
            let request = self
                .client
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

        async fn put_unzipping_item(
            &self,
            key_name: &str,
            time: Option<u128>,
        ) -> Result<bool, String> {
            let now = get_now(time)?;

            let request = self
                .client
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

        async fn put_unzipped_item(
            &self,
            key_name: &str,
            time: Option<u128>,
        ) -> Result<bool, String> {
            let now = get_now(time)?;

            let request = self
                .client
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

    async fn get_date_list(key: &str) -> Result<Vec<String>, String> {
        let request = dynamodb_client()
            .await
            .get_item()
            .table_name(table_name())
            .key("PK", AttributeValue::S(key.to_string()))
            .key("SK", AttributeValue::N("0".to_string()));

        let saved_date = match request.send().await {
            Ok(result) => match result.item {
                None => return Ok(Vec::new()),
                Some(item) => match item.get_key_value("SavedDate") {
                    None => return Err("Saved date is not found".to_string()),
                    Some(val) => match val.1.as_l() {
                        Ok(attribute) => attribute.to_owned(),
                        Err(_) => return Err("Casting to list is failed.".to_string()),
                    },
                },
            },
            Err(e) => return Err(e.to_string()),
        };

        let mut date = Vec::new();

        for attribute in saved_date {
            match attribute.as_s() {
                Ok(s) => date.push(s.to_owned()),
                Err(_) => return Err("Invalid date is stored".to_string()),
            }
        }
        Ok(date)
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
