use crate::db::{
    insert_into_pedidos, insert_into_platillos_pedidos, obtener_conexion_db,
    transform_last_id_and_platillos_into_text, PlatilloPedido,
};
use crate::printing::print_order_escpos;
use rusqlite::Connection;

const TIPO_PEDIDO_LOCAL: &str = "local";

#[tauri::command]
pub fn insert_pedido_local(
    platillos: Vec<PlatilloPedido>,
    piso: i64,
    mesa: i64,
    mesero_seleccionado: String,
    nombre_cliente: Option<String>,
    notas: Option<String>,
) -> Result<(), String> {
    let mut text_info_to_ticket: String = format!(
        "PEDIDO LOCAL\nPiso: {} - Mesa: {}\nMesero: {}\n",
        piso, mesa, mesero_seleccionado
    );
    if let Some(nombre_cliente_some) = nombre_cliente.as_deref() {
        text_info_to_ticket += &format!("Nombre cliente: {}\n", nombre_cliente_some);
    }
    if let Option::Some(notas_some) = notas.as_deref() {
        text_info_to_ticket += &format!("Notas: {}\n", notas_some);
    }

    let conn: Connection = obtener_conexion_db()?;

    let (last_id_pedido, platillos_real) = insert_into_pedidos(
        &platillos,
        TIPO_PEDIDO_LOCAL,
        nombre_cliente.as_deref(),
        notas.as_deref(),
    )?;

    let text_platillos: String =
        transform_last_id_and_platillos_into_text(last_id_pedido, &platillos_real);
    text_info_to_ticket += &text_platillos;

    insert_into_platillos_pedidos(last_id_pedido, platillos_real)?;

    //Insertar en pedidos_local
    let comando_insert_pedidos_local: &str =
        "INSERT INTO pedidos_local (piso, mesa, mesero, pedido_id) VALUES (?1, ?2, ?3, ?4);";
    let parametros_pedidos_local =
        rusqlite::params![piso, mesa, &mesero_seleccionado, last_id_pedido];
    conn.execute(comando_insert_pedidos_local, parametros_pedidos_local)
        .map_err(|e: rusqlite::Error| e.to_string())?;

    print_order_escpos(&text_info_to_ticket)?;

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

#[tauri::command]
pub fn is_mesa_exists(mesa: i64) -> Result<bool, String> {
    let conn: Connection = obtener_conexion_db()?;

    let comando: &str = "SELECT EXISTS(
        SELECT 1 FROM edificio
        WHERE ?1 <= mesas
    )";

    let parametros = rusqlite::params![mesa];

    let is_mesa_existe: bool = conn
        .query_row(comando, parametros, |row| row.get(0))
        .map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(is_mesa_existe)
}

#[tauri::command]
pub fn is_piso_exists(piso: i64) -> Result<bool, String> {
    let conn: Connection = obtener_conexion_db()?;

    let comando: &str = "SELECT EXISTS(
        SELECT 1 FROM edificio
        WHERE piso = ?1
    )";

    let parametros = rusqlite::params![piso];

    let is_piso_existe: bool = conn
        .query_row(comando, parametros, |row| row.get(0))
        .map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(is_piso_existe)
}
