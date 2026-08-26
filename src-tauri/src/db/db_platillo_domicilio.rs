use crate::db::{obtener_conexion_db, insert_into_pedidos, insert_into_platillos_pedidos, PlatilloPedido};
use rusqlite::{Connection};

const TIPO_PEDIDO_DOMICILIO: &str = "domicilio";

#[tauri::command]
pub fn insert_pedido_domicilio(platillos: Vec<PlatilloPedido>, nombre_cliente: Option<String>, notas: Option<String>, colonia: Option<String>, calle: Option<String>, numero_exterior_interior: Option<i64>, numero_celular: Option<String>, repartidor: String)->Result<(), String>{

    let conn: Connection = obtener_conexion_db()?;

    let (last_id_pedido, platillos_real) = insert_into_pedidos(&platillos, TIPO_PEDIDO_DOMICILIO, nombre_cliente.as_deref(), notas.as_deref())?;

    insert_into_platillos_pedidos(last_id_pedido, platillos_real)?;

    //Insertar en pedidos_domicilio
    let comando_insertar_pedidos_domicilio: &str = "INSERT INTO pedidos_domicilio (colonia, calle, numero_interior_exterior, telefono, repartidor, pedido_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6);";
    let colonia_str: Option<&str> = colonia.as_deref();
    let calle_str: Option<&str> = calle.as_deref();
    let numero_celular_str: Option<&str> = numero_celular.as_deref();
    let parametros_pedido_domicilio = rusqlite::params![colonia_str, calle_str, numero_exterior_interior, numero_celular_str, &repartidor, last_id_pedido];
    conn.execute(comando_insertar_pedidos_domicilio, parametros_pedido_domicilio).map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(())
}