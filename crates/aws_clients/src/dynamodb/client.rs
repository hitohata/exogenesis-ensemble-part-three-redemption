use crate::dynamodb::entities::collection::CollectionItem;
#[cfg(not(test))]
use crate::dynamodb::environment_values::{dynamodb_client, table_name};
use aws_sdk_dynamodb::types::AttributeValue;
use shared::traits::GetFileListTrait;
use std::future::Future;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct DynamoDbClient<'a> {
    pub(crate) client: &'static aws_sdk_dynamodb::Client,
    pub(crate) table_name: &'a str,
}

impl DynamoDbClient<'_> {
    #[allow(dead_code)]
    #[cfg(not(test))]
    pub async fn new() -> Self {
        Self {
            client: dynamodb_client().await,
            table_name: table_name(),
        }
    }
}

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait DynamoClientTrait {
    /// put a new collection item
    #[allow(dead_code)]
    fn put_collection_items(
        &self,
        collection: &Vec<CollectionItem>,
    ) -> impl Future<Output = Result<(), String>> + Send;
    // put zipping time
    /// time is mill sec
    #[allow(dead_code)]
    fn put_unzipping_item(
        &self,
        key_name: &str,
        time: Option<u128>,
    ) -> impl Future<Output = Result<(), String>> + Send;
    /// put unzipped item
    /// time is mill sec
    #[allow(dead_code)]
    fn put_unzipped_item(
        &self,
        key_name: &str,
        time: Option<u128>,
    ) -> impl Future<Output = Result<(), String>> + Send;
}

impl GetFileListTrait for DynamoDbClient<'_> {
    async fn get_years(&self) -> Result<Vec<String>, String> {
        let mut years = self.get_date_list("root").await?;
        years.sort();
        Ok(years)
    }

    async fn get_months(&self, year: usize) -> Result<Vec<String>, String> {
        let mut months = self.get_date_list(format!("{year}").as_str()).await?;
        months.sort();
        Ok(months)
    }

    async fn get_days(&self, year: usize, month: usize) -> Result<Vec<String>, String> {
        let mut days = self
            .get_date_list(format!("{year}-{month}").as_str())
            .await?;
        days.sort();
        Ok(days)
    }

    async fn get_objects(
        &self,
        year: usize,
        month: usize,
        day: usize,
    ) -> Result<Vec<String>, String> {
        self.get_date_list(format!("{year}-{month}-{day}").as_str())
            .await
    }
}

impl crate::dynamodb::client::DynamoClientTrait for DynamoDbClient<'_> {
    async fn put_collection_items(&self, collection: &Vec<CollectionItem>) -> Result<(), String> {
        let update_collections = collection
            .iter()
            .map(|collection| async { self.put_collection_item(collection).await });

        let _ = tokio::try_join!(self.put_years(collection))?;

        let results = futures::future::join_all(update_collections).await;

        for result in results {
            result?;
        }

        Ok(())
    }

    async fn put_unzipping_item(&self, key_name: &str, time: Option<u128>) -> Result<(), String> {
        let now = get_now(time)?;

        let request = self
            .client
            .put_item()
            .table_name(self.table_name)
            .item("PK", AttributeValue::S("Unzipping".to_string()))
            .item("SK", AttributeValue::N(now.to_string()))
            .item("KeyName", AttributeValue::S(key_name.to_string()));

        if let Err(e) = request.send().await {
            return Err(e.to_string());
        }

        Ok(())
    }

    async fn put_unzipped_item(&self, key_name: &str, time: Option<u128>) -> Result<(), String> {
        let now = get_now(time)?;

        let request = self
            .client
            .put_item()
            .table_name(self.table_name)
            .item("PK", AttributeValue::S("Unzipped".to_string()))
            .item("SK", AttributeValue::N(now.to_string()))
            .item("KeyName", AttributeValue::S(key_name.to_string()));

        if let Err(e) = request.send().await {
            return Err(e.to_string());
        }

        Ok(())
    }
}

impl DynamoDbClient<'_> {
    /// get date
    /// doc<https://hitohata.github.io/ExogenesisEnsemble-Part3-Redemption/project/docs/technical-information/DynamoDB-Definition#date-lookup>
    async fn get_date_list(&self, key: &str) -> Result<Vec<String>, String> {
        let request = self
            .client
            .get_item()
            .table_name(self.table_name)
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

    /// put a collection item
    async fn put_collection_item(&self, collection: &CollectionItem) -> Result<(), String> {
        let request = self
            .client
            .put_item()
            .table_name(self.table_name)
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

        Ok(())
    }

    /// update years lookup
    async fn put_years(&self, collections: &Vec<CollectionItem>) -> Result<(), String> {
        let recorded_years = self.get_years().await?;

        let collections_years = collections
            .iter()
            .map(|el| el.year.to_string())
            .collect::<Vec<String>>();

        let mut years = vec![&recorded_years[..], &collections_years[..]].concat();
        years.sort_unstable();
        years.dedup();

        // if the new vector contains new collections
        if years.len() == recorded_years.len() {
            return Ok(());
        }

        let result = self
            .client
            .put_item()
            .table_name(self.table_name)
            .item("PK", AttributeValue::S("root".to_string()))
            .item("SK", AttributeValue::N("0".to_string()))
            .item(
                "SavedDate",
                AttributeValue::L(
                    years
                        .iter()
                        .map(|el| AttributeValue::S(el.to_string()))
                        .collect(),
                ),
            );

        let result = result.send().await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
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

#[cfg(test)]
mod get_file_list_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_years() {
        // Arrange
        let table_name = "test_get_years";
        let client = DynamoDbClient::new(table_name).await;
        save_test_data(table_name).await;

        // Act
        let result = client.get_years().await.unwrap();

        // Assert
        assert_eq!(result, ["1984", "1985"]);
    }

    async fn save_test_data(table_name: &str) {
        let init_data = vec![
            "1984/04/04/1984-04-04-12-34-50.MOV",
            "1984/04/04/1984-04-04-12-34-51.MOV",
            "1984/04/05/1984-04-05-12-34-50.MOV",
            "1984/05/04/1984-05-04-12-34-50.MOV",
            "1985/04/04/1985-04-04-12-34-50.MOV",
        ];

        let init_collections: Vec<CollectionItem> = init_data
            .iter()
            .map(|data| CollectionItem::dummy_object(data))
            .collect();

        let client: DynamoDbClient = DynamoDbClient::new(table_name).await;

        client
            .put_collection_items(&init_collections)
            .await
            .unwrap();
    }
}
