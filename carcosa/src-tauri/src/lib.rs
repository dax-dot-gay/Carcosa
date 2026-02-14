pub mod procedures;
pub mod extensions;
pub mod types;
mod error;

pub use error::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .invoke_handler(procedures::handler())
        .setup(|app| {
                app.manage(types::ApplicationState::default());
                Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
