use crate::dynamodb::entities::collection::{CollectionItem, LookUpItems};
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
    async fn put_collection_items(&self, collections: &Vec<CollectionItem>) -> Result<(), String> {
        let look_up_items = LookUpItems::new(collections)?;

        // years
        self.put_years(&look_up_items.years).await?;

        // month
        let month_results = futures::future::join_all(
            look_up_items
                .months
                .iter()
                .map(|month| self.put_months(month.0, month.1.as_ref())),
        )
        .await;

        for result in month_results {
            result?;
        }

        // days
        let day_results = futures::future::join_all(
            look_up_items
                .days
                .iter()
                .map(|day| self.put_days(day.0, day.1, day.2.as_ref())),
        )
        .await;

        for result in day_results {
            result?;
        }

        // objects
        let object_results = futures::future::join_all(
            look_up_items
                .objects
                .iter()
                .map(|day| self.put_objects(day.0, day.1, day.2, day.3.as_ref())),
        )
        .await;

        for result in object_results {
            result?;
        }

        let update_collections = collections
            .iter()
            .map(|collection| async { self.put_collection_item(collection).await });

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
    async fn put_years(&self, years: &Vec<String>) -> Result<(), String> {
        let recorded_years = self.get_years().await?;

        let mut concat_years = vec![&recorded_years[..], &years[..]].concat();
        concat_years.sort_unstable();
        concat_years.dedup();

        // if the new vector contains new collections
        if concat_years.len() == recorded_years.len() {
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
                    concat_years
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

    async fn put_months(&self, years: usize, months: &Vec<String>) -> Result<(), String> {
        let recorded_months = self.get_months(years).await?;

        let mut concat_months = vec![&recorded_months[..], &months[..]].concat();
        concat_months.sort_unstable();
        concat_months.dedup();

        if concat_months.len() == recorded_months.len() {
            return Ok(());
        }
        let result = self
            .client
            .put_item()
            .table_name(self.table_name)
            .item("PK", AttributeValue::S(format!("{years}")))
            .item("SK", AttributeValue::N("0".to_string()))
            .item(
                "SavedDate",
                AttributeValue::L(
                    concat_months
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

    async fn put_days(&self, years: usize, month: usize, days: &Vec<String>) -> Result<(), String> {
        let recorded_days = self.get_days(years, month).await?;

        let mut concat_days = vec![&recorded_days[..], &days[..]].concat();
        concat_days.sort_unstable();
        concat_days.dedup();

        if concat_days.len() == recorded_days.len() {
            return Ok(());
        }
        let result = self
            .client
            .put_item()
            .table_name(self.table_name)
            .item("PK", AttributeValue::S(format!("{years}-{month}")))
            .item("SK", AttributeValue::N("0".to_string()))
            .item(
                "SavedDate",
                AttributeValue::L(
                    concat_days
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

    async fn put_objects(
        &self,
        years: usize,
        month: usize,
        day: usize,
        objects: &Vec<String>,
    ) -> Result<(), String> {
        let recorded_months = self.get_objects(years, month, day).await?;

        let mut concat_objects = vec![&recorded_months[..], &objects[..]].concat();
        concat_objects.sort_unstable();
        concat_objects.dedup();

        if concat_objects.len() == recorded_months.len() {
            return Ok(());
        }

        let result = self
            .client
            .put_item()
            .table_name(self.table_name)
            .item("PK", AttributeValue::S(format!("{years}-{month}-{day}")))
            .item("SK", AttributeValue::N("0".to_string()))
            .item(
                "SavedDate",
                AttributeValue::L(
                    concat_objects
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

    #[tokio::test]
    async fn test_get_years_with_no_data() {
        // Arrange
        let table_name = "test_get_years_with_no_data";
        let client = DynamoDbClient::new(table_name).await;

        // Act
        let result = client.get_years().await.unwrap();

        // Assert
        assert_eq!(result, Vec::<String>::new());
    }

    #[tokio::test]
    async fn test_get_months() {
        // Arrange
        let table_name = "test_get_months";
        let client = DynamoDbClient::new(table_name).await;
        save_test_data(table_name).await;

        // Act
        let result = client.get_months(1984).await.unwrap();

        // Assert
        assert_eq!(result, ["4", "5"]);
    }

    #[tokio::test]
    async fn test_get_months_with_no_data() {
        // Arrange
        let table_name = "test_get_months_with_no_data";
        let client = DynamoDbClient::new(table_name).await;

        // Act
        let result = client.get_months(1984).await.unwrap();

        // Assert
        assert_eq!(result, Vec::<String>::new());
    }

    #[tokio::test]
    async fn test_get_objects() {
        // Arrange
        let table_name = "test_get_objects";
        let client = DynamoDbClient::new(table_name).await;
        save_test_data(table_name).await;

        // Act
        let (result_1984_4_4, result_1984_4_5, result_1984_5_4, result_1985_4_4) = tokio::join!(
            client.get_objects(1984, 4, 4),
            client.get_objects(1984, 4, 5),
            client.get_objects(1984, 5, 4),
            client.get_objects(1985, 4, 4),
        );

        // Assert
        assert_eq!(
            result_1984_4_4.unwrap(),
            [
                "1984/04/04/1984-04-04-12-34-50.MOV",
                "1984/04/04/1984-04-04-12-34-51.MOV"
            ]
        );
        assert_eq!(
            result_1984_4_5.unwrap(),
            ["1984/04/05/1984-04-05-12-34-50.MOV"]
        );
        assert_eq!(
            result_1984_5_4.unwrap(),
            ["1984/05/04/1984-05-04-12-34-50.MOV"]
        );
        assert_eq!(
            result_1985_4_4.unwrap(),
            ["1985/04/04/1985-04-04-12-34-50.MOV"]
        );
    }

    #[tokio::test]
    async fn test_get_objects_with_no_data() {
        // Arrange
        let table_name = "test_get_objects_with_no_data";
        let client = DynamoDbClient::new(table_name).await;

        // Act
        let result = client.get_months(1984).await.unwrap();

        // Assert
        assert_eq!(result, Vec::<String>::new());
    }

    #[tokio::test]
    async fn test_get_days() {
        // Arrange
        let table_name = "test_get_days";
        let client = DynamoDbClient::new(table_name).await;
        save_test_data(table_name).await;

        // Act
        let (result_1984_4, result_1984_5, result_1985_4) = tokio::join!(
            client.get_days(1984, 4),
            client.get_days(1984, 5),
            client.get_days(1985, 4)
        );

        // Assert
        assert_eq!(result_1984_4.unwrap(), ["4", "5"]);
        assert_eq!(result_1984_5.unwrap(), ["4"]);
        assert_eq!(result_1985_4.unwrap(), ["4"]);
    }

    #[tokio::test]
    async fn test_get_days_with_no_data() {
        // Arrange
        let table_name = "test_get_days_with_no_data";
        let client = DynamoDbClient::new(table_name).await;

        // Act
        let result = client.get_days(1984, 4).await.unwrap();

        // Assert
        assert_eq!(result, Vec::<String>::new());
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
