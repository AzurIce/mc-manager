use std::{fs::{self, File}, io::{Read, BufReader, Write}, path::Path};
use json::object;

#[tauri::command]
pub fn get_json(path: &str) -> String {
    if let Ok(mut file) = File::open(path) {
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Read json failed");
        return buf;
    } else {
        return String::from("{}");
    }
}

#[tauri::command]
pub fn save_json(path: &str, json: String) {
    let mut file = File::create(path).expect("Create json failed");

    file.write(json.as_bytes()).expect("Write json failed");
}

#[tauri::command]
pub fn is_dir_exist(path: &str) -> bool {
    return fs::try_exists(path).unwrap_or(false);
}

#[tauri::command]
pub fn get_mod_file_name_list(path: &str) -> Vec<String> {
    let mut mod_list: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            let file_path = entry.expect("Entry err").path();
            if let Some(ext) = file_path.extension() && ext == "jar" {
                let file_name = String::from(
                    file_path.file_name().unwrap().to_str().unwrap()
                );
                // println!("{:?}", file_path);
                mod_list.push(file_name);
            }
        }
    }
    return mod_list;
}

// Return ModFileInfo in json
#[tauri::command]
pub fn get_mod_file_info_json(path: &str) -> String {
    let file_path = Path::new(path);
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut archive = zip::ZipArchive::new(reader).unwrap();

    let mut mod_file_info = object!{
        filename: String::from(file_path.file_name().unwrap().to_str().unwrap()),
        is_fabric_mod: false,
        mod_id: String::from(""),
        mod_name: String::from(""),
        mod_version: String::from(""),
        game_version: String::from(""),
        is_bad_json_syntax: false,
    };

    if let Ok(mut fabric_mod_json) = archive.by_name("fabric.mod.json") {
        let mut buf = String::new();
        fabric_mod_json
            .read_to_string(&mut buf)
            .expect("Read fabric.mod.json failed");
        // println!("{:?}", buf);
        if let Ok(fabric_mod_json) = json::parse(&mut buf) {
            mod_file_info = object!{
                filename: String::from(file_path.file_name().unwrap().to_str().unwrap()),
                is_fabric_mod: true,
                mod_id: fabric_mod_json["id"].to_string(),
                mod_name: fabric_mod_json["name"].to_string(),
                mod_version: fabric_mod_json["version"].to_string(),
                game_version: fabric_mod_json["depends"]["minecraft"].to_string(),
                is_bad_json_syntax: false,
            };
        } else {
            mod_file_info = object!{
                filename: String::from(file_path.file_name().unwrap().to_str().unwrap()),
                is_fabric_mod: true,
                mod_id: String::from(""),
                mod_name: String::from(""),
                mod_version: String::from(""),
                game_version: String::from(""),
                is_bad_json_syntax: true,
            };
        }


        // println!("{:#?}", fabric_mod_json);
    }

    return json::stringify(mod_file_info);
}