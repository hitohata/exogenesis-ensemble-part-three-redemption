mod local_file_dir;
mod local_file_error;

use crate::local_file::local_file_error::ExogenesisEnsembleLocalFileErrors;
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

// pub fn move_file(origin: &Path, target: &Path) -> Result<(), std::io::Error> {
//
// }

/// Check if the requested if exists or not
fn does_file_exist(path: &Path) -> Result<(), ExogenesisEnsembleLocalFileErrors> {
    if path.exists() {
        Ok(())
    } else {
        Err(ExogenesisEnsembleLocalFileErrors::FileNotFoundError(
            format!("Required Path does not exists: {:?}", path.as_os_str()),
        ))
    }
}

/// Extract a modified datetime from the meta-data of the video.
fn extract_modified_datetime_form_video(
    path: &str,
) -> Result<DateTime<Utc>, ExogenesisEnsembleLocalFileErrors> {
    let path = Path::new(path);

    does_file_exist(path)?;

    let meta_date = match fs::metadata(path) {
        Ok(meta) => meta,
        Err(_) => {
            return Err(ExogenesisEnsembleLocalFileErrors::ReadingMetaDataError(
                "Reading a metadata is failed".to_string(),
            ))
        }
    };

    let created_date_system_time = match meta_date.created() {
        Ok(created) => created,
        Err(_) => {
            return Err(ExogenesisEnsembleLocalFileErrors::ReadingMetaDataError(
                "Reading the crated date time is failed".to_string(),
            ))
        }
    };

    let created_date_time = match created_date_system_time.duration_since(UNIX_EPOCH) {
        Ok(created) => created,
        Err(_) => {
            return Err(ExogenesisEnsembleLocalFileErrors::ReadingMetaDataError(
                "Invalid datetime is recorded".to_string(),
            ))
        }
    };

    match chrono::DateTime::from_timestamp_micros(created_date_time.as_micros() as i64) {
        Some(datetime) => Ok(datetime),
        None => Err(ExogenesisEnsembleLocalFileErrors::InvalidDateError(
            format!("{}", created_date_time.as_micros()),
        )),
    }
}

#[cfg(test)]
mod file_existence_test {
    use crate::local_file::does_file_exist;
    use std::path::Path;

    #[test]
    fn file_found() {
        // Arrange
        let path = Path::new("./test-data/video/IMG_0282.MOV");

        // Act
        let result = does_file_exist(&path);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn file_not_found() {
        // Arrange
        let path = Path::new("./this/is/dummy/path.txt");

        // Act
        let result = does_file_exist(&path);

        // Assert
        assert!(result.is_err());
    }
}

// #[cfg(test)]
// mod
