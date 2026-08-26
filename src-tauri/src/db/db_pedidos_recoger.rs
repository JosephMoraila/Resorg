use crate::db::{obtener_conexion_db, insert_into_pedidos, insert_into_platillos_pedidos, PlatilloPedido};
use rusqlite::{Connection};

const TIPO_PEDIDO_RECOGER: &str = "recoger";

#[tauri::command]
pub fn insert_pedido_recoger(platillos: Vec<PlatilloPedido>, nombre_cliente: Option<String>, notas: Option<String>)->Result<(), String>{

    let conn: Connection = obtener_conexion_db()?;

    let (last_id_pedido, platillos_real) = insert_into_pedidos(&platillos, TIPO_PEDIDO_RECOGER, nombre_cliente.as_deref(), notas.as_deref())?;

    insert_into_platillos_pedidos(last_id_pedido, platillos_real)?;

    //Insertar en pedidos_recoger
    let comando_insertar_recoger: &str = "INSERT INTO pedidos_recoger (pedido_id) VALUES (?1);";
    let parametros_insertar_recoger = rusqlite::params![last_id_pedido];
    conn.execute(comando_insertar_recoger, parametros_insertar_recoger).map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(())
}