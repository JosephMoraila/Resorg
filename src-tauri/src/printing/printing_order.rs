#[cfg(target_os = "linux")]
use crate::escpos::imprimir_raw_linux;
#[cfg(target_os = "windows")]
use crate::escpos::imprimir_raw_windows;
use crate::escpos::EscposCommands;
use crate::printer::get_printting_settings;

use crate::db::{InfoTipoPedido, Pedido, PedidoPlatillo};

pub fn print_order_escpos(text_to_show: &str) -> Result<(), String> {
    let (printer_name, is_print_order, _) = get_printting_settings()?;

    if is_print_order {
        let mut comando: Vec<u8> = Vec::new();

        // Inicializar e instruir alineación
        comando.extend_from_slice(&EscposCommands::INICIALIZAR);
        comando.extend_from_slice(&EscposCommands::CENTRAR);

        // Texto del ticket
        comando.extend_from_slice(text_to_show.as_bytes());

        // AVANCE DE PAPEL (Fundamental para que el texto pase la cuchilla)
        comando.extend_from_slice(&EscposCommands::FEED_LINEAS);
        // O alternativamente: comando.extend_from_slice(b"\n\n\n\n\n");

        // Corte de papel
        comando.extend_from_slice(&EscposCommands::CORTE_TOTAL);

        #[cfg(target_os = "windows")]
        {
            imprimir_raw_windows(&printer_name, &comando)?;
        }
        #[cfg(target_os = "linux")]
        {
            imprimir_raw_linux(&printer_name, &comando)?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn print_again_order_escpos(pedido: Pedido) -> Result<(), String> {
    let (printer_name, _, _) = get_printting_settings()?;
    let info_tipo: InfoTipoPedido = pedido.info_tipo_pedido;
    let mut text: String = match info_tipo {
        InfoTipoPedido::PedidoLocal(local) => {
            let mut text: String = format!(
                "PEDIDO LOCAL\nPiso: {} - Mesa: {}\nMesero: {}\n",
                local.piso, local.mesa, local.mesero
            );
            text
        }
        InfoTipoPedido::PedidoDomicilio(domicilio) => {
            let mut text: String = format!("PEDIDO DOMICILIO\n");
            if let Some(repartidor_some) = domicilio.repartidor.as_deref() {
                text += &format!("Repartidor: {}\n", repartidor_some);
            }
            if let Some(colonia_some) = domicilio.colonia.as_deref() {
                text += &format!("Colonia: {}\n", colonia_some);
            }
            if let Some(calle_some) = domicilio.calle.as_deref() {
                text += &format!("Calle: {}\n", calle_some);
            }
            if let Some(numero_interior_exterior_some) = domicilio.numero_interior_exterior {
                text += &format!(
                    "Número interior/exterior:: {}\n",
                    numero_interior_exterior_some
                );
            }
            if let Some(telefono_some) = domicilio.telefono.as_deref() {
                text += &format!("Número celular:: {}\n", telefono_some);
            }
            text
        }
        InfoTipoPedido::PedidoRecoger(_) => {
            let text: String = format!("PEDIDO RECOGER\n");
            text
        }
    };

    if let Some(nombre_cliente_some) = pedido.nombre_cliente.as_deref() {
        text += &format!("Nombre cliente: {}\n", nombre_cliente_some);
    }
    let nota_ref: Option<&str> = pedido.nota.as_deref();
    if let Option::Some(notas_some) = nota_ref {
        let new: String = format!("Notas: {}\n", notas_some);
        text += &new;
    }
    //Agregamos id de pedido
    text += &format!(
        "PEDIDO ID: {}\nFecha:{}\n\nPlatillos:\n\n",
        pedido.id, pedido.fecha_hora
    );
    //Agregamos platillos
    let platillos_pedidos: Vec<PedidoPlatillo> = pedido.platillos_pedidos;
    let mut lista: i32 = 1;
    for platillo_pedido in platillos_pedidos {
        let platillo_nombre: String = platillo_pedido.name;
        let precio: f64 = platillo_pedido.precio;
        text += &format!(
            "{}.- Nombre: {} - Precio: {}\n",
            lista, &platillo_nombre, precio
        );
        lista += 1;
    }

    text += "\n";

    let mut comando: Vec<u8> = Vec::new();

    // Inicializar e instruir alineación
    comando.extend_from_slice(&EscposCommands::INICIALIZAR);
    comando.extend_from_slice(&EscposCommands::CENTRAR);

    // Texto del ticket
    comando.extend_from_slice(text.as_bytes());

    // AVANCE DE PAPEL (Fundamental para que el texto pase la cuchilla)
    comando.extend_from_slice(&EscposCommands::FEED_LINEAS);
    // O alternativamente: comando.extend_from_slice(b"\n\n\n\n\n");

    // Corte de papel
    comando.extend_from_slice(&EscposCommands::CORTE_TOTAL);

    #[cfg(target_os = "windows")]
    {
        imprimir_raw_windows(&printer_name, &comando)?;
    }
    #[cfg(target_os = "linux")]
    {
        imprimir_raw_linux(&printer_name, &comando)?;
    }

    Ok(())
}
