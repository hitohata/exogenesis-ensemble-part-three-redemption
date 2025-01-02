//! Entities

/// Collection
/// <https://hitohata.github.io/ExogenesisEnsemble-Part3-Redemption/project/docs/technical-information/DynamoDB-Definition/#collection>
pub mod collection {
    use time_file_name::file_datetime::PathDateTime;

    pub struct CollectionItem {
        pub year: String,
        pub unix_time: i64,
        pub is_unzipped: bool,
        pub vault: String,
        /// This is a S3 bucket name
        pub key_name: String,
    }

    impl CollectionItem {
        /// create a new item
        pub fn new_object(key_name: &str, vault: &str) -> Result<Self, String> {
            let path_date_time = PathDateTime::parse(key_name)?;

            Ok(CollectionItem {
                year: path_date_time.year.to_string(),
                unix_time: path_date_time.unix_time,
                is_unzipped: false,
                vault: vault.to_string(),
                key_name: key_name.to_string(),
            })
        }
    }

    #[cfg(test)]
    impl CollectionItem {
        /// dummy data from datetime
        /// the vault is just a string, "vault".
        pub fn dummy_object(key_name: &str) -> CollectionItem {
            CollectionItem::new_object(&key_name, "vault").unwrap()
        }
    }
}
