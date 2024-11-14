use crate::local_file::local_file_error::ExogenesisEnsembleLocalFileErrors;
use directories::UserDirs;
use std::path::PathBuf;
use time_file_name::file_path::FilePath;

const DIRECTORY_PATH: &str = "ExogenesisEnsemblePartThreeRedemption";

/// Return the video path
/// The video directory is OS specific though, under that, a path will be an app name + /yyyy/MM/dd/yyyy-MM-dd-hh-mm-ss.{extension}
pub fn generate_video_file_dir(
    date_time: u128,
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

    let file_path = match FilePath::new().generate_file_path(date_time, extension) {
        Ok(path) => path,
        Err(e) => return Err(ExogenesisEnsembleLocalFileErrors::FileError(e)),
    };

    let joined_app_name = video_dir.join(format!("{}{}", DIRECTORY_PATH, file_path.as_str()));

    Ok(joined_app_name.into())
}
