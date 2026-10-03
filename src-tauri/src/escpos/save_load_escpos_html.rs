use crate::escpos::{image_to_escpos_bytes, mm_to_pixels};
use crate::path_and_files::get_html_escpos_path;
use base64::{engine::general_purpose, Engine as _};
use image::load_from_memory;
use scraper::{Html, Selector};
use std::fs;
use std::path::PathBuf;

#[tauri::command]
pub fn save_escpos_html(contenido_html: String) -> Result<(), String> {
    let escpos_html_file_path: PathBuf = get_html_escpos_path()?;

    fs::write(escpos_html_file_path, contenido_html).map_err(|e: std::io::Error| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_escpos_html() -> Result<String, String> {
    let escpos_html_file_path: PathBuf = get_html_escpos_path()?;
    let html_escpos: String =
        fs::read_to_string(escpos_html_file_path).unwrap_or_else(|_| "<div><br></div>".to_string());
    Ok(html_escpos)
}

pub struct EscposTextElement {
    pub text: String,
}

pub struct EscposImageElement {
    pub src: String,
}

pub struct EscposInfoElement {
    pub text: String,
}

impl EscposInfoElement {
    /// Convierte un vector de EscposElement con el info del ejemplo y lo cambia por la informacion de los productos y los datos correspondientes de compra mediante new_info
    /// # Parámetros
    /// - `items_escpos` — Los items que salen del archivo de html
    pub fn replace_info(
        mut items_escpos: Vec<EscposElement>,
        new_info: String,
    ) -> Vec<EscposElement> {
        // Usamos None para saber si realmente encontramos elementos o no
        let mut inicio_purchase_info: Option<usize> = None;
        let mut fin_purchase_info: Option<usize> = None;

        // .enumerate() nos da el índice (i) de cada elemento en cada vuelta
        for (i, item) in items_escpos.iter_mut().enumerate() {
            match item.element {
                EscposElementType::Info(_) => {
                    // Si 'inicio' es None, significa que este es el PRIMER elemento que encontramos
                    if inicio_purchase_info.is_none() {
                        inicio_purchase_info = Some(i);
                    }
                    // El ÚLTIMO se va actualizando siempre.
                    fin_purchase_info = Some(i);
                }
                _ => {}
            }
        }

        // Si encontramos tanto el inicio como el fin, hacemos el reemplazo
        if let (Some(inicio), Some(fin)) = (inicio_purchase_info, fin_purchase_info) {
            let nuevo_info_element = EscposInfoElement { text: new_info };
            let reemplazo_element_type = EscposElementType::Info(nuevo_info_element);

            let reemplazo = EscposElement {
                line: inicio as u64,
                element: reemplazo_element_type,
            };

            // 1. Usamos 'inicio..=fin' (rango inclusivo, borra desde inicio hasta fin completo).
            // 2. Envolvemos 'reemplazo' en un array '[reemplazo]' para convertirlo en una colección válida para splice.
            items_escpos.splice(inicio..=fin, [reemplazo]);
        }

        items_escpos
    }
}

pub enum EscposElementType {
    Text(EscposTextElement),
    Image(EscposImageElement),
    Info(EscposInfoElement),
}

pub struct EscposElement {
    pub line: u64,
    pub element: EscposElementType,
}

/// Entrega un vector de EscposElement tal como el test
/// # Parámetros
/// - `html` — DOM del EscPos
///
/// # Correcciones aplicadas
/// 1. El selector de los divs individuales estaba mal escrito como `"body"` (copiado del
///    selector de arriba), por lo que en vez de iterar cada `<div>` por separado, se
///    iteraba una sola vez el `<body>` completo tratándolo como si fuera "el div". Sus
///    hijos directos (los `<div>` reales) nunca coincidían con las ramas `br`/`img` del
///    match, así que se ignoraban por completo y el texto nunca se recolectaba — esto
///    era la causa de que la impresión saliera en blanco. Se corrigió a `Selector::parse("div")`.
/// 2. Los marcadores de la sección de compra se comparaban contra `"[START]"`/`"[END]"`,
///    pero el HTML real usa `"[INFO_START]"`/`"[INFO_END]"`. Como nunca coincidían, esa
///    sección nunca se marcaba como `EscposElementType::Info`. Se corrigió la comparación
///    para que use los marcadores reales.
pub fn escpos_html_escpos_elements() -> Result<Vec<EscposElement>, String> {
    let html_string: String = get_escpos_html()?;
    let document: Html = Html::parse_document(&html_string);
    let mut elements: Vec<EscposElement> = Vec::new();
    let mut line: u64 = 0;

    let body_selector: Selector = Selector::parse("body").map_err(|e| e.to_string())?;
    //Texto suelto fuera de los divs
    let mut iterador_body = document.select(&body_selector);
    let primer_body = iterador_body.next(); //Solo el primer elemento que es body que solo hay uno
    if let Some(body) = primer_body {
        let hijos = body.children(); //Cada hijo puede ser element o text
        for nodo in hijos {
            let nodo_valor = nodo.value(); //Tomamos de que tipo es el nodo
            let only_text = nodo_valor.as_text(); //Solo tomamos el texto directo dentro de div y si lo es es Some, si es un span, p  u etiqueta es None
            if let Some(text) = only_text {
                let trimmed: &str = text.trim(); //Como puede tener espacios y saltos a los lados lo quitamos
                if !trimmed.is_empty() {
                    //Si no está vacío
                    let text_element = EscposTextElement {
                        text: trimmed.to_string(),
                    };
                    let type_text = EscposElementType::Text(text_element);
                    let escpos_element = EscposElement {
                        element: type_text,
                        line,
                    };
                    elements.push(escpos_element);
                    line += 1;
                }
            }
        }
    }

    //Ahora una iteracion para cada div
    // antes decía Selector::parse("body") por error de tipeo — debe ser "div"
    // para iterar cada <div> individual del documento, no el <body> completo.
    let div_selector: Selector = Selector::parse("div").map_err(|e| e.to_string())?;
    let mut iterador_divs = document.select(&div_selector);
    let mut in_purchase = false;
    for div in iterador_divs {
        let mut raw_text = String::new();
        // Procesa y vacía el texto acumulado hasta ahora, respetando [INFO_START]/[INFO_END]
        let flush = |raw_text: &mut String,
                     in_purchase: &mut bool,
                     line: &mut u64,
                     elements: &mut Vec<EscposElement>| {
            let lineas = raw_text.lines();
            for text_line in lineas {
                //Si el text tiene \n los separamos por líneas, si no tiene se toma esa linea
                let trimmed: &str = text_line.trim();

                if trimmed.is_empty() {
                    let element_text = EscposTextElement {
                        text: String::new(),
                    };
                    let element_type = EscposElementType::Text(element_text);
                    let escpos_element = EscposElement {
                        line: *line,
                        element: element_type,
                    };
                    elements.push(escpos_element);
                    *line += 1;
                    continue;
                }

                if trimmed == "[INFO_START]" {
                    println!("MARCADOR INFO_START encontrado en línea {}", line);
                    *in_purchase = true;
                    continue;
                }
                if trimmed == "[INFO_END]" {
                    *in_purchase = false;
                    continue;
                }

                if *in_purchase {
                    let element_info = EscposInfoElement {
                        text: trimmed.to_string(),
                    };
                    let element_type = EscposElementType::Info(element_info);
                    let espos_element = EscposElement {
                        line: *line,
                        element: element_type,
                    };
                    elements.push(espos_element);
                } else {
                    let element_text = EscposTextElement {
                        text: trimmed.to_string(),
                    };
                    let element_type = EscposElementType::Text(element_text);
                    let espos_element = EscposElement {
                        line: *line,
                        element: element_type,
                    };
                    elements.push(espos_element);
                }
                *line += 1;
            }
            raw_text.clear();
        };

        let div_children = div.children(); //iteramos por cada hijo de div que puede ser otro element o un text
        for child in div_children {
            let children_value = child.value(); //Si es text o element
            let children_text = children_value.as_text(); //Solo tomamos el texto directo dentro de div y si lo es es Some, si es un span, p  u etiqueta es None
            let children_element = children_value.as_element(); //Si es una etiqueta HTML como <br>, span o cualquier otro es Some, de lo contario es None

            if let Some(text) = children_text {
                raw_text.push_str(text);
            } else if let Some(elem) = children_element {
                if elem.name() == "br" {
                    //Si la etiqueta es un br que es una linea en blanco
                    raw_text.push('\n');
                } else if elem.name() == "img" {
                    // Vacía el texto acumulado ANTES de esta imagen,
                    // para respetar el orden real del HTML.
                    flush(&mut raw_text, &mut in_purchase, &mut line, &mut elements);
                    let real_html_tag = scraper::ElementRef::wrap(child); //Con wrap es option si es un etiqueta html y no otra cosa, en este caso u  img
                    if let Some(el) = real_html_tag {
                        let el_value = el.value(); //Su elemento que puede ser un tag html o cualquier otra cosa, en img es Element
                        let src_img: Option<&str> = el_value.attr("src"); //En este caso solo queremos obtener el base64 que esta dentro de img
                        let value_str: &str = src_img.unwrap_or(""); //Si es None o no tiene nada vacío
                        let value_src_string: String = value_str.to_string();
                        if !value_src_string.is_empty() {
                            //Si tiene algún valor
                            let element_img = EscposImageElement {
                                src: value_src_string,
                            };
                            let element_type = EscposElementType::Image(element_img);
                            let escpos_element = EscposElement {
                                element: element_type,
                                line,
                            };
                            elements.push(escpos_element);
                            line += 1;
                        }
                    }
                }
            }
        }

        // Vacía lo que quede después del último nodo
        flush(&mut raw_text, &mut in_purchase, &mut line, &mut elements);
    }

    Ok(elements)
}

pub fn escposelement_to_bytes(
    items_escpos: &Vec<EscposElement>,
    width_mm: u32,
) -> Result<Vec<u8>, String> {
    let mut bytes_finales: Vec<u8> = Vec::new();
    let printer_width: u32 = mm_to_pixels(width_mm as f32, 180);

    // Inicializar impresora ESC @
    bytes_finales.extend_from_slice(&[0x1B, 0x40]);
    //bytes_finales.extend_from_slice(&[0x1B, 0x74, 0x10]); // 0x10 = PC858 (Latin/Euro)

    for item in items_escpos {
        match &item.element {
            EscposElementType::Text(texto) => {
                bytes_finales.extend(texto.text.as_bytes());
                bytes_finales.push(b'\n');
            }
            EscposElementType::Image(img_elem) => {
                let datos_base64 = img_elem.src.split(',').last().unwrap_or(&img_elem.src);

                if let Ok(bytes_img) = general_purpose::STANDARD.decode(datos_base64) {
                    if let Ok(img) = load_from_memory(&bytes_img) {
                        let img_bytes = image_to_escpos_bytes(&img, true, true, printer_width);
                        bytes_finales.extend(img_bytes);
                    } else {
                        return Err(format!("Error cargando imagen en línea {}", item.line));
                    }
                } else {
                    return Err(format!("Error decodificando base64 en línea {}", item.line));
                }
            }
            EscposElementType::Info(info) => {
                bytes_finales.extend(info.text.as_bytes());
                bytes_finales.push(b'\n');
            }
        }
    }

    // Cortar papel (opcional)
    bytes_finales.extend_from_slice(&[0x1D, 0x56, 0x41, 0x00]);

    Ok(bytes_finales)
}
