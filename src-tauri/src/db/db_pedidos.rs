use std::vec;
use crate::db::{obtener_conexion_db, get_platillo_by_id, Platillo};
use rusqlite::{Connection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatilloPedido{
    pub id_platillo: u64,
    pub id_category: Option<u64>,
    pub cantidad: u64
}



///Inserta un pedido en la tabla pedidos
/// # Argumentos:
/// * `platillos` - Platillos de tipo orden el cual se pasará cada uno (el ID basta) a la funcion `get_platillo_by_id()` para tomar un struct de platillo original y se pondrá en el vector a retornar.
/// * `tipo_pedido` - Una referencia a un texto el cual es el tipo de pedido como local, domicilio recoger y se pondrá en la tabla
/// * `nombre_cliente` - Referencia al nombre del cliente que puede ser None o tener un valor
/// * `notas` - Referencia a notas del pedido que puede ser None o tener un valor
/// 
/// # Retorna:
/// * `Ok((i64, Vec<Platillo>))` - Si todo sale bien retorna el ID del registro reciente agregado para la tabla platillos pedidos y la tabla de tipo de orden y tambien retorna el vector de tipo platillos que se usa para agregarlo a platillos pedidos(En el vector de tipo orden si un orden tiene en su campo cantidad más de dos en el vector de salida vendrán esos elementos agregados ese número de veces ya que el tipo Platillo no tiene campo cantidad).
/// * `Err(String)` - Indicando el tipo de error.
pub fn insert_into_pedidos(platillos: &Vec<PlatilloPedido>, tipo_pedido: &str, nombre_cliente: Option<&str>, notas: Option<&str>)->Result<(i64, Vec<Platillo>), String>{
    let conn: Connection = obtener_conexion_db()?;
    let comando_insert_pedidos: &str = "INSERT INTO pedidos (total, tipo_pedido, nombre_cliente, nota) VALUES (?1, ?2, ?3, ?4);";
    let mut total: f64 = 0.0;
    let mut platillos_real: Vec<Platillo> = vec![]; //Guardamos los objetos de platillos 

    for platillito in platillos{
        let plat_opt: Option<Platillo> = get_platillo_by_id(platillito.id_platillo)?;
        let Some(platillo_some) = plat_opt else{
            continue; // si era None, salta a la siguiente iteración
        };
        for _ in 0..platillito.cantidad{
            total += platillo_some.precio;
            platillos_real.push(platillo_some.clone());
        }
    }

    let parametros_pedidos = rusqlite::params![total, tipo_pedido, nombre_cliente, notas];
    conn.execute(comando_insert_pedidos, parametros_pedidos).map_err(|e: rusqlite::Error| e.to_string())?;
    let last_id_pedido: i64 = conn.last_insert_rowid();
    
    Ok((last_id_pedido, platillos_real))
}

///Inserta en la tabla platillos_pedidos, usa un for para cada platillo del vector
/// # Argumentos:
/// * `last_id_pedido` - El ID de la ultima fila insertada en la tabla pedidos (Lo retorna la función `insert_into_pedidos`)
/// * `platillos_real` - Vector de platillos que retorna la función `insert_into_pedidos`
/// 
/// # Retorna:
/// * `Ok(())` - Nada
/// * `Err(String)` - Indicando el tipo de error.
pub fn insert_into_platillos_pedidos(last_id_pedido: i64, platillos_real: Vec<Platillo>)->Result<(), String>{

    let conn: Connection = obtener_conexion_db()?;

    //Insertar en tabla platillos_pedidos
    let comando_insert_platillos_pedidos: &str = "INSERT INTO platillos_pedidos (name, precio, platillo_id, category_id, pedido_id) VALUES (?1, ?2, ?3, ?4, ?5);";
    let comando_insert_platillos_pedidos_null: &str = "INSERT INTO platillos_pedidos (name, precio, platillo_id, category_id,pedido_id) VALUES (?1, ?2, ?3, NULL, ?4);";
    for platillo_real in platillos_real {
        let n: &str = &platillo_real.nombre;
        let p: &f64 = &platillo_real.precio;
        let platillo_id: i64 = platillo_real.id as i64;
        let category_id: i64 = platillo_real.id_categoria as i64;
        let parametros_platillo_pedido = rusqlite::params![n, p, platillo_id, category_id, last_id_pedido];
        let parametros_platillo_pedido_null = rusqlite::params![n, p, platillo_id, last_id_pedido];
        if platillo_real.id_categoria == 0{
            conn.execute(comando_insert_platillos_pedidos_null, parametros_platillo_pedido_null).map_err(|e: rusqlite::Error| e.to_string())?;
        }else{
            conn.execute(comando_insert_platillos_pedidos, parametros_platillo_pedido).map_err(|e: rusqlite::Error| e.to_string())?;
        }
    }

    Ok(())
}

/// Se entrega un formato de texto con el ID del pedido y los platillos
/// # Argumentos:
/// * `last_id_pedido` - El ID de la ultima fila insertada en la tabla pedidos (Lo retorna la función `insert_into_pedidos`)
/// * `platillos_real` - Slice de platillos que retorna la función `insert_into_pedidos`
/// 
/// # Retorna:
/// `String` - String primero con el ID y abajo en una lista los platillos con su nombre y precio
pub fn transform_last_id_and_platillos_into_text(last_id_pedido: i64, platillos_real: &[Platillo])->String{

    let mut s: String = format!("Pedido ID: {}\n\n", last_id_pedido);
    s += "Platillos:\n\n";

    let mut lista: i32 = 1;

    for platillo in platillos_real{
        let n: &String = &platillo.nombre;
        let p: f64 = platillo.precio;
        s += &format!("{}.- Nombre: {} - Precio: {}\n", lista, n, p);
        lista += 1;
    }

    s += "\n";

    s
}