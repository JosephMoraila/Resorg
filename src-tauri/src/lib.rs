use std::sync::OnceLock;
use tauri::AppHandle;

mod window;
use crate::window::enfocar_ventana;
mod db;
use crate::db::{get_categories_platillo,insert_floor,delete_piso,delete_mesero, insert_mesero, get_meseros, get_floor_and_mesas,update_floor, update_mesas, inicializar_tablas, insert_category_platillo, insert_platillo, update_category_platillo, update_platillo, delete_category_platillo, delete_platillo, is_mesa_ocupada, insert_pedido_local, insert_pedido_domicilio, insert_pedido_recoger};
mod path_and_files;
use crate::path_and_files::{get_imagen_platillo};
mod printer;
use crate::printer::{obtener_impresoras, save_printting_settings, get_printting_settings};

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
            get_imagen_platillo, update_category_platillo, update_platillo, delete_platillo, delete_category_platillo,
            insert_floor, update_floor, update_mesas, get_floor_and_mesas, delete_piso,
            insert_mesero, get_meseros, delete_mesero,
            obtener_impresoras, save_printting_settings, get_printting_settings,
            insert_pedido_local, is_mesa_ocupada, insert_pedido_local, insert_pedido_domicilio, insert_pedido_recoger
        ])
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            inicializar_tablas()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
