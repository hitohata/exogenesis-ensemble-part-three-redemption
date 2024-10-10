use crate::local_file::assign_file;

/// select file by user. The path is passed to this function.
#[tauri::command]
pub fn select_file(path: &str) -> String {
    let file_name = match assign_file(path) {
        Ok(file) => file,
        Err(error) => return error.to_string(),
    };

    file_name
}
