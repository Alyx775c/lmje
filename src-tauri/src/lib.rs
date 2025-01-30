mod file_handler;
mod skill_data;

use file_handler::FileHandler;
use skill_data::Skill;
use std::collections::HashMap;
use std::{ffi::OsString, fs, path::PathBuf, sync::Mutex};
use tauri::Runtime;
use tauri_plugin_dialog::{DialogExt, FilePath};

static FH: Mutex<FileHandler> = Mutex::new(FileHandler::new());

#[tauri::command]
async fn file_dialog<R: Runtime>(app: tauri::AppHandle<R>) -> Result<FilePath, String> {
    let fol = app.dialog().file().blocking_pick_folder();
    if let Some(fol) = fol {
        FH.lock()
            .unwrap()
            .set_folder(PathBuf::from(fol.to_string()));
        Ok(fol)
    } else {
        Err("No folder selected".to_string())
    }
}

#[tauri::command]
async fn refresh_data<R: Runtime>(_app: tauri::AppHandle<R>) -> Result<String, String> {
    let fol = FH.lock().unwrap().get_folder();
    if let Some(fol) = fol {
        let skill_dir = fol.join("limbus_data\\skill");
        let skills = fs::read_dir(skill_dir.clone()).unwrap();
        let mut skill: String = String::from("");
        for sk in skills {
            let skill_path = sk.unwrap();
            let name = skill_path.file_name();
            if name == OsString::from("32.json") {
                skill = fs::read_to_string(skill_path.path()).unwrap();
                break;
            } else {
                continue;
            }
        }

        let skill_dat: HashMap<String, Vec<Skill>> =
            serde_json::from_str(&skill).unwrap();
        let skill: &Skill;

        if let Some(list) = skill_dat.get("list") {
            let first_skill = &list[0];
            skill = first_skill;
        } else {
            return Err("Skills map is empty".to_string());
        }

        FH.lock().unwrap().set_skill_folder(skill_dir);
        Ok(format!("First skill: {:?}", &skill))
    } else {
        Err("No folder selected".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), ()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![file_dialog, refresh_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
