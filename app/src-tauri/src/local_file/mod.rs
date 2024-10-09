mod local_file_dir;
pub mod local_file_error;

use crate::local_file::local_file_error::ExogenesisEnsembleLocalFileErrors;
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

/// copy the file from the requested path to the appropriate path
pub fn copy_file(origin: &Path, target: &Path) -> Result<(), ExogenesisEnsembleLocalFileErrors> {
    // if there is no requested directory, created
    if let Some(parents_path) = target.parent() {
        if !parents_path.exists() {
            if fs::create_dir_all(parents_path).is_err() {
                return Err(ExogenesisEnsembleLocalFileErrors::FileCopyFailed);
            }
        }
    }

    match fs::copy(origin, target) {
        Ok(_) => Ok(()),
        Err(_) => Err(ExogenesisEnsembleLocalFileErrors::FileCopyFailed),
    }
}

/// Check if the requested exists or not
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

    let Ok(meta_date) = fs::metadata(path) else {
        return Err(ExogenesisEnsembleLocalFileErrors::ReadingMetaDataError(
            "Reading a metadata is failed".to_string(),
        ));
    };

    let Ok(created_date_system_time) = meta_date.created() else {
        return Err(ExogenesisEnsembleLocalFileErrors::ReadingMetaDataError(
            "Reading the crated date time is failed".to_string(),
        ));
    };

    let Ok(created_date_time) = created_date_system_time.duration_since(UNIX_EPOCH) else {
        return Err(ExogenesisEnsembleLocalFileErrors::ReadingMetaDataError(
            "Invalid datetime is recorded".to_string(),
        ));
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

#[cfg(test)]
mod test_copy_file {
    use super::*;

    /// for test use
    fn remove_file(path: &Path) {
        if does_file_exist(path).is_ok() {
            fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn test_copy_file() {
        // Arrange
        let test_video_origin = Path::new("./test-data/video/IMG_0282.MOV");
        let target_path = Path::new("./test-data/1984/4/4/1984-4-4.MOV");

        remove_file(target_path); // remove file

        // Act
        let _ = copy_file(test_video_origin, target_path);

        // Assert
        assert!(does_file_exist(target_path).is_ok());

        remove_file(target_path); // remove file
    }
}
