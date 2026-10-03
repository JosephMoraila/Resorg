use crate::path_and_files::get_json_ticket_measurement;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

const JSON_TICKET_WIDTH_CLAVE: &str = "width";
const JSON_TICKET_HEIGHT_CLAVE: &str = "height";
const JSON_TICKET_IS_CANVAS_CLAVE: &str = "is_canvas";

#[tauri::command]
pub fn save_ticket_measurement(width: u32, height: u32, is_canvas: bool) -> Result<(), String> {
    let ticket_measurement_path: PathBuf = get_json_ticket_measurement()?;
    let data: Value = json!({
        JSON_TICKET_WIDTH_CLAVE: width,
        JSON_TICKET_HEIGHT_CLAVE: height,
        JSON_TICKET_IS_CANVAS_CLAVE: is_canvas
    });

    let texto: String =
        serde_json::to_string_pretty(&data).map_err(|e: serde_json::Error| e.to_string())?;
    fs::write(ticket_measurement_path, texto).map_err(|e: std::io::Error| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_ticket_measurement() -> Result<(u32, u32, bool), String> {
    let ticket_measurement_path: PathBuf = get_json_ticket_measurement()?;
    let existe: bool = ticket_measurement_path.exists();
    if existe {
        let contenido: String =
            fs::read_to_string(ticket_measurement_path).map_err(|e| e.to_string())?;
        let data: Value = serde_json::from_str(&contenido).map_err(|e| e.to_string())?;

        let width_json: Value = data[JSON_TICKET_WIDTH_CLAVE].clone();
        let width_opt: Option<u64> = width_json.as_u64();
        let width: u64 = width_opt.unwrap_or(58);

        let height: u64 = data[JSON_TICKET_HEIGHT_CLAVE].as_u64().unwrap_or(100);

        let is_canvas: bool = data[JSON_TICKET_IS_CANVAS_CLAVE].as_bool().unwrap_or(true);

        Ok((width as u32, height as u32, is_canvas))
    } else {
        Ok((58, 100, true))
    }
}
