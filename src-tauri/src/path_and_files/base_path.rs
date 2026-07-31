use std::fs;
use std::path::PathBuf;
use tauri::AppHandle; 
use crate::APP_HANDLE;
use tauri::Manager;

///Obtiene y crea la ruta si no existe donde se guarda la infomación del programa
/// # Retorna
/// `Pathbuf` Linux: ~/.local/share/appname/ o en Windows C:\Users\Nombre\AppData\Roaming\appname\
/// `String` El mensaje de error
pub fn obtener_base_path()->Result<PathBuf, String>{
    let app_reference: &AppHandle = APP_HANDLE.get().ok_or("no inicializado".to_string())?;
    let app = app_reference.clone();
    let app_dir: std::path::PathBuf = app.path().app_data_dir().map_err(|e| e.to_string())?;

    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;

    Ok(app_dir)
}