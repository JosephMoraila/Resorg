use crate::db::{EstadoPedido, InfoTipoPedido, Pedido, PedidoDomicilio, PedidoLocal, PedidoRecoger,TipoPedido,PedidoPlatillo, obtener_conexion_db, MetodoPago};
use rusqlite::{Connection, Transaction};
use crate::utils::{descapitalizar};

///Verifica si el nuevo tipo es igual al que está en DB
/// # Arguments
/// * `new_tipo` - Una referencia a un TipoPedido que sería su nuevo tipo
/// * `pedido_id` - El ID del pedido que se quiere buscar su tipo en DB
/// # Returns
/// Tupla en su primer espacio si es o no el mismo tipo y en segundo espacio su tipo en DB
fn is_same_tipo_to_update(new_tipo: &TipoPedido, pedido_id: u64)->Result<(bool, TipoPedido), String>{
    let conn: Connection = obtener_conexion_db()?;
    let pedido_id_i64: i64 = pedido_id as i64;
    let param = rusqlite::params![pedido_id_i64];
    let comando: &str = "SELECT tipo_pedido FROM pedidos WHERE id = ?1;";
    let res_query: Result<String, rusqlite::Error> = conn.query_row(comando, param, |row|{
        let tipo_pedido_lower: String = row.get::<&str, String>("tipo_pedido")?;
        Ok(tipo_pedido_lower)
    });
    let tipo_pedido_lower: String = res_query.map_err(|e: rusqlite::Error|e.to_string())?;
    let tipo_in_db: TipoPedido = Pedido::create_new_tipo_pedido_by_name(&tipo_pedido_lower);
    let is: bool = if tipo_in_db == *new_tipo{
        true
    }else {
        false
    };
    Ok((is, tipo_in_db))
}

///Actualiza el registro de la tabla del tipo del pedido el cual es el mismo porque es el mismo tipo pero solo se cambia la información
/// # Arguments
/// * `pedido` - Estructura que viene desde frondend con los nuevos datos a actualizar
/// * `tx` - Referencia a transacción ya que si hay un error cancelar todo
///  # Returns
/// Enum de tipo InfoTipoPedido con su nueva información el cual es el mismo que viene en ese campo en el parametro `pedido`
fn update_same_tipo(pedido: Pedido, tx: &Transaction)->Result<InfoTipoPedido, String>{
    let info: InfoTipoPedido = match pedido.info_tipo_pedido {
        InfoTipoPedido::PedidoLocal(pl) =>{
            let comando_update_local: &str = "UPDATE pedidos_local SET piso = ?1, mesa = ?2, mesero = ?3 WHERE pedido_id = ?4;";
            let params_update_local = rusqlite::params![pl.piso, pl.mesa as i64, &pl.mesero, pl.pedido_id as i64];
            let fila_actualizada: usize = tx.execute(comando_update_local, params_update_local).map_err(|e: rusqlite::Error|e.to_string())?;
            if fila_actualizada == 0{
                return Err("No se pudo actualizar el pedido local".to_string());
            }
            //El ID del tipo es incluso el mismo porque no se esta borrando ni insertando nada, solo se está actualizando algo que ya era del mismo tipo
            let pedido_local = PedidoLocal{id: pl.id, mesa: pl.mesa, mesero: pl.mesero, pedido_id: pl.pedido_id, piso: pl.piso};
            let info = InfoTipoPedido::PedidoLocal(pedido_local);
            info
        }
        InfoTipoPedido::PedidoDomicilio(pd) =>{
            let comando_update_domicilio: &str = "UPDATE pedidos_domicilio SET colonia = ?1, calle = ?2, numero_interior_exterior = ?3, telefono = ?4, repartidor = ?5 WHERE pedido_id = ?6;";
            let colonia: Option<&str> = pd.colonia.as_deref();
            let calle: Option<&str> = pd.calle.as_deref();
            let numero_exterior_interior: Option<i64> = pd.numero_interior_exterior;
            let telefono: Option<&str> = pd.telefono.as_deref();
            let repartidor: Option<&str> = pd.repartidor.as_deref();
            let params_update_domicilio = rusqlite::params![colonia, calle, numero_exterior_interior, telefono, repartidor, pd.pedido_id as i64];
            let fila_actualizada: usize = tx.execute(comando_update_domicilio, params_update_domicilio).map_err(|e: rusqlite::Error|e.to_string())?;
            if fila_actualizada == 0{
                return Err("No se pudo actualizar el pedido domicilio".to_string());
            }
            let pedido_domicilio = PedidoDomicilio{id: pd.id, calle: pd.calle, colonia: pd.colonia, numero_interior_exterior: pd.numero_interior_exterior, pedido_id: pd.pedido_id, repartidor: pd.repartidor, telefono: pd.telefono};
            let info = InfoTipoPedido::PedidoDomicilio(pedido_domicilio);
            info
        }
        //Pedido recoger no actualiza nada ya que solo es una llave foranea y no contiene datos relevantes del pedido
        InfoTipoPedido::PedidoRecoger(pr) =>{
            let pedido_recoger = PedidoRecoger{id: pr.id, pedido_id: pr.pedido_id};
            let info = InfoTipoPedido::PedidoRecoger(pedido_recoger);
            info
        }
    };

    Ok(info)
}

///Elimina el registro del tipo antiguo e inserta en la tabla de su tipo un nuevo registro
/// # Arguments
/// * `pedido` - Estructura que viene desde frondend con los nuevos datos a insertar
/// * `tx` - Referencia a transacción ya que si hay un error cancelar todo
/// * `tipo_in_db` - Enum de su tipo antiguo en DB para saber en que tabla(domicilio, recoger o local) eliminar su registro
///  # Returns
/// Enum de tipo InfoTipoPedido con su nueva información incluyendo el ID recien insertado de su nueva tabla
fn update_different_tipo(pedido: Pedido, tx: &Transaction, tipo_in_db: TipoPedido)->Result<InfoTipoPedido, String>{
    let param_delete = rusqlite::params![pedido.id as i64];
    let comando_delete: &str = match tipo_in_db {
        TipoPedido::Domicilio => "DELETE FROM pedidos_domicilio WHERE pedido_id = ?1;",
        TipoPedido::Local =>"DELETE FROM pedidos_local WHERE pedido_id = ?1;",
        TipoPedido::Recoger =>"DELETE FROM pedidos_recoger WHERE pedido_id = ?1;"
    };
    let fila_eliminada: usize = tx.execute(comando_delete, param_delete).map_err(|e: rusqlite::Error|e.to_string())?;
    if fila_eliminada == 0{
        return Err("No se pudo eliminar el tipo de pedido asociado".to_string());
    }
    //Si todo sale bien al eliminar ahora insertamos la fila nueva en su nuevo tipo
    let info: InfoTipoPedido = match pedido.info_tipo_pedido{
        InfoTipoPedido::PedidoDomicilio(pd) =>{
            let comando_insert: &str = "INSERT INTO pedidos_domicilio (colonia, calle, numero_interior_exterior, telefono, repartidor, pedido_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6);";
            let colonia: Option<&str> = pd.colonia.as_deref();
            let calle: Option<&str> = pd.calle.as_deref();
            let numero_exterior_interior: Option<i64> = pd.numero_interior_exterior;
            let telefono: Option<&str> = pd.telefono.as_deref();
            let repartidor: Option<&str> = pd.repartidor.as_deref();
            let params_insert = rusqlite::params![colonia, calle, numero_exterior_interior, telefono, repartidor, pd.pedido_id as i64]; //El pedido ID aunque se cambie su tipo en la orden sigue siendo el mismo
            tx.execute(comando_insert, params_insert).map_err(|e: rusqlite::Error|e.to_string())?;
            let id_generado: i64 = tx.last_insert_rowid();
            let pedido_domicilio = PedidoDomicilio{id: id_generado as u64, calle: pd.calle, colonia: pd.colonia, numero_interior_exterior: pd.numero_interior_exterior, pedido_id: pd.pedido_id, repartidor: pd.repartidor, telefono: pd.telefono};
            let info = InfoTipoPedido::PedidoDomicilio(pedido_domicilio);
            info //Retornamos el info ya con la nueva información
        }
        InfoTipoPedido::PedidoLocal(pl) =>{
            let comando_insert = "INSERT INTO pedidos_local (piso, mesa, mesero, pedido_id) VALUES (?1, ?2, ?3, ?4);";
            let params_insert = rusqlite::params![pl.piso, pl.mesa as i64, &pl.mesero, pl.pedido_id as i64];
            tx.execute(comando_insert, params_insert).map_err(|e: rusqlite::Error|e.to_string())?;
            let id_generado: i64 = tx.last_insert_rowid();
            let pedido_local= PedidoLocal{id: id_generado as u64,mesa: pl.mesa, piso: pl.piso, mesero: pl.mesero, pedido_id: pl.pedido_id};
            let info = InfoTipoPedido::PedidoLocal(pedido_local);
            info
        }
        InfoTipoPedido::PedidoRecoger(pr)=>{
            let comando_insert: &str = "INSERT INTO pedidos_recoger (pedido_id) VALUES (?1);";
            let params_insert = rusqlite::params![pr.pedido_id as i64];
            tx.execute(comando_insert, params_insert).map_err(|e: rusqlite::Error|e.to_string())?;
            let id_generado: i64 = tx.last_insert_rowid();
            let pedido_recoger = PedidoRecoger{id: id_generado as u64, pedido_id: pr.pedido_id};
            let info = InfoTipoPedido::PedidoRecoger(pedido_recoger);
            info
        }
    };

    Ok(info)
}

#[tauri::command]
pub fn update_pedido(pedido: Pedido)->Result<Pedido, String>{
    println!("Estructura pedido: {:#?}", pedido);
    let (is_same, tipo_in_db) = is_same_tipo_to_update(&pedido.tipo, pedido.id)?;
    let mut conn: Connection = obtener_conexion_db()?;
    let cliente_nombre_ref: Option<&str> = pedido.nombre_cliente.as_deref();
    let nota_ref: Option<&str> = pedido.nota.as_deref();
    let estatus_clone: EstadoPedido = pedido.estado.clone();//Estado
    let estatus_string: String = estatus_clone.to_string();
    let estatus_lower: String = descapitalizar(&estatus_string);
    let tipo_clone: TipoPedido = pedido.tipo.clone();//Tipo
    let tipo_string: String = tipo_clone.to_string();
    let tipo_lower: String = descapitalizar(&tipo_string);
    let parametros_update_pedido = rusqlite::params![cliente_nombre_ref, nota_ref, &estatus_lower, &tipo_lower, pedido.id as i64];
    print!("Tipo pedido frondend: {}", &tipo_lower);
    let comando_update_pedido: &str = "UPDATE pedidos SET nombre_cliente = ?1, nota = ?2, estatus = ?3, tipo_pedido = ?4 WHERE id = ?5;";
    //Iniciamos transaccion
    let tx = conn.transaction().map_err(|e: rusqlite::Error|e.to_string())?;
    let fila_actualizada: usize = tx.execute(comando_update_pedido, parametros_update_pedido).map_err(|e: rusqlite::Error|e.to_string())?;
    if fila_actualizada == 0{
        return Err("No se pudo actualizar el pedido".to_string());
    }
    //Clonamos la lista de platillos y otros datos de pedido porque pedido se mueve al pasarlo a las funciones del siguiente if
    let platillos_pedidos: Vec<PedidoPlatillo> = pedido.platillos_pedidos.clone();
    let total: f64 = pedido.total;
    let tipo: TipoPedido = pedido.tipo.clone();
    let nota: Option<String> = pedido.nota.clone();
    let nombre_cliente: Option<String> = pedido.nombre_cliente.clone();
    let fecha_hora: String = pedido.fecha_hora.clone();
    let estado: EstadoPedido = pedido.estado.clone();
    let metodo_pago: Option<MetodoPago> = pedido.metodo_pago.clone();
    let pedido_id: u64 = pedido.id;
    let info: InfoTipoPedido = if is_same{
        let info = update_same_tipo(pedido, &tx)?;
        info
    }else{
        let info = update_different_tipo(pedido, &tx, tipo_in_db)?;
        info
    };
    //Confirmar trasnaccion si todo sale bien
    tx.commit().map_err(|e: rusqlite::Error|e.to_string())?;
    let pedido_actualizado = Pedido{id: pedido_id, estado, fecha_hora, nombre_cliente, nota, tipo, total, platillos_pedidos, info_tipo_pedido: info, metodo_pago};
    Ok(pedido_actualizado)
}