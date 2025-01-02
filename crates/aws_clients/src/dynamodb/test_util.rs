use crate::dynamodb::client::{DynamoClientTrait, DynamoDbClient};
use crate::dynamodb::entities::collection::CollectionItem;
use crate::dynamodb::environment_values::dynamodb_client;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};

impl<'a> DynamoDbClient<'a> {
    /// when this function is called, the new table, which is provided by argument, will be created.
    #[cfg(test)]
    pub async fn new(table_name: &'a str) -> Self {
        let client = Self {
            client: dynamodb_client().await,
            table_name,
        };

        client.create_table().await;

        client
    }

    /// create a new dynamodb table named by the argument
    /// if that table already exists, this function delete it before creating a table..
    pub async fn create_table(&self) {
        match table_existence(self.table_name).await {
            true => {
                self.delete_table(self.table_name).await;
                _create_table(self.table_name).await;
            }
            false => _create_table(self.table_name).await,
        }
    }

    async fn delete_table(&self, table_name: &str) {
        self.client
            .delete_table()
            .table_name(table_name)
            .send()
            .await
            .expect("delete table failed");
    }

    /// add dummy data
    /// the key must be the file name
    async fn add_dummy_data(&self, key_name: &str) -> Result<(), String> {
        let collection = CollectionItem::dummy_object(key_name);
        self.put_collection_item(&collection).await?;
        Ok(())
    }
}

async fn _create_table(table_name: &str) {
    let pk_ad = AttributeDefinition::builder()
        .attribute_name("PK")
        .attribute_type(ScalarAttributeType::S)
        .build()
        .expect("pk attribute error");

    let pk_ks = KeySchemaElement::builder()
        .attribute_name("PK")
        .key_type(KeyType::Hash)
        .build()
        .expect("pk key error");

    let sk_ad = AttributeDefinition::builder()
        .attribute_name("SK")
        .attribute_type(ScalarAttributeType::N)
        .build()
        .expect("sort attribute error");

    let sk_ks = KeySchemaElement::builder()
        .attribute_name("SK")
        .key_type(KeyType::Range)
        .build()
        .expect("sort key error");

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(10)
        .build()
        .unwrap();

    dynamodb_client()
        .await
        .create_table()
        .table_name(table_name)
        .key_schema(pk_ks)
        .key_schema(sk_ks)
        .attribute_definitions(pk_ad)
        .attribute_definitions(sk_ad)
        .set_provisioned_throughput(Some(pt))
        .send()
        .await
        .expect("couldn't create a table");
}

/// Check if the table exists or not by describing the table.
/// If there is no table, the `describe_table` returns Err.
/// It can be considered as there is no table thus this function returns false.
async fn table_existence(table_name: &str) -> bool {
    let result = dynamodb_client()
        .await
        .describe_table()
        .table_name(table_name)
        .send()
        .await;

    match result {
        Ok(table) => match table.table {
            Some(_) => true,
            None => false,
        },
        Err(_) => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dynamodb::test_util::DynamoDbClient;
    use tokio;

    #[tokio::test]
    async fn test_table_existence() {
        // Act
        let result = table_existence("non-existing-table").await;

        // Assert
        assert_eq!(result, false);
    }

    #[tokio::test]
    async fn test_create_table() {
        // Arrange
        let table_name = "new-table";
        let _ = DynamoDbClient::new(table_name).await;

        // Act
        let result = table_existence(table_name).await;

        // Assert
        assert!(result);
    }

    #[tokio::test]
    async fn test_add_dummy_data() {
        // Arrange
        let client = DynamoDbClient::new("add_dummy_data").await;
        let result = client
            .add_dummy_data("1984/04/04/1984-04-04-12-34-50.MOV")
            .await;

        // Assert
        assert!(result.is_ok());
    }
}
