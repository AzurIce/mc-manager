use std::fs;

#[tauri::command]
pub fn is_dir_exist(path: &str) -> bool {
    return fs::try_exists(path).unwrap_or(false);
}