/// select file by user. The path is passed to this function.
#[tauri::command]
pub fn select_file(path: &str) -> String {
    return String::from(path);
}
