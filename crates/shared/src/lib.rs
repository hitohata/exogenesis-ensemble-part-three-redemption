pub mod traits {
    use std::future::Future;

    /// The searching is shared in the DB and bucket
    /// This trait defines basic access patterns
    pub trait GetFileListTrait {
        /// get years list
        fn get_years() -> impl Future<Output = Result<Vec<String>, String>> + Send;
        /// get months list
        fn get_month(years: usize) -> impl Future<Output = Result<Vec<String>, String>> + Send;
        /// get days list
        fn get_days(
            year: usize,
            month: usize,
        ) -> impl Future<Output = Result<Vec<String>, String>> + Send;
        /// get objects list
        fn get_objects(
            year: usize,
            month: usize,
            day: usize,
        ) -> impl Future<Output = Result<Vec<String>, String>> + Send;
    }
}
