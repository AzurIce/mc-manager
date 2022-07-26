use std::fs;

#[tauri::command]
pub fn is_dir_exist(path: &str) -> bool {
    return fs::try_exists(path).unwrap_or(false);
}

#[tauri::command]
pub fn get_mod_list(path: &str) -> Vec<String> {
    let mut mod_list: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            let file_path = entry.expect("Entry err").path();
            if let Some(ext) = file_path.extension() && ext == "jar" {
                let file_name = String::from(
                    file_path.file_name().unwrap().to_str().expect("?")
                );
                println!("{:?}", file_name);
                mod_list.push(file_name);
            }
            // if let Ok(file_type) = entry.file_type() {
            //     println!("{:?}", file_type)
            // }
            // println!("{:?}", entry)
        }
    }
    return mod_list;
}