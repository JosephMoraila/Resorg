use crate::db::obtener_conexion_db;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[tauri::command]
pub fn insert_floor(floor_number: i64)->Result<(), String>{
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "INSERT INTO edificio (piso) VALUES (?1);";
    conn.execute(comando, [floor_number]).map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn update_floor(new_floor_number: i64,old_floor_number: i64)->Result<(), String>{
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "UPDATE edificio SET piso = ?1 WHERE piso = ?2";
    conn.execute(comando, [new_floor_number, old_floor_number]).map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn update_mesas(floor_number: i64, mesas: u32)->Result<(), String>{
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "UPDATE edificio SET mesas = ?1 WHERE piso = ?2";
    conn.execute(comando, (mesas, floor_number)).map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn delete_piso(floor_number: i64)->Result<(), String>{
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "DELETE FROM edificio WHERE piso = ?1;";
    conn.execute(comando, [floor_number]).map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Piso {
    pub piso: i64,
    pub numero_mesas: u32,
}

#[tauri::command]
pub fn get_floor_and_mesas() -> Result<Vec<Piso>, String> {
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "SELECT piso, mesas FROM edificio";
    let mut stmt = conn.prepare(comando).map_err(|e: rusqlite::Error| e.to_string())?;

    let filas = stmt
        .query_map([], |row| {
            let pisito: i64 = row.get::<&str, i64>("piso")?;
            let mesas: i64 = row.get::<&str, i64>("mesas")?; 
            Ok(Piso {
                piso: pisito,
                numero_mesas: mesas as u32, 
            })
        }).map_err(|e: rusqlite::Error| e.to_string())?;

    let pisos: Vec<Piso> = filas
        .filter_map(|fila: Result<Piso, rusqlite::Error>| fila.ok())
        .collect();

    Ok(pisos)
}