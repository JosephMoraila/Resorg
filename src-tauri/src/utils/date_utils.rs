use chrono::{Local, NaiveDateTime, TimeZone};

/// Convierte un string de fecha/hora en hora LOCAL del sistema
/// (formato "YYYY-MM-DD HH:MM:SS") a un string en UTC, en el mismo
/// formato, para poder compararlo contra columnas que se guardan en UTC.
pub fn convertir_local_a_utc(fecha_local_str: &str) -> Result<String, String> {
    let naive = NaiveDateTime::parse_from_str(fecha_local_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| format!("Error al parsear fecha '{}': {}", fecha_local_str, e))?;

    let local_dt = Local
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| format!("Fecha local ambigua o inexistente: {}", fecha_local_str))?;

    let utc_dt = local_dt.with_timezone(&chrono::Utc);

    Ok(utc_dt.format("%Y-%m-%d %H:%M:%S").to_string())
}
