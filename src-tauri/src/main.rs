#![feature(fs_try_exists)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use commands::{get_json, get_mod_file_info_json, get_mod_file_name_list, is_dir_exist, save_json};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_dir_exist,
            get_mod_file_name_list,
            get_mod_file_info_json,
            get_json,
            save_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
