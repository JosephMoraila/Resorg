use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

pub fn imprimir_raw_linux(device_path: &str, datos: &[u8]) -> Result<(), String> {
    let mut archivo = OpenOptions::new()
        .write(true)
        .open(device_path)
        .map_err(|e| format!("Error abriendo dispositivo de impresora ({}): {}", device_path, e))?;

    // Le damos tiempo a la impresora de terminar su propia inicialización
    // interna justo después de abrir el dispositivo, antes de mandarle
    // el primer bloque grande de datos (como una imagen).
    sleep(Duration::from_millis(300));

    archivo
        .write_all(datos)
        .map_err(|e| format!("Error escribiendo datos a la impresora: {}", e))?;

    archivo
        .flush()
        .map_err(|e| format!("Error al vaciar el buffer de escritura: {}", e))?;

    Ok(())
}