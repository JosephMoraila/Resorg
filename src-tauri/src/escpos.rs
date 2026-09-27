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