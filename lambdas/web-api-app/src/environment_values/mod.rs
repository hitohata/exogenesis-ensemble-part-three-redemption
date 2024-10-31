/// The lambda's environment values
pub mod lambda {
    use std::env::var;
    use std::sync::OnceLock;

    /// The standard bucket name.
    pub fn standard_bucked_name() -> &'static str {
        static STANDARD_BUCKET: OnceLock<String> = OnceLock::new();
        STANDARD_BUCKET.get_or_init(|| var("STANDARD_BUCKET_NAME").unwrap())
    }
}