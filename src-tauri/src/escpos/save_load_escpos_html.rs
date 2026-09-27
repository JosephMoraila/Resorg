use crate::path_and_files::{get_html_escpos_path};
use std::path::PathBuf;
use std::fs;

#[tauri::command]
pub fn save_escpos_html(contenido_html: String)->Result<(), String>{
    let escpos_html_file_path: PathBuf = get_html_escpos_path()?;

    fs::write(escpos_html_file_path, contenido_html).map_err(|e: std::io::Error|e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_escpos_html()->Result<String, String>{
    let escpos_html_file_path: PathBuf = get_html_escpos_path()?;
    let html_escpos: String = fs::read_to_string(escpos_html_file_path).unwrap_or_else(|_| "<div><br></div>".to_string());
    Ok(html_escpos)
}