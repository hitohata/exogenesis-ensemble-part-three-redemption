pub mod client {
    use shared::traits::GetFileListTrait;

    pub struct DynamoDbClient {}

    pub trait DynamoClientTrait: GetFileListTrait {}

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

    impl DynamoClientTrait for DynamoDbClient {}
}


#[cfg(test)]
mod test_util;

#[cfg(test)]
mod test {
    use super::*;
    use test_util::create_table;
    use tokio;

    #[tokio::test]
    async fn test() {
        create_table("new-table").await;
        assert!(true)
    }
}
