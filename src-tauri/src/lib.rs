use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

mod window;
use crate::window::enfocar_ventana;
mod db;
use crate::db::{
    cobrar_pedido, count_ordenes, delete_category_platillo, delete_mesero, delete_piso,
    delete_platillo, get_categories_platillo, get_floor_and_mesas, get_meseros,
    guardar_pedido_compartido, inicializar_tablas, insert_category_platillo, insert_floor,
    insert_mesero, insert_pedido_domicilio, insert_pedido_local, insert_pedido_recoger,
    insert_platillo, is_mesa_exists, is_mesa_ocupada, is_piso_exists, obtener_ordenes,
    obtener_pedido_compartido, update_category_platillo, update_floor, update_mesas, update_pedido,
    update_platillo, update_platillos_orden, PedidoCompartido,
};
mod path_and_files;
use crate::path_and_files::get_imagen_platillo;
mod printer;
use crate::printer::{get_printting_settings, obtener_impresoras, save_printting_settings};
mod printing;
use crate::printing::{print_again_cobro_ticket, print_again_order_escpos, print_prueba};

mod escpos;
use crate::escpos::{get_escpos_html, save_escpos_html};

#[cfg(target_os = "windows")]
mod windows;

mod utils;

mod canvas;
use crate::canvas::{get_canvas, save_canvas};

mod ticket;
use crate::ticket::{get_ticket_measurement, save_ticket_measurement};

mod graficos;
use crate::graficos::{
    guardar_pedidos_filtros_compartido, obtener_pedidos_filtros_compartido,
    GraficaPedidosFiltrosCompartido,
};

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        //Registramos el plugin de Single Instance
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // 2. Este bloque se ejecuta cuando alguien intenta abrir una 2da instancia.

            // Buscamos tu ventana principal (por defecto se llama "main")
            if let Some(window) = app.get_webview_window("main") {
                // Hacemos visible la ventana si estaba oculta
                let _ = window.show();
                // La restauramos si estaba minimizada en la barra de tareas
                let _ = window.unminimize();
                // La traemos al frente
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]) //Se pueden pasar argumentos al ejecutable, en este caso no se pasan
        ))
        .invoke_handler(tauri::generate_handler![
            enfocar_ventana,
            insert_category_platillo,
            get_categories_platillo,
            insert_platillo,
            get_imagen_platillo,
            update_category_platillo,
            update_platillo,
            delete_platillo,
            delete_category_platillo,
            insert_floor,
            update_floor,
            update_mesas,
            get_floor_and_mesas,
            delete_piso,
            insert_mesero,
            get_meseros,
            delete_mesero,
            obtener_impresoras,
            save_printting_settings,
            get_printting_settings,
            insert_pedido_local,
            is_mesa_ocupada,
            insert_pedido_local,
            insert_pedido_domicilio,
            insert_pedido_recoger,
            is_mesa_exists,
            is_piso_exists,
            obtener_ordenes,
            count_ordenes,
            guardar_pedido_compartido,
            obtener_pedido_compartido,
            update_pedido,
            update_platillos_orden,
            cobrar_pedido,
            save_canvas,
            save_ticket_measurement,
            save_escpos_html,
            get_ticket_measurement,
            get_canvas,
            get_escpos_html,
            print_prueba,
            print_again_order_escpos,
            print_again_cobro_ticket,
            guardar_pedidos_filtros_compartido,
            obtener_pedidos_filtros_compartido
        ])
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            inicializar_tablas()?;
            Ok(())
        })
        .manage(PedidoCompartido(Mutex::new(HashMap::new())))
        .manage(GraficaPedidosFiltrosCompartido(Mutex::new(HashMap::new())))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
