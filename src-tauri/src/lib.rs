use std::sync::{Mutex, OnceLock};
use tauri::AppHandle;
use std::collections::HashMap;

mod window;
use crate::window::enfocar_ventana;
mod db;
use crate::db::{PedidoCompartido,get_categories_platillo,insert_floor,delete_piso,delete_mesero,update_platillos_orden, insert_mesero, get_meseros, get_floor_and_mesas,update_floor, update_mesas, inicializar_tablas, insert_category_platillo, insert_platillo, update_category_platillo, update_platillo, delete_category_platillo, delete_platillo, is_mesa_ocupada, insert_pedido_local, insert_pedido_domicilio, insert_pedido_recoger, is_mesa_exists, is_piso_exists, obtener_ordenes, count_ordenes, guardar_pedido_compartido, obtener_pedido_compartido, update_pedido, cobrar_pedido};
mod path_and_files;
use crate::path_and_files::{get_imagen_platillo};
mod printer;
use crate::printer::{obtener_impresoras, save_printting_settings, get_printting_settings};
mod printing;
use crate::printing::{print_prueba, print_again_order_escpos};

mod escpos;
use crate::escpos::{save_escpos_html, get_escpos_html};

#[cfg(target_os = "windows")]
mod windows;

mod utils;

mod canvas;
use crate::canvas::{save_canvas, get_canvas};

mod ticket;
use crate::ticket::{save_ticket_measurement, get_ticket_measurement};

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
            insert_pedido_local, is_mesa_ocupada, insert_pedido_local, insert_pedido_domicilio, insert_pedido_recoger, is_mesa_exists, is_piso_exists,
            obtener_ordenes, count_ordenes, guardar_pedido_compartido ,obtener_pedido_compartido,
            update_pedido, update_platillos_orden, cobrar_pedido,
            save_canvas, save_ticket_measurement, save_escpos_html, get_ticket_measurement, get_canvas, get_escpos_html,
            print_prueba, print_again_order_escpos
        ])
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            inicializar_tablas()?;
            Ok(())
        })
        .manage(PedidoCompartido(Mutex::new(HashMap::new())))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
