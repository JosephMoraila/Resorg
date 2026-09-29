use crate::printer::{get_printting_settings};
use crate::ticket::{get_ticket_measurement};
use crate::canvas::{CanvasElement, get_canvas, TypeElement};
use crate::escpos::{escpos_html_escpos_elements, EscposElement, escposelement_to_bytes, EscposInfoElement};
#[cfg(target_os = "windows")]
use crate::escpos::imprimir_raw_windows;
#[cfg(target_os = "linux")]
use crate::escpos::imprimir_raw_linux;
#[cfg(target_os = "windows")]
use crate::printing::{print_canvas_windows};

pub fn print_cobrar(texto_info: String)-> Result<(), String> {
    let (printer_name, _ , is_print_cobrar) = get_printting_settings()?;

    if is_print_cobrar{
        let (width, _, is_canvas) = get_ticket_measurement()?;
        if is_canvas{
            print_canvas_cobrar(texto_info, &printer_name)?;
        }else{//Escpos, aqui es diferente
            let escpos_elements: Vec<EscposElement> = escpos_html_escpos_elements()?;
            //Cambiamos el ejemplo de info por la info verdadera
            let replaced_info_elements: Vec<EscposElement> = EscposInfoElement::replace_info(escpos_elements, texto_info);
            let bytes_escpos_elements: Vec<u8> = escposelement_to_bytes(&replaced_info_elements, width)?;
            #[cfg(target_os = "windows")]
            let _ = imprimir_raw_windows(&printer_name, &bytes_escpos_elements)?;
            #[cfg(target_os = "linux")]
            let _ = imprimir_raw_linux(&printer_name, &bytes_escpos_elements)?;
        }
    }

    Ok(())
}

fn print_canvas_cobrar(texto_info: String, printer_name: &str)-> Result<(), String> {
    let mut elements: Vec<CanvasElement> = get_canvas()?;
    //Reemplazamos el info si es que hay del ejemplo por el real mandado por el frondend
    for el in &mut elements{
        if let TypeElement::Info(tipo_info) = &mut el.element{
            let cloned_text_info: String = texto_info.clone();
            tipo_info.replace_text(cloned_text_info);
        }
    }
    #[cfg(target_os = "windows")]{
        print_canvas_windows(elements, printer_name)?;
    }
    #[cfg(not(target_os = "windows"))]{
        Err(String::from("Canvas no disponible en este sistema operativo"))
    }
    Ok(())
}