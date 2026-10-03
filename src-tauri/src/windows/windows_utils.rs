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

pub fn to_wide(s: &str) -> Vec<u16> {
    let osstr: &OsStr = OsStr::new(s);
    let encoded = osstr.encode_wide();
    let chained = encoded.chain(std::iter::once(0)); // null terminator
    let converted: Vec<u16> = chained.collect();
    converted
}
