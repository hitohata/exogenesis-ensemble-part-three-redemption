use crate::environment_values::clients::test_dynamo_client;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};

/// create a new dynamodb table named by the argument
/// if that table already exists, this function delete it before creating a table..
pub async fn create_table(table_name: &str) {
    match table_existence(table_name).await {
        true => {
            delete_table(table_name).await;
            _create_table(table_name).await;
        }
        false => _create_table(table_name).await,
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

    test_dynamo_client()
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

async fn delete_table(table_name: &str) {
    test_dynamo_client()
        .await
        .delete_table()
        .table_name(table_name)
        .send()
        .await
        .expect("delete table failed");
}

/// Check if the table exists or not by describing the table.
/// If there is no table, the `describe_table` returns Err.
/// It can be considered as there is no table thus this function returns false.
async fn table_existence(table_name: &str) -> bool {
    let result = test_dynamo_client()
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
    use crate::dynamodb::test_util::create_table;
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
        create_table(table_name).await;

        // Act
        let result = table_existence(table_name).await;

        // Assert
        assert!(result);
    }
}
