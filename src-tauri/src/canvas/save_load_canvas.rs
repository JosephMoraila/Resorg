use crate::path_and_files::{get_json_canvas, obetener_canvas_folder};
use crate::utils::{conversion_segura, guardar_base64_a_imagen, imagen_a_base64};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CanvasElement {
    pub id: u64,
    pub x: f64,
    pub y: f64,
    pub element: TypeElement,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextCanvasElement {
    pub texto: String,
    pub size: u32,
    pub is_under_info: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageCanvasElement {
    pub src: String,
    pub alto: u32,
    pub ancho: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfoCanvasElement {
    pub texto: String,
    pub size: u32,
}

impl InfoCanvasElement {
    pub fn replace_text(&mut self, new_text: String) {
        self.texto = new_text;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "tipo")]
pub enum TypeElement {
    Texto(TextCanvasElement),
    Imagen(ImageCanvasElement),
    Info(InfoCanvasElement),
}

#[tauri::command]
pub fn save_canvas(mut canvas_elements: Vec<CanvasElement>) -> Result<(), String> {
    let json_file: PathBuf = get_json_canvas()?;
    let canvas_folder: PathBuf = obetener_canvas_folder()?;

    for canvas_element in &mut canvas_elements {
        let id_el: u64 = canvas_element.id;
        match &mut canvas_element.element {
            TypeElement::Imagen(img_el) => {
                let nombre_imagen: String = format!("img_element_{}.png", id_el);
                let ruta_imagen: PathBuf = canvas_folder.join(&nombre_imagen);
                let ruta_imagen_string: String = conversion_segura(&ruta_imagen)?;
                guardar_base64_a_imagen(&ruta_imagen_string, &img_el.src)?;
                img_el.src = ruta_imagen_string; //Cambiamos su valor por la ruta real en la computadora despues de crear la imagen
            }
            TypeElement::Info(_) => {}
            TypeElement::Texto(_) => {}
        }
    }

    let json_string: String = serde_json::to_string_pretty(&canvas_elements)
        .map_err(|e: serde_json::Error| e.to_string())?;
    fs::write(json_file, json_string).map_err(|e: std::io::Error| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_canvas() -> Result<Vec<CanvasElement>, String> {
    let json_file: PathBuf = get_json_canvas()?;
    let existe: bool = json_file.exists();
    if existe {
        let json_string: String =
            fs::read_to_string(json_file).map_err(|e: std::io::Error| e.to_string())?;
        let mut elementos: Vec<CanvasElement> =
            serde_json::from_str(&json_string).map_err(|e| e.to_string())?;

        //Convertir ruta de imagenes a base64
        for elemento in &mut elementos {
            if let TypeElement::Imagen(ref mut img_el) = &mut elemento.element {
                let imagen_base64: String = imagen_a_base64(&img_el.src)?;
                img_el.src = imagen_base64;
            }
        }

        Ok(elementos)
    } else {
        let vacio_vector: Vec<CanvasElement> = vec![];
        Ok(vacio_vector)
    }
}
