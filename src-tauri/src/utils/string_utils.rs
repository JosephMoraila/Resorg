///Hace mayúscula la primera letra de un string
/// # Parameters
/// - `s`: El string que se desea capitalizar.
/// # Returns
/// - `String`: Una nueva cadena con la primera letra en mayúscula y el resto del string sin cambios. Si el string original está vacío, retorna un string vacío.
pub fn capitalizar(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(primera) => primera.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Hace minúscula la primera letra de un string.
/// # Parameters
/// - `s`: El string cuya primera letra se desea hacer minúscula.
/// # Returns
/// - `String`: Una nueva cadena con la primera letra en minúscula y el resto del string sin cambios. Si el string original está vacío, retorna un string vacío.
pub fn descapitalizar(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(primera) => primera.to_lowercase().collect::<String>() + chars.as_str(),
    }
}
