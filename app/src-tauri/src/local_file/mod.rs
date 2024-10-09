mod file_handling;
mod local_file_dir;
pub mod local_file_error;
pub(crate) mod util;

use crate::local_file::file_handling::{copy_file, FileInformation};
use crate::local_file::local_file_error::ExogenesisEnsembleLocalFileErrors;
use std::path::Path;

/// This function takes a selected file's path.
/// Read the meta-data of the file and copy it to appropriate directory
/// If the process is success, return the path
pub fn assign_file(path: &str) -> Result<String, ExogenesisEnsembleLocalFileErrors> {
    let file_information = FileInformation::new(&path)?;

    let origin_path = Path::new(&path);
    let target_path = Path::new(file_information.file_path());

    copy_file(origin_path, target_path)?;

    Ok(file_information.file_path().to_string())
}
