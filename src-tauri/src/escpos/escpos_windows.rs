use windows::Win32::Graphics::Printing::{OpenPrinterW, ClosePrinter, StartDocPrinterW, EndDocPrinter,StartPagePrinter, EndPagePrinter, WritePrinter, DOC_INFO_1W,};
use windows::core::{PCWSTR, PWSTR};
use crate::windows::to_wide;
use windows::Win32::Foundation::{HANDLE, BOOL};


pub fn imprimir_raw_windows(printer_name: &str, datos: &[u8]) -> Result<(), String> {
    unsafe {
        let print_name_wide: Vec<u16> = to_wide(printer_name);
        let mut handle_impresora = HANDLE::default();

        // Abrir la impresora
        OpenPrinterW(PCWSTR(print_name_wide.as_ptr()),&mut handle_impresora,None,).map_err(|e| format!("Error abriendo impresora: {}", e))?;

        let mut doc_nombre: Vec<u16> = to_wide("Test ESP/POS");
        let mut tipo_datos: Vec<u16> = to_wide("RAW");

        let doc_info = DOC_INFO_1W {
            pDocName: PWSTR(doc_nombre.as_mut_ptr()),
            pOutputFile: PWSTR::null(),
            pDatatype: PWSTR(tipo_datos.as_mut_ptr()),
        };

        let resultado: u32 = StartDocPrinterW(handle_impresora, 1, &doc_info);
        if resultado == 0 {
            ClosePrinter(handle_impresora).ok();
            return Err("Error iniciando documento de impresión".to_string());
        }

        if StartPagePrinter(handle_impresora).as_bool() == false {
            let res_end_doc: BOOL = EndDocPrinter(handle_impresora);
            if !res_end_doc.as_bool(){
                return Err("Error al cerrar documento de impresora".to_string());
            }
            ClosePrinter(handle_impresora).ok();
            return Err("Error iniciando página".to_string());
        }

        let mut bytes_escritos: u32 = 0;
        let escritura_ok: BOOL = WritePrinter(
            handle_impresora,
            datos.as_ptr() as *const _,
            datos.len() as u32,
            &mut bytes_escritos,
        );

        let end_page_printer: BOOL = EndPagePrinter(handle_impresora);
        if !end_page_printer.as_bool(){
            return Err("Error al cerrar página de impresora".to_string());
        }
        let res_end_doc: BOOL = EndDocPrinter(handle_impresora);
        if !res_end_doc.as_bool(){
            return Err("Error al cerrar documento de impresora".to_string());
        }
        ClosePrinter(handle_impresora).ok();

        if escritura_ok.as_bool() == false {
            return Err("Error escribiendo datos a la impresora".to_string());
        }

        Ok(())
    }
}

