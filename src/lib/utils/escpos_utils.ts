import { convertirBlobUrlABase64 } from "./file_utils";

export function limpiarHtmlParaEscPos(html: string) {
    return html
        .replace(/<div><br><\/div>/g, '\n') // Líneas completamente vacías a \n
        .replace(/<div>/g, '\n')            // El inicio de una nueva línea a \n
        .replace(/<\/div>/g, '')            // Elimina el cierre del div
        .replace(/&nbsp;/g, ' ')            // Convierte los &nbsp; a espacios normales
        .replace(/<br>/g, '\n');            // Por si hay algún <br> suelto
}

export async function prepararStringHtmlParaExportar(htmlString: string): Promise<string> {
    // A. Creamos un elemento <div> en memoria (no se mostrará en pantalla)
    const contenedorTemporal = document.createElement('div');
    // B. Le inyectamos tu string HTML
    contenedorTemporal.innerHTML = htmlString;
    
    // C. Buscamos todas las imágenes
    const imagenes = contenedorTemporal.querySelectorAll('img');
    
    // D. Reemplazamos los blobs por Base64
    for (const img of Array.from(imagenes)) {
        if (img.src.startsWith('blob:')) {
            try {
                const base64 = await convertirBlobUrlABase64(img.src);
                img.src = base64; // Se cambia la URL temporal por el código Base64 completo
            } catch (error) {
                console.error("Error al convertir imagen a Base64:", error);
            }
        }
    }
    
    // E. Retornamos el string modificado
    return contenedorTemporal.innerHTML;
}