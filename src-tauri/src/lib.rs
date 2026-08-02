use std::sync::OnceLock;
use tauri::AppHandle;

mod window;
use crate::window::enfocar_ventana;
mod db;
use crate::db::{get_categories_platillo, inicializar_tablas, insert_category_platillo, insert_platillo, update_category_platillo, update_platillo, delete_category_platillo, delete_platillo};
mod path_and_files;
use crate::path_and_files::get_imagen_platillo;

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            enfocar_ventana,
            insert_category_platillo,
            get_categories_platillo,
            insert_platillo,
            get_imagen_platillo, update_category_platillo, update_platillo, delete_platillo, delete_category_platillo
        ])
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            inicializar_tablas()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
