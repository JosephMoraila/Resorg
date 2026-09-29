use crate::printer::{get_printting_settings};
use crate::ticket::{get_ticket_measurement};
#[cfg(target_os = "windows")]
use crate::printing::{print_canvas_windows};
use crate::canvas::{CanvasElement, get_canvas,};
use crate::escpos::{escpos_html_escpos_elements, EscposElement, escposelement_to_bytes};
#[cfg(target_os = "windows")]
use crate::escpos::imprimir_raw_windows;
#[cfg(target_os = "linux")]
use crate::escpos::imprimir_raw_linux;

#[tauri::command]
pub fn print_prueba()->Result<(), String>{
    let (printer_name, _ , _) = get_printting_settings()?;
    let (width, _, is_canvas) = get_ticket_measurement()?;

    if is_canvas{
        print_canvas_prueba(&printer_name)?;
    }else{//Escpos
        let escpos_elements: Vec<EscposElement> = escpos_html_escpos_elements()?;
        let bytes_escpos_elements: Vec<u8> = escposelement_to_bytes(&escpos_elements, width)?;
        #[cfg(target_os = "windows")]
        let _ = imprimir_raw_windows(&printer_name, &bytes_escpos_elements)?;
        #[cfg(target_os = "linux")]
        let _ = imprimir_raw_linux(&printer_name, &bytes_escpos_elements)?;
    }

    Ok(())
}

fn print_canvas_prueba(printer_name: &str)-> Result<(), String> {
    let elements: Vec<CanvasElement> = get_canvas()?;
    #[cfg(target_os = "windows")]{
        print_canvas_windows(elements, printer_name)?;
    }
    #[cfg(not(target_os = "windows"))]{
        return Err(String::from("Canvas no disponible en este sistema operativo"));
    }
    
    Ok(())
}