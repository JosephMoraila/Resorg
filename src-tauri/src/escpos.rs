//! Contiene las funciones para extraer y guardar el HTML de los elementos escpos así como tambien la impresion de escpos para Linux y Windows
//! El de canvas que sería impresión normal está en printing en printing > printing_canvas_windows.rs y las funciones para obtener Canvas en el modulo canvas

#[cfg(target_os = "windows")]
mod escpos_windows;
#[cfg(target_os = "windows")]
pub use crate::escpos::escpos_windows::*;

#[cfg(target_os = "linux")]
mod escpos_linux;
#[cfg(target_os = "linux")]
pub use crate::escpos::escpos_linux::*;

mod escpos_commands;
pub use crate::escpos::escpos_commands::*;

mod save_load_escpos_html;
pub use crate::escpos::save_load_escpos_html::*;

mod escpos_image;
pub use crate::escpos::escpos_image::*;