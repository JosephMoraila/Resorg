use crate::db::{EstadoPedido, MetodoPago, Pedido, TipoPedido};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FiltrosVerOrdenesProps {
    pub id: Option<u64>,
    pub tipo_pedido: Option<TipoPedido>,
    pub nombre_cliente: Option<String>,
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub total_desde: Option<f64>,
    pub total_hasta: Option<f64>,
    pub nota: Option<String>,
    pub estatus: Option<EstadoPedido>,
    pub metodo_pago: Option<MetodoPago>,
}

pub struct GraficaPedidosFiltrosCompartido(
    pub Mutex<HashMap<String, (FiltrosVerOrdenesProps, Vec<Pedido>)>>,
);

#[tauri::command]
pub fn guardar_pedidos_filtros_compartido(
    clave: String,
    pedidos: Vec<Pedido>,
    filtros: FiltrosVerOrdenesProps,
    graficas: State<'_, GraficaPedidosFiltrosCompartido>,
) -> Result<(), String> {
    let resultado_lock = graficas.0.lock();
    let mut mapa = resultado_lock.map_err(|e| e.to_string())?;
    mapa.insert(clave, (filtros, pedidos));
    Ok(())
}

#[tauri::command]
pub fn obtener_pedidos_filtros_compartido(
    clave: String,
    graficas: State<'_, GraficaPedidosFiltrosCompartido>,
) -> Result<Option<(FiltrosVerOrdenesProps, Vec<Pedido>)>, String> {
    let resultado_lock = graficas.0.lock();
    let mapa = resultado_lock.map_err(|e| e.to_string())?;
    let valor: Option<&(FiltrosVerOrdenesProps, Vec<Pedido>)> = mapa.get(&clave);
    let valor_clonado: Option<(FiltrosVerOrdenesProps, Vec<Pedido>)> = valor.cloned();
    Ok(valor_clonado)
}
