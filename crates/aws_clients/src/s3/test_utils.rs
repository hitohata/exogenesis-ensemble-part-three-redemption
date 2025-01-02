use crate::s3::environment_value::{s3_client, standard_bucked_name};

pub async fn put_test_object(key_name: &str) {
    let path = || {
        let target_path = std::path::Path::new("/data/test.MOV");
        if target_path.exists() {
            return target_path;
        }

        let target_path = std::path::Path::new("./storage/test_data/test.MOV");
        if target_path.exists() {
            return target_path;
        }

        panic!("dummy object path is not found");
    };

    let body = aws_sdk_s3::primitives::ByteStream::from_path(path())
        .await
        .unwrap();

    let _ = s3_client()
        .await
        .put_object()
        .bucket(standard_bucked_name())
        .key(key_name)
        .body(body)
        .send()
        .await;
}
