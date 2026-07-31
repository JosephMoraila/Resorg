use crate::db::obtener_conexion_db;
use rusqlite::{Connection, ToSql};


#[tauri::command]
/// Inserta una nueva categoría de platillo en la base de datos.
///
/// # Argumentos
///
/// * `app` - Manejador de la aplicación de Tauri (`&AppHandle`).
/// * `name` - Nombre de la nueva categoría a registrar.
///
/// # Retorno
///
/// Retorna `Ok(i64)` con el ID generado automáticamente para la categoría recién insertada.
///
/// # Errors
///
/// Retornará un `Err(String)` en caso de que:
/// * No se pueda obtener la conexión a la base de datos.
/// * Ocurra un error al ejecutar la instrucción SQL (por ejemplo, si el nombre viola una restricción de unicidad).
pub fn insert_category_platillo(name: String) -> Result<u64, String> {
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "INSERT INTO categories_platillos (name) VALUES(?1);";
    let parametros: &[&dyn ToSql] = rusqlite::params![name];
    
    conn.execute(comando, parametros).map_err(|e: rusqlite::Error| e.to_string())?;

    // Obtener el ID generado por autoincrement
    let nuevo_id: i64 = conn.last_insert_rowid();
    let last_id: u64 = nuevo_id as u64;

    Ok(last_id)
}