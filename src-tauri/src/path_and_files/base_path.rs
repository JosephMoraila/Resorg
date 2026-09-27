use crate::APP_HANDLE;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

///Obtiene y crea la ruta si no existe donde se guarda la infomación del programa
/// # Retorna
/// `Pathbuf` Linux: ~/.local/share/appname/ o en Windows C:\Users\Nombre\AppData\Roaming\appname\
/// `String` El mensaje de error
pub fn obtener_base_path() -> Result<PathBuf, String> {
    let app_reference: &AppHandle = APP_HANDLE.get().ok_or("no inicializado".to_string())?;
    let app = app_reference.clone();
    let app_dir: std::path::PathBuf = app.path().app_data_dir().map_err(|e| e.to_string())?;

    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;

    Ok(app_dir)
}

pub fn obtener_carpeta_imagen_platillos() -> Result<PathBuf, String> {
    let base_path: PathBuf = obtener_base_path()?;
    let image_folder: PathBuf = base_path.join("platillos_images");

    fs::create_dir_all(&image_folder).map_err(|e| e.to_string())?;

    Ok(image_folder)
}

#[tauri::command]
pub fn get_imagen_platillo(id: u64) -> Result<Option<Vec<u8>>, String> {
    let carpeta: PathBuf = obtener_carpeta_imagen_platillos()?;
    let ruta: PathBuf = carpeta.join(format!("platillo_{}.png", id));

    match std::fs::read(&ruta) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None), // caso esperado, no es error
        Err(e) => Err(e.to_string()), // error real (permisos, etc.)
    }
}

pub fn get_json_printing_settings() -> Result<PathBuf, String> {
    let base_path: PathBuf = obtener_base_path()?;
    let ruta: PathBuf = base_path.join("printing_settings.json"); 

    Ok(ruta)
}

//Canvas

pub fn obetener_canvas_folder()->Result<PathBuf, String>{
    let base_path: PathBuf = obtener_base_path()?;
    let canvas_folder: PathBuf = base_path.join("canvas_folder");

    fs::create_dir_all(&canvas_folder).map_err(|e| e.to_string())?;

    Ok(canvas_folder)
}

pub fn get_json_canvas() -> Result<PathBuf, String> {
    let canvas_folder: PathBuf = obetener_canvas_folder()?;
    
    let ruta: PathBuf = canvas_folder.join("canvas_elements.json"); 
    
    Ok(ruta)
}

//Ticket measurement

pub fn get_json_ticket_measurement()-> Result<PathBuf, String> {
    let base_path: PathBuf = obtener_base_path()?;
    let ruta: PathBuf = base_path.join("ticket_measurement.json"); 

    Ok(ruta)
}

//Escpos

pub fn obtener_escpos_folder()->Result<PathBuf, String>{
    let base_path: PathBuf = obtener_base_path()?;
    let escpos_folder: PathBuf = base_path.join("escpos_folder");

    fs::create_dir_all(&escpos_folder).map_err(|e| e.to_string())?;

    Ok(escpos_folder)
}

pub fn get_html_escpos_path() -> Result<PathBuf, String> {
    let escpos_folder: PathBuf = obtener_escpos_folder()?;
    
    let ruta: PathBuf = escpos_folder.join("escpos_elements.html"); 
    
    Ok(ruta)
}