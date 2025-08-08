pub mod api;
pub mod models;
pub mod carcosa;
pub mod types;
pub mod templates;
mod error;

use std::sync::Arc;

pub use error::{ Error, SerializableError, Result };
use native_db::Database;
use parking_lot::RwLock;
use tauri::Manager;

use crate::carcosa::CarcosaExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(api::router().into_handler())
        .setup(|app| {
            let _ = app.carcosa().app_state();
            app.manage(RwLock::new(None::<Arc<RwLock<Database>>>));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
