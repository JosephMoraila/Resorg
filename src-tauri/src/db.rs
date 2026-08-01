mod db_category_platillo;
pub use crate::db::db_category_platillo::*;
mod db_food_platillo;
pub use crate::db::db_food_platillo::*;

use crate::path_and_files::obtener_base_path;
use rusqlite::Connection;
use std::path::PathBuf;

pub fn obtener_conexion_db() -> Result<Connection, String> {
    let base_path: PathBuf = obtener_base_path()?;
    let db_path: PathBuf = base_path.join("info.db");
    let conn: Connection = Connection::open(&db_path).map_err(|e| e.to_string())?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(|e| e.to_string())?;

    Ok(conn)
}

pub fn inicializar_tablas() -> Result<(), String> {
    let conn: Connection = obtener_conexion_db()?;

    //Como en categories_platillos todos van al mismo nivel no se necesita un padre
    let comando: &str = "
        CREATE TABLE IF NOT EXISTS categories_platillos (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE COLLATE NOCASE
        );

        CREATE TABLE IF NOT EXISTS food_platillos (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL UNIQUE COLLATE NOCASE,
            descripcion TEXT,
            precio      REAL NOT NULL DEFAULT 0.0,
            category_id INTEGER,
            FOREIGN KEY (category_id) REFERENCES categories_platillos(id) ON DELETE CASCADE
        );
    ";

    conn.execute_batch(comando)
        .map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(())
}
