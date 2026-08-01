use tauri::AppHandle;
use tauri::Manager;

/// Enfoca una ventana específica de la aplicación por su etiqueta (label).
///
/// Si la ventana está minimizada, primero la restaura y luego le da el foco.
/// Es útil para evitar abrir ventanas duplicadas: en vez de crear una nueva
/// instancia de una ventana que ya existe, se llama a esta función para
/// traer al frente la que ya está abierta.
///
/// # Argumentos
///
/// * `app` - El `AppHandle` de la aplicación, usado para buscar la ventana.
/// * `label` - El identificador único de la ventana (el mismo que se usó
///   al crearla con `WebviewWindow::new`).
///
/// # Retorna
///
/// `true` si la ventana fue encontrada y enfocada correctamente.
/// `false` si no existe ninguna ventana con ese `label`.
///
/// # Ejemplo
///
/// ```
/// let encontrada = enfocar_ventana(app, "registrar-platillos".to_string());
/// if !encontrada {
///     // la ventana no existe, hay que crearla
/// }
/// ```
#[tauri::command]
pub fn enfocar_ventana(app: AppHandle, label: String) -> bool {
    if let Some(ventana) = app.get_webview_window(&label) {
        let _ = ventana.unminimize();
        let _ = ventana.set_focus();
        true
    } else {
        false
    }
}
