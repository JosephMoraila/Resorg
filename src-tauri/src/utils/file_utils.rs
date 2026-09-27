use base64::{Engine as _, engine::general_purpose};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::fs;
use std::path::{Path, PathBuf};

pub fn guardar_base64_a_imagen(ruta_destino: &str, base64_string: &str) -> Result<(), String> {
    // Limpiar el string: Si incluye el prefijo "data:image/...", lo cortamos a partir de la coma
    let datos_limpios: &str = match base64_string.find(',') {
        Some(indice) => &base64_string[indice + 1..],
        None => base64_string,
    };

    // Decodificar el texto Base64 a un vector de bytes (Vec<u8>)
    let bytes_imagen: Vec<u8> = general_purpose::STANDARD
        .decode(datos_limpios)
        .map_err(|e| format!("Error al decodificar el texto Base64: {}", e))?;

    // Escribir los bytes binarios directamente en el archivo
    fs::write(Path::new(ruta_destino), bytes_imagen)
        .map_err(|e| format!("Error al guardar el archivo en disco: {}", e))?;

    Ok(())
}

/// Convierte un `PathBuf` a `String` de forma segura verificando su codificación.
///
/// Toma una ruta del sistema y valida que todos sus caracteres sean UTF-8 compatibles.
/// Retorna `Ok(String)` con la ruta en texto plano si es válida, o un `Err(String)`
/// con un mensaje de error si el sistema operativo entregó caracteres ilegibles.
pub fn conversion_segura(ruta: &PathBuf) -> Result<String, String> {
    match ruta.to_str() {
        Some(ruta_texto) => {
            let mi_string: String = ruta_texto.to_string();
            Ok(mi_string)
        }
        None => {
            Err("Error: La ruta contiene caracteres inválidos y no se puede leer".to_string())
        }
    }
}

/// Lee una imagen física del disco y devuelve su representación en Base64.
pub fn imagen_a_base64(ruta: &str) -> Result<String, String> {
    let bytes: Vec<u8> = fs::read(ruta).map_err(|e| format!("Error al leer el archivo de imagen: {}", e))?;

    let base64_string: String = STANDARD.encode(&bytes);

    Ok(base64_string)
}