//! Contiene en printing_canvas_windows la función para mandar elementos de Canvas e imprimirlos en Windows como impresora normal, para escpos esta en escpos > escpos_windows.rs y para Linux en escpos_linux.rs.
//! A pesar de que este modulo se llama printing solo contiene impresión de canvas para Windows, el de escpos para Linux y Windows está en el modulo escpos.
//! La función que retorna los elementos de canvas están en el modulo Canvas

mod printing_order;
pub use crate::printing::printing_order::*;
mod printing_cobrar;
pub use crate::printing::printing_cobrar::*;
mod printing_prueba;
pub use crate::printing::printing_prueba::*;

#[cfg(target_os = "windows")]
mod printing_canvas_windows;
#[cfg(target_os = "windows")]
pub use crate::printing::printing_canvas_windows::*;