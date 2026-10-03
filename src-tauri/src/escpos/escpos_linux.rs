use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

pub fn imprimir_raw_linux(device_path: &str, datos: &[u8]) -> Result<(), String> {
    let archivo: Result<std::fs::File, std::io::Error> =
        OpenOptions::new().write(true).open(device_path);

    let mut archivo: std::fs::File = match archivo {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Si el dispositivo no existe, ignoramos el error para no afectar el flujo del pedido
            return Ok(());
        }
        Err(e) => {
            return Err(format!(
                "Error abriendo dispositivo de impresora ({}): {}",
                device_path, e
            ))
        }
    };

    // Le damos tiempo a la impresora de terminar su propia inicialización
    sleep(Duration::from_millis(300));

    archivo
        .write_all(datos)
        .map_err(|e| format!("Error escribiendo datos a la impresora: {}", e))?;

    archivo
        .flush()
        .map_err(|e| format!("Error al vaciar el buffer de escritura: {}", e))?;

    Ok(())
}
