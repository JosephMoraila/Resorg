use windows::Win32::Foundation::{BOOL, SIZE, COLORREF};
use windows::Win32::Graphics::Gdi::*;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Graphics::Imaging::{IWICImagingFactory, IWICFormatConverter, CLSID_WICImagingFactory,WICDecodeMetadataCacheOnLoad,WICBitmapDitherTypeNone,WICBitmapPaletteTypeCustom,GUID_WICPixelFormat32bppBGR,};
use windows::Win32::Storage::Xps::{DOCINFOA, StartDocA, EndDoc, StartPage, EndPage};
use std::ffi::CString;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, CoCreateInstance,COINIT_APARTMENTTHREADED, CLSCTX_INPROC_SERVER,};
use windows::Win32::Graphics::Gdi::{GetTextExtentPoint32A, SelectObject};

/// Espacio extra entre líneas, en las mismas unidades que text_size.
/// Debe coincidir con el valor usado en draw_text para que measure_text_height
/// sea consistente con lo que realmente se dibuja.
const LINE_SPACING: i32 = 4;

pub fn to_wide(s: &str) -> Vec<u16> {
    let osstr: &OsStr = OsStr::new(s);
    let encoded = osstr.encode_wide();
    let chained = encoded.chain(std::iter::once(0)); // null terminator
    let converted: Vec<u16> = chained.collect();
    converted
}
