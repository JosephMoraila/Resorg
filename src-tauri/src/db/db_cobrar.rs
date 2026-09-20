use crate::db::{Pedido, obtener_conexion_db};
use rusqlite::{Connection, params};

const COBRADO: &str = "cobrado";

#[tauri::command]
pub fn cobrar_pedido(pedido: Pedido)->Result<(), String>{
    let conn_res: Result<Connection, String> = obtener_conexion_db();
    let conn: Connection = conn_res?;
    let params = params![COBRADO, pedido.id as i64];
    let comando: &str = "UPDATE pedidos SET estatus = ?1 WHERE id = ?2;";
    conn.execute(comando, params).map_err(|e: rusqlite::Error|e.to_string())?;

    Ok(())
}