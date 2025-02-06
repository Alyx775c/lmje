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
async fn refresh_data<R: Runtime>(
    _app: tauri::AppHandle<R>,
) -> Result<std::option::Option<Vec<Skill>>, String> {
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

        let skill_dat_res: Result<HashMap<String, Vec<Skill>>, serde_json::Error> =
            serde_json::from_str(&skill);

        if let Err(e) = skill_dat_res {
            println!(
                "Error, line: {}, col: {}, type: {:#?}, data: {:#?}",
                e.line(),
                e.column(),
                e.classify(),
                e.to_string()
            );
            return Err(e.to_string());
        }

        // this can be safely unwrapped because we know the result is Ok
        let skill_dat = skill_dat_res.unwrap();

        FH.lock().unwrap().set_skill_folder(skill_dir);
        Ok(skill_dat.get("list").cloned())
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
