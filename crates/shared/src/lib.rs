//! This is a shared crate.
//! This crate contains general things that are used in the whole system

pub mod traits {
    use std::future::Future;

    /// The searching is shared in the DB and bucket
    /// This trait defines basic access patterns
    #[cfg_attr(feature = "mock", mockall::automock)]
    pub trait GetFileListTrait {
        /// get years list
        fn get_years(&self) -> impl Future<Output = Result<Vec<String>, String>> + Send;
        /// get months list
        fn get_months(
            &self,
            years: usize,
        ) -> impl Future<Output = Result<Vec<String>, String>> + Send;
        /// get days list
        fn get_days(
            &self,
            year: usize,
            month: usize,
        ) -> impl Future<Output = Result<Vec<String>, String>> + Send;
        /// get objects list
        fn get_objects(
            &self,
            year: usize,
            month: usize,
            day: usize,
        ) -> impl Future<Output = Result<Vec<String>, String>> + Send;
    }
}
