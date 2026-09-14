use std::{fmt::{self}};

use rusqlite::{Connection, ToSql};
use crate::db::{PAGINACION_50_TAMANO, Platillo, calcular_offset, obtener_conexion_db, get_platillo_by_id};
use serde::{Deserialize, Serialize};
use crate::utils::{descapitalizar, capitalizar, convertir_local_a_utc};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

pub struct PedidoCompartido(pub Mutex<HashMap<String, Pedido>>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PedidoPlatillo{
    pub id: u64,
    pub name: String,
    pub precio: f64,
    pub platillo_id: u64,
    pub categoria_id: u64,
    pub pedido_id: u64,
    pub platillo: Option<Platillo>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PedidoLocal{
    pub id: u64,
    pub piso: i64,
    pub mesa: u64,
    pub mesero: String,
    pub pedido_id: u64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PedidoDomicilio{
    pub id: u64,
    pub colonia: Option<String>,
    pub calle: Option<String>,
    pub numero_interior_exterior: Option<i64>,
    pub telefono: Option<String>,
    pub repartidor: Option<String>,
    pub pedido_id: u64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PedidoRecoger{
    pub id: u64,
    pub pedido_id: u64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "tipo")]
pub enum InfoTipoPedido {
    #[serde(rename = "Local")]
    PedidoLocal(PedidoLocal),
    #[serde(rename = "Domicilio")]
    PedidoDomicilio(PedidoDomicilio),
    #[serde(rename = "Recoger")]
    PedidoRecoger(PedidoRecoger),
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum TipoPedido{
    Local, Domicilio, Recoger
}

impl fmt::Display for TipoPedido {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TipoPedido::Domicilio => write!(f, "Domicilio"),
            TipoPedido::Local => write!(f, "Local"),
            TipoPedido::Recoger => write!(f, "Recoger"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EstadoPedido {
    Pendiente, Finalizado, Cancelado, Entregado, Cobrado
}

impl fmt::Display for EstadoPedido {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EstadoPedido::Pendiente => write!(f, "Pendiente"),
            EstadoPedido::Finalizado => write!(f, "Finalizado"),
            EstadoPedido::Cancelado => write!(f, "Cancelado"),
            EstadoPedido::Cobrado => write!(f, "Cobrado"),
            EstadoPedido::Entregado => write!(f, "Entregado"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pedido{
    pub id: u64,
    pub total: f64,
    pub tipo: TipoPedido,
    pub estado: EstadoPedido,
    pub nombre_cliente: Option<String>,
    pub nota: Option<String>,
    pub fecha_hora: String,
    pub platillos_pedidos: Vec<PedidoPlatillo>,
    pub info_tipo_pedido: InfoTipoPedido
}

impl Pedido {
    ///Retorna un enum con struct vacio dependiendo del tipo de pedido, ya sea local, domicilio o recoger
    /// # Arguments
    /// * `info_tipo_pedido_str` - Un string que representa el tipo de pedido
    /// # Returns
    /// * `InfoTipoPedido` - Un enum que contiene un struct vacío correspondiente
    fn create_new_info_tipo_pedido_by_name(info_tipo_pedido_str: &str)->InfoTipoPedido{
        if info_tipo_pedido_str == "local" || info_tipo_pedido_str == "Local"{
            InfoTipoPedido::PedidoLocal(PedidoLocal{id: 0, piso: 0, mesa: 0, mesero: String::new(), pedido_id: 0})
        }else if info_tipo_pedido_str == "domicilio" || info_tipo_pedido_str == "Domicilio"{
            InfoTipoPedido::PedidoDomicilio(PedidoDomicilio{id: 0, colonia: None, calle: None, numero_interior_exterior: None, telefono: None, repartidor: None, pedido_id: 0})
        }else{
            InfoTipoPedido::PedidoRecoger(PedidoRecoger{id: 0, pedido_id: 0})
        }
    }

    pub fn create_new_tipo_pedido_by_name(info_tipo_pedido_str: &str)->TipoPedido{
        if info_tipo_pedido_str == "local" || info_tipo_pedido_str == "Local"{
            TipoPedido::Local
        }else if info_tipo_pedido_str == "domicilio" || info_tipo_pedido_str == "Domicilio"{
            TipoPedido::Domicilio
        }else{
            TipoPedido::Recoger
        }
    }

    fn create_new_estado_pedido_by_name(estatus_str: &str)->EstadoPedido{
        if estatus_str == "pendiente" || estatus_str == "Pendiente"{
            EstadoPedido::Pendiente
        }else if estatus_str == "finalizado" || estatus_str == "Finalizado"{
            EstadoPedido::Finalizado
        }else if estatus_str == "cancelado" || estatus_str == "Cancelado"{
            EstadoPedido::Cancelado
        }else if estatus_str == "entregado" || estatus_str == "Entregado"{
            EstadoPedido::Entregado
        }else{
            EstadoPedido::Cobrado
        }
    }



}

#[tauri::command]
pub fn obtener_ordenes(pagina_frontend: i64, id: Option<u64>, tipo_pedido: Option<TipoPedido>, nombre_cliente: Option<String>, fecha_inicio: Option<String>, fecha_fin: Option<String>, total_desde: Option<f64>, total_hasta: Option<f64>, nota: Option<String>, estatus: Option<EstadoPedido>) -> Result<Vec<Pedido>, String>{
    let conn: Connection = obtener_conexion_db()?;
    let mut pedidos: Vec<Pedido> = Vec::new();
    let mut comando: String = String::from("SELECT id, total, tipo_pedido, nombre_cliente, nota, estatus, fecha_hora_pedido FROM pedidos");
    let pagina: i64 = calcular_offset(pagina_frontend);
    let mut parametros: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    let mut numero_parametro: i32 = 1;
    let mut vec_campos_where: Vec<String> = vec![];

    if let Some(estatus_some) = estatus{ //EL unico campo que se valida sin importar el ID es el estatus
        let str_some: String = estatus_some.to_string();
        let lower: String = descapitalizar(&str_some);
        let text_param = format!("estatus = ?{}", numero_parametro);
        let caja: Box<String> = Box::new(lower);
        let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
        parametros.push(caja_as);
        numero_parametro += 1;
        vec_campos_where.push(text_param);
    }

    if let Some(id_some) = id { //Buscar por ID anula los otros campos
        let s: String = format!("id = ?{}", numero_parametro); 
        numero_parametro += 1;
        parametros.push(Box::new(id_some as i64) as Box<dyn rusqlite::ToSql>); 
        vec_campos_where.push(s);
    }else{//Se validan los otros campos
        if let Some(tipo_pedido_some) = tipo_pedido{
            let str_some: String = tipo_pedido_some.to_string();
            let lower: String = descapitalizar(&str_some);
            let text_param = format!("tipo_pedido = ?{}", numero_parametro);
            let caja: Box<String> = Box::new(lower);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
        if let Some(nombre_cliente_some) = nombre_cliente {
            let text_param = format!("nombre_cliente LIKE ?{}", numero_parametro);
            let valor_busqueda = format!("%{}%", &nombre_cliente_some);
            let caja: Box<String> = Box::new(valor_busqueda);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
        if let Some(nota_some) = nota{
            let text_param = format!("nota LIKE ?{}", numero_parametro);
            let valor_busqueda = format!("%{}%", &nota_some);
            let caja: Box<String> = Box::new(valor_busqueda);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
        if let Some(total_desde_some) = total_desde{
            let text_param = format!("total >= ?{}", numero_parametro);
            let caja: Box<f64> = Box::new(total_desde_some);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
        if let Some(total_hasta_some) = total_hasta{
            let text_param = format!("total <= ?{}", numero_parametro);
            let caja: Box<f64> = Box::new(total_hasta_some);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
        if let Some(fecha_inicio_some) = fecha_inicio {
            let fecha_utc: String = convertir_local_a_utc(&fecha_inicio_some)?;
            let text_param = format!("fecha_hora_pedido >= ?{}", numero_parametro);
            let caja: Box<String> = Box::new(fecha_utc);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }

        if let Some(fecha_fin_some) = fecha_fin {
            let fecha_utc = convertir_local_a_utc(&fecha_fin_some)?;
            let text_param = format!("fecha_hora_pedido <= ?{}", numero_parametro);
            let caja: Box<String> = Box::new(fecha_utc);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
    }

    let where_clause: String = if vec_campos_where.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", vec_campos_where.join(" AND ")) //AND no se pone al inicio ni al final porque se pone entre elementos
    };
    //Finalmente poner el ORDER BY y el limit y offset
    let final_string = format!("ORDER BY fecha_hora_pedido DESC LIMIT ?{} OFFSET ?{}", numero_parametro, numero_parametro + 1);

    comando = format!("{} {} {}", comando, where_clause, final_string);

    let caja_limit: Box<i8> = Box::new(PAGINACION_50_TAMANO);
    let caja_limit_as: Box<dyn ToSql> = caja_limit as Box<dyn rusqlite::ToSql>;
    parametros.push(caja_limit_as);
    let caja_offset: Box<i64> = Box::new(pagina);
    let caja_offset_as: Box<dyn ToSql> = caja_offset as Box<dyn rusqlite::ToSql>;
    parametros.push(caja_offset_as);

    let mut stmt = conn.prepare(&comando).map_err(|e| e.to_string())?;
    let res_map = stmt.query_map(rusqlite::params_from_iter(parametros), |row|{
        let identifier_i64: i64 = row.get::<&str, i64>("id")?;
        let total: f64 = row.get::<&str, f64>("total")?;
        let tipo_pedido_lower: String = row.get::<&str, String>("tipo_pedido")?;
        let nombre_cliente: Option<String> = row.get::<&str, Option<String>>("nombre_cliente")?;
        let nota: Option<String> = row.get::<&str, Option<String>>("nota")?;
        let estatus_lower: String = row.get::<&str, String>("estatus")?;
        let fecha_hora_pedido: String = row.get::<&str, String>("fecha_hora_pedido")?;

        let tipo_pedido: String = capitalizar(&tipo_pedido_lower);
        let estatus: String = capitalizar(&estatus_lower);
        let enum_info_tipo_pedido: InfoTipoPedido = Pedido::create_new_info_tipo_pedido_by_name(&tipo_pedido);
        let enum_estado_pedido: EstadoPedido = Pedido::create_new_estado_pedido_by_name(&estatus);
        let enum_tipo_pedido_enum: TipoPedido = Pedido::create_new_tipo_pedido_by_name(&tipo_pedido);
        let pedidito = Pedido{id: identifier_i64 as u64, total: total, tipo: enum_tipo_pedido_enum, estado: enum_estado_pedido, nota: nota, nombre_cliente: nombre_cliente, fecha_hora: fecha_hora_pedido, platillos_pedidos: Vec::new(), info_tipo_pedido: enum_info_tipo_pedido};
        Ok(pedidito)
    }).map_err(|e: rusqlite::Error| e.to_string())?;
    
    for result in res_map {
        let Ok(mut obj) = result else {
            continue;
        };
        //Obtener los platillos pedidos para cada pedido
        obj.platillos_pedidos = obtener_platillos_pedidos_by_pedido_id(obj.id)?;
        //Ahora obtener la info del tipo de pedido, ya sea local, domicilio o recoger
        obj.info_tipo_pedido = obetener_info_tipo_pedido_by_pedido_id(obj.id, &obj.tipo)?;
        pedidos.push(obj);
    }

    Ok(pedidos)

}

#[tauri::command]
pub fn count_ordenes(id: Option<u64>,tipo_pedido: Option<TipoPedido>,nombre_cliente: Option<String>,fecha_inicio: Option<String>,fecha_fin: Option<String>,total_desde: Option<f64>,total_hasta: Option<f64>,nota: Option<String>,estatus: Option<EstadoPedido>,) -> Result<i64, String> {
    let conn: Connection = obtener_conexion_db()?;
    let mut comando: String = String::from("SELECT COUNT(*) FROM pedidos");
    let mut parametros: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    let mut numero_parametro: i32 = 1;
    let mut vec_campos_where: Vec<String> = vec![];

    if let Some(estatus_some) = estatus {
        let str_some: String = estatus_some.to_string();
        let lower: String = descapitalizar(&str_some);
        let text_param = format!("estatus = ?{}", numero_parametro);
        let caja: Box<String> = Box::new(lower);
        let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
        parametros.push(caja_as);
        numero_parametro += 1;
        vec_campos_where.push(text_param);
    }

    if let Some(id_some) = id {
        let s: String = format!("id = ?{}", numero_parametro);
        numero_parametro += 1;
        parametros.push(Box::new(id_some as i64) as Box<dyn rusqlite::ToSql>);
        vec_campos_where.push(s);
    } else {
        if let Some(tipo_pedido_some) = tipo_pedido {
            let str_some: String = tipo_pedido_some.to_string();
            let lower: String = descapitalizar(&str_some);
            let text_param = format!("tipo_pedido = ?{}", numero_parametro);
            let caja: Box<String> = Box::new(lower);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
        if let Some(nombre_cliente_some) = nombre_cliente {
            let text_param = format!("nombre_cliente LIKE ?{}", numero_parametro);
            let valor_busqueda = format!("%{}%", &nombre_cliente_some);
            let caja: Box<String> = Box::new(valor_busqueda);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
        if let Some(nota_some) = nota {
            let text_param = format!("nota LIKE ?{}", numero_parametro);
            let valor_busqueda = format!("%{}%", &nota_some);
            let caja: Box<String> = Box::new(valor_busqueda);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
        if let Some(total_desde_some) = total_desde {
            let text_param = format!("total >= ?{}", numero_parametro);
            let caja: Box<f64> = Box::new(total_desde_some);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
        if let Some(total_hasta_some) = total_hasta {
            let text_param = format!("total <= ?{}", numero_parametro);
            let caja: Box<f64> = Box::new(total_hasta_some);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
        if let Some(fecha_inicio_some) = fecha_inicio {
            let fecha_utc = convertir_local_a_utc(&fecha_inicio_some)?;
            let text_param = format!("fecha_hora_pedido >= ?{}", numero_parametro);
            let caja: Box<String> = Box::new(fecha_utc);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }

        if let Some(fecha_fin_some) = fecha_fin {
            let fecha_utc = convertir_local_a_utc(&fecha_fin_some)?;
            let text_param = format!("fecha_hora_pedido <= ?{}", numero_parametro);
            let caja: Box<String> = Box::new(fecha_utc);
            let caja_as: Box<dyn ToSql> = caja as Box<dyn rusqlite::ToSql>;
            parametros.push(caja_as);
            numero_parametro += 1;
            vec_campos_where.push(text_param);
        }
    }

    let where_clause = if vec_campos_where.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", vec_campos_where.join(" AND "))
    };

    comando = format!("{} {}", comando, where_clause);

    let mut stmt = conn.prepare(&comando).map_err(|e| e.to_string())?;
    let count: i64 = stmt
        .query_row(rusqlite::params_from_iter(parametros), |row| row.get(0))
        .map_err(|e| e.to_string())?;

    Ok(count)
}

fn obtener_platillos_pedidos_by_pedido_id(pedido_id: u64) -> Result<Vec<PedidoPlatillo>, String> {
    let conn: Connection = obtener_conexion_db()?;
    let mut platillos: Vec<PedidoPlatillo> = Vec::new();
    let comando: &str = "SELECT id, name, precio, platillo_id, category_id FROM platillos_pedidos WHERE pedido_id = ?";
    let parametros = rusqlite::params![pedido_id as i64];

    let mut stmt = conn.prepare(comando).map_err(|e| e.to_string())?;
    let res_map = stmt.query_map(parametros, |row|{
        let identifier_i64: i64 = row.get::<&str, i64>("id")?;
        let name: String = row.get::<&str, String>("name")?;
        let precio: f64 = row.get::<&str, f64>("precio")?;
        let platillo_id: i64 = row.get::<&str, i64>("platillo_id")?;
        let category_id: Option<i64> = row.get::<&str, Option<i64>>("category_id")?;

        let definitive_category_id: u64 = category_id.unwrap_or(0) as u64;

        let platillo_pedido = PedidoPlatillo{id: identifier_i64 as u64, name: name, precio: precio, platillo_id: platillo_id as u64, categoria_id: definitive_category_id, pedido_id: pedido_id, platillo: None};
        Ok(platillo_pedido)

    }).map_err(|e|e.to_string())?;

    for result in res_map{
        let Ok(mut obj) = result else {
            continue;
        };
        let platillo_opt: Option<Platillo> = get_platillo_by_id(obj.platillo_id)?;
        obj.platillo = platillo_opt;
        platillos.push(obj);
    }

    Ok(platillos)
}

fn obetener_info_tipo_pedido_by_pedido_id(pedido_id: u64, tipo_pedido: &TipoPedido) -> Result<InfoTipoPedido, String>{
    let conn: Connection = obtener_conexion_db()?;
    let params = rusqlite::params![pedido_id as i64];
    let enum_info: Result<InfoTipoPedido, String> = match tipo_pedido {
        TipoPedido::Local=>{
            let comando: &str = "SELECT id, piso, mesa, mesero FROM pedidos_local WHERE pedido_id = ?";
            let res: Result<PedidoLocal, rusqlite::Error> = conn.query_row(comando, params, |row|{
                let identifier_i64: i64 = row.get::<&str, i64>("id")?;
                let piso: i64 = row.get::<&str, i64>("piso")?;
                let mesa: i64 = row.get::<&str, i64>("mesa")?;
                let mesero: String = row.get::<&str, String>("mesero")?;
                let pedido_local = PedidoLocal{id: identifier_i64 as u64, piso: piso, mesa: mesa as u64, mesero: mesero, pedido_id: pedido_id};
                Ok(pedido_local)
            });
            match res {
                Ok(pedido_local) => Ok(InfoTipoPedido::PedidoLocal(pedido_local)),
                Err(e) => Err(e.to_string()),
            }
        }
        TipoPedido::Domicilio=>{
            let comando: &str = "SELECT id, calle, numero_interior_exterior, colonia, telefono, repartidor FROM pedidos_domicilio WHERE pedido_id = ?";
            let res: Result<PedidoDomicilio, rusqlite::Error> = conn.query_row(comando, params, |row|{
                let identifier_i64: i64 = row.get::<&str, i64>("id")?;
                let calle: Option<String> = row.get::<&str, Option<String>>("calle")?;
                let numero_interior_exterior: Option<i64> = row.get::<&str, Option<i64>>("numero_interior_exterior")?;
                let colonia: Option<String> = row.get::<&str, Option<String>>("colonia")?;
                let telefono: Option<String> = row.get::<&str, Option<String>>("telefono")?;
                let repartidor: Option<String> = row.get::<&str, Option<String>>("repartidor")?;
                let pedido_domicilio = PedidoDomicilio{pedido_id: pedido_id, id: identifier_i64 as u64, calle, numero_interior_exterior, colonia, telefono, repartidor};
                Ok(pedido_domicilio)
            });
            match res {
                Ok(pedido_domicilio) => Ok(InfoTipoPedido::PedidoDomicilio(pedido_domicilio)),
                Err(e) => Err(e.to_string()),
            }
        }
        TipoPedido::Recoger=>{
            let comando : &str = "SELECT id FROM pedidos_recoger WHERE pedido_id = ?";
            let res: Result<PedidoRecoger, rusqlite::Error> = conn.query_row(comando, params, |row|{
                let identifier_i64: i64 = row.get::<&str, i64>("id")?;
                let pedido_recoger = PedidoRecoger{id: identifier_i64 as u64, pedido_id: pedido_id};
                Ok(pedido_recoger)
            });
            match res {
                Ok(pedido_recoger) => Ok(InfoTipoPedido::PedidoRecoger(pedido_recoger)),
                Err(e) => Err(e.to_string()),
            }
        }
    };
    let info: InfoTipoPedido = enum_info?;
    Ok(info)
}

#[tauri::command]
pub fn guardar_pedido_compartido(clave: String, valor: Pedido, pedido: State<'_, PedidoCompartido>) -> Result<(), String> {
    let resultado_lock = pedido.0.lock();
    let mut mapa = resultado_lock.map_err(|e| e.to_string())?;
    mapa.insert(clave, valor);
    Ok(())
}

#[tauri::command]
pub fn obtener_pedido_compartido(clave: String, pedido: State<'_, PedidoCompartido>,) -> Result<Option<Pedido>, String> {
    let resultado_lock = pedido.0.lock();
    let mapa = resultado_lock.map_err(|e| e.to_string())?;
    let valor: Option<&Pedido> = mapa.get(&clave);
    let valor_clonado: Option<Pedido> = valor.cloned();
    Ok(valor_clonado)
}