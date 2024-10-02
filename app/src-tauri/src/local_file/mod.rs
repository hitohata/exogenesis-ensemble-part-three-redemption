// pub fn move_file(path: &str) -> Result<(), std::io::Error> {
//
// }

/// Extract a modified datetime from the meta-data of the video.
fn extract_modified_datetime_form_video(path: &str) {
    let path = std::env::current_dir().unwrap();
    println!("{:?}", path);
    // let meta_date = fs::metadata(path);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {
        assert!(true);
    }
}
