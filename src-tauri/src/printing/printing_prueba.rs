use crate::canvas::{get_canvas, CanvasElement};
#[cfg(target_os = "linux")]
use crate::escpos::imprimir_raw_linux;
#[cfg(target_os = "windows")]
use crate::escpos::imprimir_raw_windows;
use crate::escpos::{escpos_html_escpos_elements, escposelement_to_bytes, EscposElement};
use crate::printer::get_printting_settings;
#[cfg(target_os = "windows")]
use crate::printing::print_canvas_windows;
use crate::ticket::get_ticket_measurement;

#[tauri::command]
pub fn print_prueba() -> Result<(), String> {
    let (printer_name, _, _) = get_printting_settings()?;
    let (width, _, is_canvas) = get_ticket_measurement()?;

    if is_canvas {
        print_canvas_prueba(&printer_name)?;
    } else {
        //Escpos
        let escpos_elements: Vec<EscposElement> = escpos_html_escpos_elements()?;
        let bytes_escpos_elements: Vec<u8> = escposelement_to_bytes(&escpos_elements, width)?;
        #[cfg(target_os = "windows")]
        let _ = imprimir_raw_windows(&printer_name, &bytes_escpos_elements)?;
        #[cfg(target_os = "linux")]
        let _ = imprimir_raw_linux(&printer_name, &bytes_escpos_elements)?;
    }

    Ok(())
}

fn print_canvas_prueba(printer_name: &str) -> Result<(), String> {
    let elements: Vec<CanvasElement> = get_canvas()?;
    #[cfg(target_os = "windows")]
    {
        print_canvas_windows(elements, printer_name)?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        return Err(String::from(
            "Canvas no disponible en este sistema operativo",
        ));
    }

    Ok(())
}
