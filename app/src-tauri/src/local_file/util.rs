use crate::local_file::local_file_error::ExogenesisEnsembleLocalFileErrors;
use std::path::Path;

/// Check if the requested exists or not
pub fn does_file_exist(path: &Path) -> Result<(), ExogenesisEnsembleLocalFileErrors> {
    if path.exists() {
        Ok(())
    } else {
        Err(ExogenesisEnsembleLocalFileErrors::FileNotFoundError(
            format!("Required Path does not exists: {:?}", path.as_os_str()),
        ))
    }
}
