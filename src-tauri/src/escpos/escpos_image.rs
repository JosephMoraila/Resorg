use image::{DynamicImage, GenericImageView, imageops::FilterType};

// ─────────────────────────────────────────────
// Detectar ancho máximo según nombre de impresora
// ─────────────────────────────────────────────
pub fn detect_printer_width(printer_name: &str) -> u32 {
    let lower = printer_name.to_lowercase();

    if lower.contains("58mm") || lower.contains("58") || lower.contains("rp58") {
        return 384; // 58mm @ 203 DPI
    }

    if lower.contains("80mm")
        || lower.contains("80")
        || lower.contains("rp80")
        || lower.contains("tm-t88")
        || lower.contains("tm-t20")
    {
        return 576; // 80mm @ 203 DPI
    }

    576 // defecto: 80mm
}

// ─────────────────────────────────────────────
// Ancho óptimo sin exceder el máximo de la impresora
// ─────────────────────────────────────────────
pub fn get_optimal_width(img_width: u32, printer_max_width: u32) -> u32 {
    if img_width <= printer_max_width {
        img_width
    } else {
        printer_max_width
    }
}

// ─────────────────────────────────────────────
// Conversión simple por umbral fijo
// ─────────────────────────────────────────────
pub fn convert_to_raster_simple(img: &DynamicImage, threshold: u8, target_width: u32) -> Vec<u8> {
    let work_img = if target_width > 0 && img.width() != target_width {
        let new_height = (img.height() * target_width) / img.width();
        img.resize_exact(target_width, new_height, FilterType::Lanczos3)
    } else {
        img.clone()
    };

    let width = work_img.width() as usize;
    let height = work_img.height() as usize;
    let width_bytes = (width + 7) / 8;
    let rgba = work_img.to_rgba8();

    let mut raster_data: Vec<u8> = Vec::with_capacity(width_bytes * height);

    for y in 0..height {
        for x_byte in 0..width_bytes {
            let mut byte: u8 = 0;

            for bit in 0..8 {
                let x = x_byte * 8 + bit;
                if x < width {
                    let pixel = rgba.get_pixel(x as u32, y as u32);
                    let r = pixel[0];
                    let g = pixel[1];
                    let b = pixel[2];
                    let a = pixel[3];

                    // Píxel transparente = blanco (no imprimir)
                    let gray = if a == 0 {
                        255u8
                    } else {
                        ((r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000) as u8
                    };

                    if gray < threshold {
                        byte |= 1 << (7 - bit);
                    }
                }
            }

            raster_data.push(byte);
        }
    }

    raster_data
}

// ─────────────────────────────────────────────
// Conversión con dithering Floyd-Steinberg
// ─────────────────────────────────────────────
pub fn convert_to_raster_dithered(img: &DynamicImage, target_width: u32) -> Vec<u8> {
    let work_img = if target_width > 0 && img.width() != target_width {
        let new_height = (img.height() * target_width) / img.width();
        img.resize_exact(target_width, new_height, FilterType::Lanczos3)
    } else {
        img.clone()
    };

    let width = work_img.width() as usize;
    let height = work_img.height() as usize;
    let width_bytes = (width + 7) / 8;
    let rgba = work_img.to_rgba8();

    // Convertir a escala de grises en buffer temporal (i32 para absorber errores negativos)
    let mut gray_buffer: Vec<i32> = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let pixel = rgba.get_pixel(x as u32, y as u32);
            let r = pixel[0] as i32;
            let g = pixel[1] as i32;
            let b = pixel[2] as i32;
            let a = pixel[3];

            let gray = if a == 0 {
                255 // transparente = blanco
            } else {
                (r * 299 + g * 587 + b * 114) / 1000
            };
            gray_buffer.push(gray);
        }
    }

    // Floyd-Steinberg dithering
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let old_pixel = gray_buffer[idx];
            let new_pixel = if old_pixel < 128 { 0 } else { 255 };
            gray_buffer[idx] = new_pixel;

            let error = old_pixel - new_pixel;

            // Distribuir error a píxeles vecinos
            if x + 1 < width {
                gray_buffer[idx + 1] = (gray_buffer[idx + 1] + error * 7 / 16).clamp(0, 255);
            }
            if y + 1 < height {
                if x > 0 {
                    gray_buffer[idx + width - 1] = (gray_buffer[idx + width - 1] + error * 3 / 16).clamp(0, 255);
                }
                gray_buffer[idx + width] = (gray_buffer[idx + width] + error * 5 / 16).clamp(0, 255);
                if x + 1 < width {
                    gray_buffer[idx + width + 1] = (gray_buffer[idx + width + 1] + error * 1 / 16).clamp(0, 255);
                }
            }
        }
    }

    // Convertir a formato raster
    let mut raster_data: Vec<u8> = Vec::with_capacity(width_bytes * height);
    for y in 0..height {
        for x_byte in 0..width_bytes {
            let mut byte: u8 = 0;
            for bit in 0..8 {
                let x = x_byte * 8 + bit;
                if x < width {
                    if gray_buffer[y * width + x] < 128 {
                        byte |= 1 << (7 - bit);
                    }
                }
            }
            raster_data.push(byte);
        }
    }

    raster_data
}

// ─────────────────────────────────────────────
// Crear comando raster GS v 0
// ─────────────────────────────────────────────
pub fn create_raster_command(width: usize, height: usize, raster_data: &[u8], density: u8) -> Vec<u8> {
    let width_bytes = (width + 7) / 8;
    let mut cmd: Vec<u8> = Vec::with_capacity(8 + raster_data.len());

    // GS v 0
    cmd.push(0x1D); // GS
    cmd.push(0x76); // v
    cmd.push(0x30); // 0
    cmd.push(density);

    // Ancho en bytes (little-endian)
    cmd.push((width_bytes & 0xFF) as u8);
    cmd.push(((width_bytes >> 8) & 0xFF) as u8);

    // Alto en píxeles (little-endian)
    cmd.push((height & 0xFF) as u8);
    cmd.push(((height >> 8) & 0xFF) as u8);

    // Datos de imagen
    cmd.extend_from_slice(raster_data);

    cmd
}

/// Convierte milímetros a píxeles de impresora térmica.
/// La mayoría de impresoras térmicas usan 203 DPI estándar.
///
/// # Parámetros
/// - `mm` — Ancho en milímetros (ej: 58.0 para impresoras de 58mm)
/// - `dpi` — Resolución de la impresora (203 es el estándar, algunos modelos usan 300)
///
/// # Ejemplo
/// ```rust
/// let pixels = mm_to_pixels(58.0, 203); // → 463
/// let pixels = mm_to_pixels(80.0, 203); // → 639
/// ```
pub fn mm_to_pixels(mm: f32, dpi: u32) -> u32 {
    ((mm * dpi as f32) / 25.4).round() as u32
}

// ─────────────────────────────────────────────
// Función principal: imagen → bytes ESC/POS
// ─────────────────────────────────────────────
/// Convierte una imagen a bytes de comandos ESC/POS listos para enviar a una impresora térmica.
///
/// # Parámetros
/// - `img` — Imagen a convertir (cualquier formato soportado por el crate `image`)
/// - `use_dithering` — Si `true`, usa Floyd-Steinberg para mejor calidad (fotos/degradados);
///   si `false`, usa umbral fijo (más rápido, mejor para logos y texto)
/// - `center_image` — Si `true`, agrega comandos ESC/POS para centrar la imagen
/// - `max_width` — Ancho máximo en píxeles (`0` = usar ancho original de la imagen).
///   Valores comunes: `384` para impresoras 58mm, `576` para 80mm
///
/// # Retorna
/// `Vec<u8>` con los bytes ESC/POS listos para enviar directamente a la impresora.
/// Incluye el comando `GS v 0` (raster bit image), alineación y saltos de línea finales.
/// Si la imagen supera 1024px de alto, se fragmenta automáticamente.
///
/// # Ejemplo
/// ```rust
/// let img = image::open("logo.png").unwrap();
/// let bytes = image_to_escpos_bytes(&img, false, true, 576);
/// imprimir_raw_windows("EPSON TM-T20", &bytes).unwrap();
/// ```
pub fn image_to_escpos_bytes(img: &DynamicImage,use_dithering: bool,center_image: bool,
    max_width: u32,         // 0 = usar ancho original
) -> Vec<u8> {
    let mut commands: Vec<u8> = Vec::new();

    // Centrar imagen
    if center_image {
        commands.extend_from_slice(&[0x1B, 0x61, 0x01]); // ESC a 1
    }

    let target_width: u32 = if max_width > 0 { max_width } else { img.width() };

    // Redimensionar si es necesario
    let work_img: DynamicImage = if img.width() > target_width {
        let new_height: u32 = (img.height() * target_width) / img.width();
        img.resize_exact(target_width, new_height, FilterType::Lanczos3)
    } else {
        img.clone()
    };

    let final_width = work_img.width() as usize;
    let final_height = work_img.height() as usize;

    // Convertir a raster
    let raster_data = if use_dithering {
        convert_to_raster_dithered(&work_img, 0)
    } else {
        convert_to_raster_simple(&work_img, 128, 0)
    };

    // Fragmentar si supera 1024px de alto
    const MAX_HEIGHT: usize = 1024;
    if final_height > MAX_HEIGHT {
        let width_bytes = (final_width + 7) / 8;
        let num_fragments = (final_height + MAX_HEIGHT - 1) / MAX_HEIGHT;

        for i in 0..num_fragments {
            let fragment_height = MAX_HEIGHT.min(final_height - i * MAX_HEIGHT);
            let start_byte = i * MAX_HEIGHT * width_bytes;
            let fragment_size = fragment_height * width_bytes;

            let fragment_data = &raster_data[start_byte..start_byte + fragment_size];
            commands.extend(create_raster_command(final_width, fragment_height, fragment_data, 0));
        }
    } else {
        commands.extend(create_raster_command(final_width, final_height, &raster_data, 0));
    }

    // Restaurar alineación izquierda
    if center_image {
        commands.extend_from_slice(&[0x1B, 0x61, 0x00]); // ESC a 0
    }

    // Saltos de línea al final
    commands.push(b'\n');
    commands.push(b'\n');

    commands
}

// ─────────────────────────────────────────────
// Versión con detección automática de impresora
// ─────────────────────────────────────────────
pub fn image_to_escpos_bytes_auto(
    img: &DynamicImage,
    printer_name: &str,
    use_dithering: bool,
    center_image: bool,
) -> Vec<u8> {
    let printer_max_width = detect_printer_width(printer_name);
    let target_width = get_optimal_width(img.width(), printer_max_width);
    image_to_escpos_bytes(img, use_dithering, center_image, target_width)
}