use crate::db::{get_platillos_by_category_id, obtener_conexion_db, Platillo};
use rusqlite::{Connection, ErrorCode, ToSql};
use serde::{Deserialize, Serialize};

#[tauri::command]
/// Inserta una nueva categoría de platillo en la base de datos.
///
/// # Argumentos
///
/// * `name` - Nombre de la nueva categoría a registrar.
///
/// # Retorno
///
/// Retorna `Ok(u64)` con el ID generado automáticamente para la categoría recién insertada.
///
/// # Errors
///
/// Retornará un `Err(String)` si no se puede obtener la conexión o si falla el INSERT.
pub fn insert_category_platillo(name: String) -> Result<u64, String> {
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "INSERT INTO categories_platillos (name) VALUES(?1);";
    let parametros: &[&dyn rusqlite::ToSql] = rusqlite::params![name];

    conn.execute(comando, parametros)
        .map_err(|e: rusqlite::Error| {
            if let rusqlite::Error::SqliteFailure(err, _) = &e {
                if err.code == ErrorCode::ConstraintViolation {
                    return format!("Ya existe una categoría con el nombre '{}'", name);
                }
            }
            e.to_string()
        })?;

    let nuevo_id: i64 = conn.last_insert_rowid();
    Ok(nuevo_id as u64)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatilloCategoria {
    pub id: u64,
    pub nombre: String,
    pub platillos: Vec<Platillo>,
}

#[tauri::command]
pub fn get_categories_platillo() -> Result<Vec<PlatilloCategoria>, String> {
    let padre = PlatilloCategoria{
        id: 0,
        nombre: "Platillos".to_string(),
        platillos: get_platillos_by_category_id(None)?, // ahora sí se llena
    };

    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "SELECT id, name FROM categories_platillos";
    let mut stmt = conn
        .prepare(comando)
        .map_err(|e: rusqlite::Error| e.to_string())?;

    let categorias_basicas: Vec<(i64, String)> = stmt
        .query_map((), |row| {
            let id: i64 = row.get::<&str, i64>("id")?;
            let nombre: String = row.get::<&str, String>("name")?;
            Ok((id, nombre))
        })
        .map_err(|e: rusqlite::Error| e.to_string())?
        .filter_map(|fila: Result<(i64, String), rusqlite::Error>| fila.ok())
        .collect();

    let mut categorias: Vec<PlatilloCategoria> = vec![padre]; // 👈 padre va primero en el vec
    for (id, nombre) in categorias_basicas {
        let platillos: Vec<Platillo> = get_platillos_by_category_id(Some(id as u64))?;
        categorias.push(PlatilloCategoria {
            id: id as u64,
            nombre,
            platillos,
        });
    }

    Ok(categorias)
}