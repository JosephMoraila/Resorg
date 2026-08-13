use printers::get_printers;
use crate::path_and_files::{get_json_printing_settings};
use std::path::PathBuf;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const JSON_PRINTER_CLAVE: &str = "printer";
const JSON_PRINT_ORDER_CLAVE: &str = "is_print_order";

#[tauri::command]
pub fn obtener_impresoras() -> Vec<String>{
    let mut impresoras: Vec<String> = Vec::new();
    if cfg!(target_os = "windows") {

        let printers_vec = get_printers();
        let ii = printers_vec.into_iter();
        let mapeado = ii.map(|p|p.name);
        impresoras = mapeado.collect();
        
    }else if cfg!(target_os = "linux"){

        let Ok(entradas) = std::fs::read_dir("/dev/usb") else {
            return impresoras;
        };

        for entrada in entradas.flatten() {
            let nombre = entrada.file_name();
            let nombre_str = nombre.to_string_lossy();
            if nombre_str.starts_with("lp") {
                impresoras.push(format!("/dev/usb/{}", nombre_str));
            }
        }

    }

    impresoras
}

#[tauri::command]
pub fn save_printting_settings(printer_name: String, is_print_order: bool)->Result<(), String>{
    let json_path: PathBuf = get_json_printing_settings()?;
    let data: Value = json!({
        JSON_PRINTER_CLAVE: printer_name,
        JSON_PRINT_ORDER_CLAVE: is_print_order
    });

    let texto: String = serde_json::to_string_pretty(&data).map_err(|e: serde_json::Error| e.to_string())?;
    fs::write(json_path, texto).map_err(|e: std::io::Error| e.to_string())?;

    Ok(())
}

#[tauri::command]
///Obtiene la configuración de impresion
/// # Retorna
/// Tupla en este orden: Nombre de la impresora seleccionada, si se imprime al crear una orden.
pub fn get_printting_settings()->Result<(String, bool), String>{
    let json_path: PathBuf = get_json_printing_settings()?;
    let existe: bool = json_path.exists();
    if existe{
        let contenido: String = fs::read_to_string(json_path).map_err(|e| e.to_string())?;
        let data: Value = serde_json::from_str(&contenido).map_err(|e| e.to_string())?;

        let imprimir_orden: bool = data[JSON_PRINT_ORDER_CLAVE].as_bool().unwrap_or(false);
        let impresora: &str = data[JSON_PRINTER_CLAVE].as_str().unwrap_or("");

        Ok((impresora.to_string(), imprimir_orden))
    }else{
        let vacio: String = String::new();
        Ok((vacio, false))
    }
}