#![feature(fs_try_exists)]
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;

use commands::is_dir_exist;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![is_dir_exist])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
