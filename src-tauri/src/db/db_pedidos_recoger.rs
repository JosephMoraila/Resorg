use crate::db::{
    insert_into_pedidos, insert_into_platillos_pedidos, obtener_conexion_db,
    transform_last_id_and_platillos_into_text, PlatilloPedido,
};
use crate::printing::print_order_escpos;
use rusqlite::Connection;

const TIPO_PEDIDO_RECOGER: &str = "recoger";

#[tauri::command]
pub fn insert_pedido_recoger(
    platillos: Vec<PlatilloPedido>,
    nombre_cliente: Option<String>,
    notas: Option<String>,
) -> Result<(), String> {
    let mut text_info_to_ticket: String = format!("PEDIDO RECOGER\n");
    if let Some(nombre_cliente_some) = nombre_cliente.as_deref() {
        text_info_to_ticket += &format!("Nombre cliente: {}\n", nombre_cliente_some);
    }
    if let Option::Some(notas_some) = notas.as_deref() {
        text_info_to_ticket += &format!("Notas: {}\n", notas_some);
    }

    let conn: Connection = obtener_conexion_db()?;

    let (last_id_pedido, platillos_real) = insert_into_pedidos(
        &platillos,
        TIPO_PEDIDO_RECOGER,
        nombre_cliente.as_deref(),
        notas.as_deref(),
    )?;

    let text_platillos: String =
        transform_last_id_and_platillos_into_text(last_id_pedido, &platillos_real);
    text_info_to_ticket += &text_platillos;

    insert_into_platillos_pedidos(last_id_pedido, platillos_real)?;

    //Insertar en pedidos_recoger
    let comando_insertar_recoger: &str = "INSERT INTO pedidos_recoger (pedido_id) VALUES (?1);";
    let parametros_insertar_recoger = rusqlite::params![last_id_pedido];
    conn.execute(comando_insertar_recoger, parametros_insertar_recoger)
        .map_err(|e: rusqlite::Error| e.to_string())?;

    print_order_escpos(&text_info_to_ticket)?;

    Ok(())
}
