mod file_handler;

use std::{path::PathBuf, sync::Mutex};
use file_handler::FileHandler;
use tauri::Runtime;
use tauri_plugin_dialog::{DialogExt, FilePath};

static FH: Mutex<FileHandler> = Mutex::new(FileHandler::new());

#[tauri::command]
async fn file_dialog<R: Runtime>(app: tauri::AppHandle<R>) -> Result<FilePath, String> {
    let fol = app.dialog().file().blocking_pick_folder();
    if let Some(fol) = fol {
        FH.lock().unwrap().set_folder(PathBuf::from(fol.to_string()));
        Ok(fol)
    } else {
        Err("No folder selected".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![file_dialog])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
