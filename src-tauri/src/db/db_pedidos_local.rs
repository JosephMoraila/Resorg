use crate::db::{obtener_conexion_db, insert_into_pedidos, insert_into_platillos_pedidos, PlatilloPedido};
use rusqlite::{Connection};

const TIPO_PEDIDO_LOCAL: &str = "local";

#[tauri::command]
pub fn insert_pedido_local(platillos: Vec<PlatilloPedido>, piso: i64, mesa: i64, mesero_seleccionado: String, nombre_cliente: Option<String>, notas: Option<String>)->Result<(), String>{

    let conn: Connection = obtener_conexion_db()?;

    let (last_id_pedido, platillos_real) = insert_into_pedidos(&platillos, TIPO_PEDIDO_LOCAL, nombre_cliente.as_deref(), notas.as_deref())?;

    insert_into_platillos_pedidos(last_id_pedido, platillos_real)?;

    //Insertar en pedidos_local
    let comando_insert_pedidos_local: &str = "INSERT INTO pedidos_local (piso, mesa, mesero, pedido_id) VALUES (?1, ?2, ?3, ?4);";
    let parametros_pedidos_local = rusqlite::params![piso, mesa, &mesero_seleccionado, last_id_pedido];
    conn.execute(comando_insert_pedidos_local, parametros_pedidos_local).map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn is_mesa_ocupada(piso: i64, mesa: i64) -> Result<bool, String> {
    let conn: Connection = obtener_conexion_db()?;

    let comando: &str = "SELECT EXISTS(
        SELECT 1 FROM pedidos_local pl
        INNER JOIN pedidos p ON p.id = pl.pedido_id
        WHERE pl.piso = ?1 AND pl.mesa = ?2 AND p.estatus = 'pendiente'
    )";

    let parametros = rusqlite::params![piso, mesa];

    let esta_ocupada: bool = conn
        .query_row(comando, parametros, |row| row.get(0))
        .map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(esta_ocupada)
}