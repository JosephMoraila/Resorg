/**
 * Sanitiza la entrada de un input numérico impidiendo valores negativos.
 * Si el valor es negativo, ajusta el input y devuelve 0 (o el min por defecto).
 * 
 * @param event Evento 'input' del HTMLInputElement
 * @param min Valor mínimo permitido (por defecto 0)
 * @returns El número parseado y sanitizado
 */
export function sanitizeNonNegativeInput(event: Event, min: number = 0): number {
    const input = event.currentTarget as HTMLInputElement;
    let val = parseFloat(input.value);

    // Si está vacío o no es un número válido (NaN)
    if (isNaN(val)) {
        return min;
    }

    // Si es menor que el mínimo permitido
    if (val < min) {
        input.value = min.toString();
        return min;
    }

    return val;
}