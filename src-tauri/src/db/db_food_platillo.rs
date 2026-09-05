use crate::db::obtener_conexion_db;
use crate::path_and_files::obtener_carpeta_imagen_platillos;
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Platillo {
    pub id: u64,
    pub tipo: Tipo,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: f64,
    pub id_categoria: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tipo{
    #[serde(rename = "categoria")]
    Categoria,
    #[serde(rename = "platillo")]
    Platillo,
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_platillo(nombre: String, descripcion: Option<String>, precio: f64, id_categoria: Option<u64>, image_bytes: Option<Vec<u8>>) -> Result<u64, String> {
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "INSERT INTO food_platillos (name, descripcion, precio, category_id) VALUES (?1, ?2, ?3, ?4)";
    conn.execute(comando, (nombre, descripcion, precio, id_categoria.map(|v| v as i64))).map_err(|e: rusqlite::Error| e.to_string())?;
    let last_id = conn.last_insert_rowid();

    if let Some(image_data) = image_bytes {
        let image_folder: std::path::PathBuf = obtener_carpeta_imagen_platillos()?;
        let image_path: std::path::PathBuf = image_folder.join(format!("platillo_{}.png", last_id));
        std::fs::write(image_path, image_data).map_err(|e| e.to_string())?;
    }

    Ok(last_id as u64)
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_platillo(id: u64,nombre: String,descripcion: Option<String>,precio: f64,image_bytes: Option<Vec<u8>>,) -> Result<(), String> {
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "UPDATE food_platillos SET name = ?1, descripcion = ?2, precio = ?3 WHERE id = ?4";
    conn.execute(comando, (nombre, descripcion, precio, id as i64)).map_err(|e: rusqlite::Error| e.to_string())?;

    let image_folder: std::path::PathBuf = obtener_carpeta_imagen_platillos()?;
    let image_path: std::path::PathBuf = image_folder.join(format!("platillo_{}.png", id));

    match image_bytes {
        Some(bytes) => {
            // Se pasó una imagen: se guarda en su lugar, exista o no exista previamente
            std::fs::write(&image_path, bytes).map_err(|e| e.to_string())?;
        }
        None => {
            // No se pasó imagen: si antes había una, se elimina; si no había, no se hace nada
            if image_path.exists() {
                std::fs::remove_file(&image_path).map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub fn delete_platillo(id: u64) -> Result<(), String> {
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "DELETE FROM food_platillos WHERE id = ?1";
    conn.execute(comando, (id as i64,)).map_err(|e: rusqlite::Error| e.to_string())?;

    let image_folder: std::path::PathBuf = obtener_carpeta_imagen_platillos()?;
    let image_path: std::path::PathBuf = image_folder.join(format!("platillo_{}.png", id));
    if image_path.exists() {
        std::fs::remove_file(image_path).map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn get_platillos_by_category_id(category_id: Option<u64>) -> Result<Vec<Platillo>, String> {
    let conn: Connection = obtener_conexion_db()?;

    let comando: &str = match category_id {
        Some(_) => "SELECT id, name, descripcion, precio, category_id FROM food_platillos WHERE category_id = ?1",
        None => "SELECT id, name, descripcion, precio, category_id FROM food_platillos WHERE category_id IS NULL",
    };

    let mut stmt = conn
        .prepare(comando)
        .map_err(|e: rusqlite::Error| e.to_string())?;

    let mapper = |row: &rusqlite::Row| -> rusqlite::Result<Platillo> {
        let id: i64 = row.get("id")?;
        let nombre: String = row.get("name")?;
        let descripcion: Option<String> = row.get("descripcion")?;
        let precio: f64 = row.get("precio")?;
        let category_id: Option<i64> = row.get("category_id")?; //Option, porque puede ser NULL

        Ok(Platillo {
            id: id as u64,
            nombre,
            descripcion,
            precio,
            tipo: Tipo::Platillo,
            id_categoria: category_id.map(|v| v as u64).unwrap_or(0),
        })
    };

    let platillos_iter = match category_id {
        Some(cid) => stmt.query_map([cid as i64], mapper),
        None => stmt.query_map([], mapper),
    }
    .map_err(|e: rusqlite::Error| e.to_string())?;

    let comidas: Vec<Platillo> = platillos_iter
        .filter_map(|res: Result<Platillo, rusqlite::Error>| res.ok())
        .collect();

    Ok(comidas)
}

pub fn get_platillo_by_id(platillo_id: u64)->Result<Option<Platillo>, String>{
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str  = "SELECT name, descripcion, precio, category_id FROM food_platillos WHERE id = ?1";
    let params = rusqlite::params![platillo_id as i64];

    let res: Result<Platillo, rusqlite::Error> = conn.query_row(comando, params, |row|{
        let nombre: String = row.get::<&str, String>("name")?;
        let descripcion: Option<String> = row.get::<&str, Option<String>>("descripcion")?;
        let precio: f64 = row.get::<&str, f64>("precio")?;
        let category_id: Option<i64> = row.get("category_id")?; //Option, porque puede ser NULL
        let cat: u64 = category_id.map(|v| v as u64).unwrap_or(0); //Si la categoria es NULL guardarla como 0
        let p = Platillo{id: platillo_id, descripcion: descripcion, id_categoria: cat, nombre, precio, tipo: Tipo::Platillo};
        Ok(p)
    });
    let opt_res: Result<Option<Platillo>, rusqlite::Error> = res.optional(); //Si no se encontró hacerlo option
    let platillo: Option<Platillo> = opt_res.map_err(|e: rusqlite::Error| e.to_string())?;
    Ok(platillo)
}