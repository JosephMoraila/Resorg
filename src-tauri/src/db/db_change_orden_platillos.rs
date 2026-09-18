use std::vec;
use std::collections::HashSet;
use rusqlite::{Connection, params_from_iter};
use serde::{Deserialize, Serialize};
use crate::db::{obtener_conexion_db, obtener_platillos_pedidos_by_pedido_id,insert_into_platillos_pedidos, PedidoPlatillo, get_platillo_by_id, Platillo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePlatilloPedido{
    pub id_platillo_pedido_original: Option<u64>,
    pub id_platillo: u64,
    pub id_category: Option<u64>,
    pub cantidad: u64,
    pub pedido_id: u64
}

fn get_pedido_id_by_id_platillo_pedido(id_platillo_pedido: u64)->Result<i64, String>{
    let conn: Connection = obtener_conexion_db()?;
    let comando: &str = "SELECT pedido_id FROM platillos_pedidos WHERE id = ?1;";
    let params = rusqlite::params![id_platillo_pedido as i64];
    let pedido_id: i64 = conn.query_row(comando, params, |row|{
        let pedido_id: i64 = row.get::<&str, i64>("pedido_id")?;
        Ok(pedido_id)
    }).map_err(|e: rusqlite::Error|e.to_string())?;
    Ok(pedido_id)
}

fn actualizar_total_pedido_id(pedido_id: i64, nuevos_platillos: &[PedidoPlatillo])->Result<(), String>{
    let mut nuevo_total: f64 = 0.0;
    for np in nuevos_platillos{
        nuevo_total += np.precio;
    }
    let comando: &str = "UPDATE pedidos SET total = ?1 WHERE id = ?2;";
    let params = rusqlite::params![nuevo_total, pedido_id];
    let conn: Connection = obtener_conexion_db()?;
    conn.execute(comando, params).map_err(|e: rusqlite::Error|e.to_string())?;
    Ok(())
}

///Elimina los platillos pedidos que estan platillos_pedidos_original_antes pero no en ids_mantener_frondend
/// # Arguments
///* `platillos_pedidos_original_antes` - Una referencia a vector que contiene el los platillos originales pedidos pero solo se requiere sus IDs
///* `ids_mantener_frondend` - Referencia a vector de IDs que manda el frondend y se compara con los platillos_pedidos_original_antes para saber cuales no están en ids_mantener_frondend y eliminarlo de DB
fn if_not_is_id_erase_from_db(platillos_pedidos_original_antes: &[PedidoPlatillo], ids_mantener_frondend: &[u64])->Result<(), String>{
    if ids_mantener_frondend.is_empty(){
        return Ok(())
    }
    //Tomamos el campo id de PedidoPlatillo para guardar en un vector cada id
    let iterador_struct_original= platillos_pedidos_original_antes.iter();
    let m = iterador_struct_original.map(|ppo|ppo.id);
    let ids_originales: Vec<u64> = m.collect();
    //Ahora transformar los del frondend en un set para luego compararlo con los ids_originales y ver cuales no estan
    let set_frondend_ids: HashSet<_> = ids_mantener_frondend.iter().copied().collect();
    //
    let iterador_vec_ids_originales = ids_originales.iter().copied();
    //Para que quede claro: ids_originales es el que se pone filter porque contiene TODOS los ids y por cada id se filtra cada uno para pasarlo al SET y preguntarle por cada elemento si esta
    //en el SET y si no está (porque se puso !) se sustrae ese valor
    let filtro = iterador_vec_ids_originales.filter(|id|!set_frondend_ids.contains(id));
    let ids_eliminados_de_original: Vec<_> = filtro.collect();
    let ids_eliminados_de_original_i64: Vec<i64> = ids_eliminados_de_original.iter().map(|&x| x as i64).collect(); //Como Sqlite solo acepta i64
    if ids_eliminados_de_original_i64.is_empty(){//Si no se eliminó nada no hacer nada
        return Ok(())
    }
    let vector_interrogracion: Vec<&str> = vec!["?"; ids_eliminados_de_original_i64.len()]; //Vector que contiene ? y el tamaño
    let string_placeholder: String = vector_interrogracion.join(", ");
    let comando: String = format!("DELETE FROM platillos_pedidos WHERE id IN ({})", string_placeholder);
    let conn: Connection = obtener_conexion_db()?;
    let filas_afectadas: usize = conn.execute(&comando, params_from_iter(ids_eliminados_de_original_i64)).map_err(|e: rusqlite::Error|e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_platillos_orden(platillos_actualizado_param: Vec<UpdatePlatilloPedido>, pedido_id: i64)->Result<Vec<PedidoPlatillo>, String>{
    let mut ids_mantener_frondend: Vec<u64> = vec![];//Se compara con con platillos_pedidos_original_antes para borrar de DB los platillos pedidos que no estan en ids_mantener_frondend
    let platillos_pedidos_original_antes: Vec<PedidoPlatillo> = obtener_platillos_pedidos_by_pedido_id(pedido_id as u64)?;
    let mut platillos_nuevos: Vec<Platillo> = vec![];
    for pap in &platillos_actualizado_param{
        //
        if let Some(n) = pap.id_platillo_pedido_original{
            ids_mantener_frondend.push(n);
        }else{//Si es NONE entonces agregar un nuevo platillo pedido
            let platillo_struct_opt: Option<Platillo> = get_platillo_by_id(pap.id_platillo)?; //Tomamos su ID y tomamos su platillo
            if let Some(platillo_struct_some) = platillo_struct_opt{
                platillos_nuevos.push(platillo_struct_some);
            }
        }
    }
    insert_into_platillos_pedidos(pedido_id, platillos_nuevos)?; //Insertar los nuevos
    if_not_is_id_erase_from_db(&platillos_pedidos_original_antes, &ids_mantener_frondend)?;
    //Deespues de hacer todo eso volver a llamar obtener_platillos_pedidos_by_pedido_id ya que ya se hizo DELETE o INSERT para obtener la info mas reciente
    let nuevos: Vec<PedidoPlatillo> = obtener_platillos_pedidos_by_pedido_id(pedido_id as u64)?;

    //Actualizar total en DB
    actualizar_total_pedido_id(pedido_id, &nuevos)?;
    Ok(nuevos)
}