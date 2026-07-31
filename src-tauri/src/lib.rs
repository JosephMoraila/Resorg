use tauri::AppHandle;
use std::sync::OnceLock;

mod window;
use crate::window::{enfocar_ventana};
mod db;
use crate::db::{inicializar_tablas, insert_category_platillo};
mod path_and_files;

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            enfocar_ventana,
            insert_category_platillo
        ])
        .setup(|app|{
            APP_HANDLE.set(app.handle().clone()).unwrap();
            inicializar_tablas()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
