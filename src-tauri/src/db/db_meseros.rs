use crate::db::obtener_conexion_db;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mesero {
    pub id: u64,
    pub nombre: String,
}

#[tauri::command]
pub fn insert_mesero(nombre: String) -> Result<u64, String> {
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "INSERT INTO meseros (nombre) VALUES (?1);";
    conn.execute(comando, rusqlite::params![nombre])
        .map_err(|e: rusqlite::Error| e.to_string())?;
    let last_id: i64 = conn.last_insert_rowid();
    Ok(last_id as u64)
}

#[tauri::command]
pub fn get_meseros() -> Result<Vec<Mesero>, String> {
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "SELECT id, nombre FROM meseros";
    let mut stmt = conn
        .prepare(comando)
        .map_err(|e: rusqlite::Error| e.to_string())?;
    let iteraciones = stmt
        .query_map([], |row| {
            let ide: i64 = row.get::<&str, i64>("id")?;
            let nombrecito: String = row.get::<&str, String>("nombre")?;
            let mes = Mesero {
                id: ide as u64,
                nombre: nombrecito,
            };
            Ok(mes)
        })
        .map_err(|e: rusqlite::Error| e.to_string())?;

    let filtro = iteraciones.filter_map(|res: Result<Mesero, rusqlite::Error>| res.ok());
    let meseros: Vec<Mesero> = filtro.collect();
    Ok(meseros)
}

#[tauri::command]
pub fn delete_mesero(id: i64) -> Result<(), String> {
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "DELETE FROM meseros WHERE id = ?1;";
    conn.execute(comando, rusqlite::params![id])
        .map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(())
}
