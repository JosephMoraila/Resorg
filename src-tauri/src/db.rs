mod db_category_platillo;
pub use crate::db::db_category_platillo::*;
mod db_food_platillo;
pub use crate::db::db_food_platillo::*;
mod db_floors;
pub use crate::db::db_floors::*;
mod db_meseros;
pub use crate::db::db_meseros::*;
mod db_pedidos_local;
pub use crate::db::db_pedidos_local::*;
mod db_pedidos;
pub use crate::db::db_pedidos::*;
mod db_platillo_domicilio;
pub use crate::db::db_platillo_domicilio::*;
mod db_pedidos_recoger;
pub use crate::db::db_pedidos_recoger::*;

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
    
        CREATE TABLE IF NOT EXISTS edificio (
            piso INTEGER UNIQUE NOT NULL,
            mesas INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS meseros (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre  TEXT NOT NULL UNIQUE COLLATE NOCASE
        );

        CREATE TABLE IF NOT EXISTS pedidos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            total REAL NOT NULL DEFAULT 0.0,
            tipo_pedido TEXT NOT NULL,
            nombre_cliente TEXT,
            nota TEXT,
            estatus TEXT NOT NULL DEFAULT 'pendiente',
            fecha_hora_pedido TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS platillos_pedidos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL COLLATE NOCASE,
            precio      REAL NOT NULL DEFAULT 0.0,
            platillo_id INTEGER NOT NULL,
            category_id INTEGER,
            pedido_id INTEGER NOT NULL,
            FOREIGN KEY (pedido_id) REFERENCES pedidos(id) ON DELETE CASCADE
            FOREIGN KEY (platillo_id) REFERENCES food_platillos(id)
        );

        CREATE TABLE IF NOT EXISTS pedidos_local(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            piso INTEGER NOT NULL,
            mesa INTEGER NOT NULL,
            mesero TEXT NOT NULL,
            pedido_id INTEGER NOT NULL,
            FOREIGN KEY (pedido_id) REFERENCES pedidos(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS pedidos_domicilio(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            colonia TEXT,
            calle TEXT,
            numero_interior_exterior INTEGER,
            telefono TEXT,
            repartidor TEXT,
            pedido_id INTEGER NOT NULL,
            FOREIGN KEY (pedido_id) REFERENCES pedidos(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS pedidos_recoger(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            pedido_id INTEGER NOT NULL,
            FOREIGN KEY (pedido_id) REFERENCES pedidos(id) ON DELETE CASCADE
        );

    ";

    conn.execute_batch(comando).map_err(|e: rusqlite::Error| e.to_string())?;

    let comando_count_pisos: &str = "SELECT COUNT(*) FROM edificio";
    let count_edificio: i32= conn.query_row(comando_count_pisos, [], |row| {
        let count: i32 = row.get(0)?;
        Ok(count)
    }).map_err(|e: rusqlite::Error| e.to_string())?;

    if count_edificio == 0 {
        let comando_insert: &str = "INSERT INTO edificio (piso, mesas) VALUES (1, 1)";
        conn.execute(comando_insert, []).map_err(|e: rusqlite::Error| e.to_string())?;
    }

    Ok(())
}
