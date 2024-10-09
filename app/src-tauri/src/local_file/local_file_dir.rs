use crate::local_file::local_file_error::ExogenesisEnsembleLocalFileErrors;
use chrono::{DateTime, Datelike, Timelike, Utc};
use directories::UserDirs;
use std::path::PathBuf;

const DIRECTORY_PATH: &str = "/ExogenesisEnsemblePartThreeRedemption";

/// Return the video path
/// The video directory is OS specific though, under that, a path will be an app name + /yyyy/MM/dd/yyyy-MM-dd-hh-mm-ss.{extension}
pub fn generate_video_file_dir(
    date_time: &DateTime<Utc>,
    extension: &str,
) -> Result<PathBuf, ExogenesisEnsembleLocalFileErrors> {
    let user_dirs = match UserDirs::new() {
        Some(dir) => dir,
        None => return Err(ExogenesisEnsembleLocalFileErrors::DirectoryMountFailed),
    };

    let video_dir = match user_dirs.video_dir() {
        Some(dir) => dir,
        None => return Err(ExogenesisEnsembleLocalFileErrors::DirectoryMountFailed),
    };

    let file_path = match generate_file_path(date_time, extension) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let joined_app_name = video_dir.join(format!("{}{}", DIRECTORY_PATH, file_path.as_str()));

    Ok(joined_app_name.into())
}

// A path is created by the date time, and it will be /yyyy/MM/dd/yyyy-MM-dd-hh-mm-ss.{extension}
fn generate_file_path(
    datetime: &DateTime<Utc>,
    extension: &str,
) -> Result<String, ExogenesisEnsembleLocalFileErrors> {
    if extension.len() < 1 {
        return Err(ExogenesisEnsembleLocalFileErrors::InvalidExtensionError);
    }

    let without_dot_extension = match extension.starts_with(".") {
        true => extension.split_at(1).1,
        false => extension,
    };

    Ok(format!(
        "/{}/{}/{}/{}-{}-{}-{}-{}-{}.{}",
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        without_dot_extension,
    ))
}

#[cfg(test)]
mod test_generate_file_path {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_generate_file_path() {
        // Assert
        let date_time = Utc.with_ymd_and_hms(1984, 4, 4, 12, 42, 42).unwrap();
        let extension = ".VIDEO";

        // Act
        let result = generate_file_path(&date_time, extension);

        // Assert
        let expected_date_time = format!(
            "/{}/{}/{}/{}-{}-{}-{}-{}-{}.{}",
            1984, 4, 4, 1984, 4, 4, 12, 42, 42, "VIDEO"
        );
        assert_eq!(result.unwrap(), expected_date_time);
    }
}
