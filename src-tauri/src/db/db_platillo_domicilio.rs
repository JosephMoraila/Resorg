use crate::db::{obtener_conexion_db, insert_into_pedidos, insert_into_platillos_pedidos, PlatilloPedido, transform_last_id_and_platillos_into_text};
use rusqlite::{Connection};
use crate::printing::print_order_escpos;

const TIPO_PEDIDO_DOMICILIO: &str = "domicilio";

#[tauri::command]
pub fn insert_pedido_domicilio(platillos: Vec<PlatilloPedido>, nombre_cliente: Option<String>, notas: Option<String>, colonia: Option<String>, calle: Option<String>, numero_exterior_interior: Option<i64>, numero_celular: Option<String>, repartidor: String)->Result<(), String>{

    let mut text_info_to_ticket: String = format!("PEDIDO DOMICILIO\nRepartidor: {}\n", repartidor);
    if let Some(nombre_cliente_some) = nombre_cliente.as_deref(){
        text_info_to_ticket += &format!("Nombre cliente: {}\n", nombre_cliente_some);
    }
    if let Option::Some(notas_some) = notas.as_deref(){
        text_info_to_ticket += &format!("Notas: {}\n", notas_some);
    }
    if let Some(colonia_some) = colonia.as_deref(){
        text_info_to_ticket += &format!("Colonia: {}\n", colonia_some);
    }
    if let Some(calle_some) = calle.as_deref(){
        text_info_to_ticket += &format!("Calle: {}\n", calle_some);
    }
    if let Some(numero_exterior_interior_some) = numero_exterior_interior{
        text_info_to_ticket += &format!("Número interior/exterior: {}\n", numero_exterior_interior_some);
    }
    if let Some(numero_celular_some) = numero_celular.as_deref(){
        text_info_to_ticket += &format!("Número celular: {}\n", numero_celular_some);
    }

    let conn: Connection = obtener_conexion_db()?;

    let (last_id_pedido, platillos_real) = insert_into_pedidos(&platillos, TIPO_PEDIDO_DOMICILIO, nombre_cliente.as_deref(), notas.as_deref())?;

    let text_platillos: String = transform_last_id_and_platillos_into_text(last_id_pedido, &platillos_real);
    text_info_to_ticket += &text_platillos;
    
    insert_into_platillos_pedidos(last_id_pedido, platillos_real)?;
   

    //Insertar en pedidos_domicilio
    let comando_insertar_pedidos_domicilio: &str = "INSERT INTO pedidos_domicilio (colonia, calle, numero_interior_exterior, telefono, repartidor, pedido_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6);";
    let colonia_str: Option<&str> = colonia.as_deref();
    let calle_str: Option<&str> = calle.as_deref();
    let numero_celular_str: Option<&str> = numero_celular.as_deref();
    let parametros_pedido_domicilio = rusqlite::params![colonia_str, calle_str, numero_exterior_interior, numero_celular_str, &repartidor, last_id_pedido];
    conn.execute(comando_insertar_pedidos_domicilio, parametros_pedido_domicilio).map_err(|e: rusqlite::Error| e.to_string())?;

    print_order_escpos(&text_info_to_ticket)?;

    Ok(())
}