mod file_handling;
mod local_file_dir;
pub mod local_file_error;
pub(crate) mod util;

use crate::local_file::file_handling::{copy_file, generate_file_path};
use crate::local_file::local_file_error::ExogenesisEnsembleLocalFileErrors;
use std::path::Path;

/// This function takes a selected file's path.
/// Read the meta-data of the file and copy it to appropriate directory
/// If the process is success, return the path
pub fn assign_file(path: &str) -> Result<String, ExogenesisEnsembleLocalFileErrors> {
    let target_file_path = generate_file_path(&path)?;

    let origin_path = Path::new(&path);
    let target_path = Path::new(target_file_path.as_str());

    copy_file(origin_path, target_path)?;

    Ok(target_file_path)
}
