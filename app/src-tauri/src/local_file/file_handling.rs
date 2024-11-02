use crate::local_file::local_file_dir::generate_video_file_dir;
use crate::local_file::local_file_error::ExogenesisEnsembleLocalFileErrors;
use crate::local_file::util::does_file_exist;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

pub(crate) fn generate_file_path(
    target_path: &str,
) -> Result<String, ExogenesisEnsembleLocalFileErrors> {
    let path = Path::new(target_path);
    let _ = does_file_exist(path)?;

    let extension = get_extension(path)?;

    let created_date_time = extract_created_datetime_form_video(path)?;
    let video_dir = generate_video_file_dir(created_date_time, &extension)?;

    let Some(video_dir_str) = video_dir.to_str() else {
        return Err(ExogenesisEnsembleLocalFileErrors::FileError(
            "Directory name change failed".to_string(),
        ));
    };

    Ok(video_dir_str.to_string())
}

/// copy the file from the requested path to the appropriate path
pub(crate) fn copy_file(
    origin: &Path,
    target: &Path,
) -> Result<(), ExogenesisEnsembleLocalFileErrors> {
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

fn get_extension(path: &Path) -> Result<String, ExogenesisEnsembleLocalFileErrors> {
    match path.extension() {
        Some(ex) => match ex.to_str() {
            Some(extension) => Ok(extension.to_owned()),
            None => Err(ExogenesisEnsembleLocalFileErrors::ExtensionParseFailed),
        },
        None => Err(ExogenesisEnsembleLocalFileErrors::ExtensionParseFailed),
    }
}

/// Extract a created datetime from the meta-data of the video.
/// The return date will be epoch time.
fn extract_created_datetime_form_video(
    path: &Path,
) -> Result<u128, ExogenesisEnsembleLocalFileErrors> {
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

    match created_date_system_time.duration_since(UNIX_EPOCH) {
        Ok(duration) => Ok(duration.as_millis()),
        Err(_) => Err(ExogenesisEnsembleLocalFileErrors::ReadingMetaDataError(
            "Invalid datetime is recorded".to_string(),
        )),
    }
}

#[cfg(test)]
mod file_existence_test {
    use crate::local_file::util::does_file_exist;
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
mod test_extension {
    use super::*;

    #[test]
    fn test_get_extension() {
        // Arrange
        let target_file_path = Path::new("./test-data/video/IMG_0282.MOV");

        // Act
        let result = get_extension(target_file_path).unwrap();

        // Assert
        assert_eq!(result, "MOV".to_string());
    }
}

#[cfg(test)]
mod test_copy_file {
    use super::*;
    use crate::local_file::util::does_file_exist;

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
