//! Se encarga de la impresión de elementos del Canvas en Windows y los manda como si fuera una impresora normal a Windows, para EscPos está en escpos > escpos_windows.rs
//! el cual ese archivo tiene la función para mandar los bytes e imprimir el contenido como escpos en Windows

use crate::canvas::{CanvasElement, TypeElement};

use crate::utils::base64_to_bytes;
use crate::windows::to_wide;
use std::ffi::CString;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::{BOOL, COLORREF, SIZE};
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::Graphics::Gdi::{GetTextExtentPoint32A, SelectObject};
use windows::Win32::Graphics::Imaging::{
    CLSID_WICImagingFactory, GUID_WICPixelFormat32bppBGR, IWICFormatConverter, IWICImagingFactory,
    WICBitmapDitherTypeNone, WICBitmapPaletteTypeCustom, WICDecodeMetadataCacheOnLoad,
};
use windows::Win32::Storage::Xps::{EndDoc, EndPage, StartDocA, StartPage, DOCINFOA};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER,
    COINIT_APARTMENTTHREADED,
};

/// Espacio extra entre líneas, en las mismas unidades que text_size.
/// Debe coincidir con el valor usado en draw_text para que measure_text_height
/// sea consistente con lo que realmente se dibuja.
const LINE_SPACING: i32 = 4;

// Crea una fuente Arial del tamaño indicado. Compartida por draw_text y
// measure_text_height para que ambas midan/dibujen con la misma fuente.
unsafe fn create_font(text_size: i32) -> HFONT {
    let font_name: Vec<u16> = to_wide("Arial");
    CreateFontW(
        -text_size,
        0,
        0,
        0,
        400,
        0,
        0,
        0,
        DEFAULT_CHARSET.0 as u32,
        OUT_DEFAULT_PRECIS.0 as u32,
        CLIP_DEFAULT_PRECIS.0 as u32,
        DEFAULT_QUALITY.0 as u32,
        DEFAULT_PITCH.0 as u32,
        PCWSTR(font_name.as_ptr()),
    )
}

fn draw_text(text: &str, hdc: &HDC, text_size: i32, x: i32, y: i32) -> Result<(), String> {
    unsafe {
        let hfont: HFONT = create_font(text_size);

        let old_font = SelectObject(*hdc, hfont);
        SetBkMode(*hdc, TRANSPARENT);
        SetTextColor(*hdc, COLORREF(0x00000000));

        // separar por saltos de línea y dibujar cada una
        let line_height = text_size + LINE_SPACING;
        for (i, line) in text.lines().enumerate() {
            let line_y = y + (i as i32 * line_height);
            let line_wide = to_wide(line);
            let result: BOOL = TextOutW(*hdc, x, line_y, &line_wide[..line_wide.len() - 1]);
            if !result.as_bool() {
                SelectObject(*hdc, old_font);
                let result_delete_obj: BOOL = DeleteObject(hfont);
                if !result_delete_obj.as_bool() {
                    return Err(String::from("Error al eliminar objeto de texto"));
                }
                return Err(String::from("Error al escribir línea"));
            }
        }

        SelectObject(*hdc, old_font);
        let res_delete: BOOL = DeleteObject(hfont);
        if !res_delete.as_bool() {
            return Err(String::from("Error al eliminar fuente"));
        }

        Ok(())
    }
}

fn draw_image(
    hdc: &HDC,
    bytes: Vec<u8>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<(), String> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .ok()
            .map_err(|e| e.to_string())?;

        let factory: IWICImagingFactory =
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)
                .map_err(|e| e.to_string())?;

        let stream = factory.CreateStream().map_err(|e| e.to_string())?;
        stream
            .InitializeFromMemory(&bytes)
            .map_err(|e| e.to_string())?;

        let decoder = factory
            .CreateDecoderFromStream(&stream, std::ptr::null(), WICDecodeMetadataCacheOnLoad)
            .map_err(|e| e.to_string())?;

        let frame = decoder.GetFrame(0).map_err(|e| e.to_string())?;

        let converter: IWICFormatConverter =
            factory.CreateFormatConverter().map_err(|e| e.to_string())?;

        converter
            .Initialize(
                &frame,
                &GUID_WICPixelFormat32bppBGR,
                WICBitmapDitherTypeNone,
                None,
                0.0,
                WICBitmapPaletteTypeCustom,
            )
            .map_err(|e| e.to_string())?;

        let mut img_w = 0u32;
        let mut img_h = 0u32;
        converter
            .GetSize(&mut img_w, &mut img_h)
            .map_err(|e| e.to_string())?;

        let stride = img_w * 4;
        let mut buffer = vec![0u8; (stride * img_h) as usize];
        converter
            .CopyPixels(std::ptr::null(), stride, &mut buffer)
            .map_err(|e| e.to_string())?;

        // describir el formato de los píxeles
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: img_w as i32,
                biHeight: -(img_h as i32), // negativo = top-down (origen arriba-izquierda)
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD::default()],
        };

        // mandar píxeles directo al DC de la impresora
        let result = StretchDIBits(
            *hdc,
            x,
            y,
            width,
            height, // destino en la impresora
            0,
            0,
            img_w as i32,
            img_h as i32, // fuente (imagen completa)
            Some(buffer.as_ptr() as _),
            &bmi,
            DIB_RGB_COLORS,
            SRCCOPY,
        );

        if result == 0 {
            return Err(String::from("Error al imprimir imagen"));
        }

        CoUninitialize();

        Ok(())
    }
}

// Mide el alto real que ocupará un bloque de texto al dibujarse.
// Selecciona la MISMA fuente (familia y tamaño) que usa draw_text, mide cada
// línea con GetTextExtentPoint32A y suma sus alturas + el espaciado entre
// líneas, para que el resultado sea consistente con lo que draw_text imprime.
unsafe fn measure_text_height(hdc: &HDC, text: &str, text_size: i32) -> Result<i64, String> {
    let hfont: HFONT = create_font(text_size);
    let old_font = SelectObject(*hdc, hfont);

    let mut total_height: i64 = 0;
    let mut line_count: i64 = 0;

    for line in text.lines() {
        // GetTextExtentPoint32A no acepta strings vacíos de forma confiable
        // en todas las implementaciones; usamos un espacio como fallback.
        let measured: &str = if line.is_empty() { " " } else { line };
        let mut size = SIZE::default();

        let ok: BOOL = GetTextExtentPoint32A(*hdc, measured.as_bytes(), &mut size as *mut SIZE);

        if !ok.as_bool() {
            SelectObject(*hdc, old_font);
            let result_delete_obj: BOOL = DeleteObject(hfont);
            if !result_delete_obj.as_bool() {
                return Err(String::from("Error al eliminar objeto de texto"));
            }
            return Err(String::from("Error al medir texto"));
        }

        total_height += size.cy as i64;
        line_count += 1;
    }

    SelectObject(*hdc, old_font);
    let res_delete: BOOL = DeleteObject(hfont);
    if !res_delete.as_bool() {
        return Err(String::from("Error al eliminar fuente"));
    }

    // Sumamos el espaciado entre líneas que también usa draw_text
    // (line_count - 1 espacios, no antes de la primera línea).
    let spacing_total: i64 = (line_count.max(1) - 1) * LINE_SPACING as i64;

    Ok(total_height + spacing_total)
}

pub fn print_canvas_windows(
    canvas_elements: Vec<CanvasElement>,
    printer_name: &str,
) -> Result<(), String> {
    unsafe {
        let printer: CString = CString::new(printer_name).map_err(|e| e.to_string())?;
        //Crear el DC de la impresora
        let hdc: HDC = CreateDCA(
            PCSTR(b"WINSPOOL\0".as_ptr()),
            PCSTR(printer.as_ptr() as *const u8),
            PCSTR::null(),
            None,
        );
        let invalid_hdc: bool = hdc.is_invalid();
        if invalid_hdc {
            return Err(String::from("Error al crear handler de impresión"));
        }
        //Iniciar el documento y la página
        let doc_name: CString = CString::new("Canvas ticket Resorg").map_err(|e| e.to_string())?;
        let doc_info: DOCINFOA = DOCINFOA {
            cbSize: std::mem::size_of::<DOCINFOA>() as i32,
            lpszDocName: PCSTR(doc_name.as_ptr() as *const u8),
            lpszOutput: PCSTR::null(),
            lpszDatatype: PCSTR::null(),
            fwType: 0,
        };
        StartDocA(hdc, &doc_info);
        StartPage(hdc);

        // --- Cálculo del delta_y para elementos under_purchase_info ---
        // PurchaseInfoElement no tiene un campo de altura de diseño, así que
        // la inferimos: es la distancia entre el y del purchase info y el y
        // del elemento under_purchase_info más cercano por debajo de él.
        // Esa distancia es, implícitamente, lo que el editor reservó.
        let mut y_info: Option<i64> = None;
        let mut info_text: String = String::new();
        let mut info_text_size: i32 = 0;

        for item in &canvas_elements {
            if let TypeElement::Info(info_data) = &item.element {
                y_info = Some(item.y as i64);
                info_text = info_data.texto.clone();
                info_text_size = info_data.size as i32;
            }
        }

        let mut delta_y: i64 = 0;

        if let Some(y_info_some) = y_info {
            // Buscamos el elemento under_purchase_info más cercano por debajo
            // del purchase info; la distancia hasta él es el alto "de diseño".
            let closest_under_info_y: Option<i64> = canvas_elements
                .iter()
                .filter_map(|item| match &item.element {
                    TypeElement::Texto(text_data)
                        if text_data.is_under_info && item.y as i64 > y_info_some =>
                    {
                        Some(item.y as i64)
                    }
                    _ => None,
                })
                .min();

            if let Some(closest_y) = closest_under_info_y {
                let designed_gap: i64 = closest_y - y_info_some;
                let real_height: i64 = measure_text_height(&hdc, &info_text, info_text_size)?;
                delta_y = real_height - designed_gap;
            }
            // Si no hay ningún elemento under_purchase_info, delta_y se
            // queda en 0 porque no hay nada que reacomodar.
        }

        //Itreamos sobre los elementos
        for item in canvas_elements {
            let x: i32 = item.x as i32;

            if let TypeElement::Texto(text_data) = &item.element {
                let y: i32 = if text_data.is_under_info {
                    (item.y as i64 + delta_y) as i32
                } else {
                    item.y as i32
                };
                draw_text(&text_data.texto, &hdc, text_data.size as i32, x, y)?;
            } else if let TypeElement::Imagen(image_data) = &item.element {
                let y: i32 = item.y as i32;
                let bytes_image: Vec<u8> = base64_to_bytes(&image_data.src)?;
                draw_image(
                    &hdc,
                    bytes_image,
                    x,
                    y,
                    image_data.ancho as i32,
                    image_data.alto as i32,
                )?;
            } else if let TypeElement::Info(info_data) = &item.element {
                let y: i32 = item.y as i32;
                draw_text(&info_data.texto, &hdc, info_data.size as i32, x, y)?;
            }
        }

        EndPage(hdc);
        EndDoc(hdc);
        let res_delete_hdc: BOOL = DeleteDC(hdc);
        if !res_delete_hdc.as_bool() {
            return Err(String::from("Error al eliminar handler"));
        }
    }
    Ok(())
}
