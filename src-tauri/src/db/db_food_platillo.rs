use rusqlite::Connection;
use crate::db::obtener_conexion_db;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Platillo {
    pub id: u64,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: f64,
    pub id_categoria: u64,
}

pub fn get_platillos_by_category_id(category_id: u64) -> Result<Vec<Platillo>, String> {
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str =
        "SELECT id, name, descripcion, precio, category_id FROM food_platillos WHERE category_id = ?1";
    let mut stmt = conn.prepare(comando).map_err(|e: rusqlite::Error| e.to_string())?;

    let platillos_iter = stmt
        .query_map([category_id as i64], |row| {
            let id: i64 = row.get::<&str, i64>("id")?;
            let nombre: String = row.get::<&str, String>("name")?;
            let descripcion: Option<String> = row.get::<&str, Option<String>>("descripcion")?;
            let precio: f64 = row.get::<&str, f64>("precio")?;
            let category_id: i64 = row.get::<&str, i64>("category_id")?;

            Ok(Platillo {
                id: id as u64,
                nombre,
                descripcion,
                precio,
                id_categoria: category_id as u64,
            })
        })
        .map_err(|e: rusqlite::Error| e.to_string())?;

    let comidas: Vec<Platillo> = platillos_iter
        .filter_map(|res: Result<Platillo, rusqlite::Error>| res.ok())
        .collect();

    Ok(comidas)
}