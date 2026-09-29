#[cfg(target_os = "windows")]
use crate::escpos::{imprimir_raw_windows};
#[cfg(target_os = "linux")]
use crate::escpos::imprimir_raw_linux;
use crate::printer::{get_printting_settings};
use crate::escpos::EscposCommands;

pub fn print_order_escpos(text_to_show: &str) -> Result<(), String> {
    let (printer_name, is_print_order,_) = get_printting_settings()?;

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