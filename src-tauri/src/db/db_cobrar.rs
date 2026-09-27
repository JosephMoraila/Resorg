use crate::db::{Pedido, obtener_conexion_db, MetodoPago,};
use rusqlite::{Connection, params};
use crate::utils::{descapitalizar,};

const COBRADO: &str = "cobrado";

#[tauri::command]
pub fn cobrar_pedido(pedido: Pedido, metodo_pago: MetodoPago, contenido_ticket: String) -> Result<(), String> {
    let mut conn = obtener_conexion_db()?;

    // Iniciamos la transacción
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let i64_pedido_id = pedido.id as i64;

    //Actualizar estatus del pedido
    let comando_update = "UPDATE pedidos SET estatus = ?1 WHERE id = ?2;";
    tx.execute(comando_update, params![COBRADO, i64_pedido_id]).map_err(|e| e.to_string())?;

    // Insertar en pedidos_pagados
    let metodo_string = metodo_pago.to_string();
    let metodo_lower = descapitalizar(&metodo_string);
    let comando_insert = "INSERT INTO pedidos_pagados (pedido_id, metodo) VALUES(?1, ?2);";
    
    tx.execute(comando_insert, params![i64_pedido_id, metodo_lower]).map_err(|e| e.to_string())?;

    // Confirmamos la transacción para guardar los cambios permanentemente
    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}