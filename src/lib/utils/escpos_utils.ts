

export function limpiarHtmlParaEscPos(html: string) {
    return html
        .replace(/<div><br><\/div>/g, '\n') // Líneas completamente vacías a \n
        .replace(/<div>/g, '\n')            // El inicio de una nueva línea a \n
        .replace(/<\/div>/g, '')            // Elimina el cierre del div
        .replace(/&nbsp;/g, ' ')            // Convierte los &nbsp; a espacios normales
        .replace(/<br>/g, '\n');            // Por si hay algún <br> suelto
}