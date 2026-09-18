
/**
 * Calcula el número total de páginas necesarias para mostrar todos los registros, dado un tamaño de página específico.
 * @param totalRegistros El número total de registros que se desean paginar.
 * @param tamanoPagina El número de registros que se mostrarán en cada página.
 * @returns El número total de páginas necesarias para mostrar todos los registros.
 * @example
 * // Calcular el total de páginas para 10 registros con un tamaño de página de 50
 * const totalPaginas = calcularTotalPaginas(10, 50); // Devuelve 1
 *
 * // Calcular el total de páginas para 51 registros con un tamaño de página de 50
 * const totalPaginas2 = calcularTotalPaginas(51, 50); // Devuelve 2
 *
 * // Calcular el total de páginas para 0 registros con un tamaño de página de 50
 * const totalPaginas3 = calcularTotalPaginas(0, 50); // Devuelve 1
 */
export function calcularTotalPaginas(totalRegistros: number, tamanoPagina: number): number {
    if (totalRegistros <= 0 || tamanoPagina <= 0) {
        return 1;
    }
    const division = totalRegistros / tamanoPagina;//Ejemplo: 10 registros, 50 por página => 10/50 = 0.2 => 1 página. 51 registros, 50 por página => 51/50 = 1.02 => 2 páginas
    const totalPaginas = Math.ceil(division); //Redondear hacia arriba para obtener el número total de páginas
    return totalPaginas;
}

/**
 * Convierte un valor opcional (string, null o undefined) a un número válido,
 * retornando null si está vacío, es nulo o no es un número parseable.
 *
 * @param valor - El valor recibido desde el input o formulario.
 * @returns El número convertido o null si no es válido.
 */
export function parsearNumeroOpcional(valor: string | null | undefined): number | null {
    if (valor === "" || valor === null || valor === undefined) {
        return null;
    }

    const numero = Number(valor);
    return Number.isNaN(numero) ? null : numero;
}

/**
 * Convierte un valor string que se espera sean numeros a un tipo number de tipo Int
 * @param valor String que se espera sean números
 * @returns Number tipo Int o null si es algo no esperado
 */
export function parsearStringToInt(valor: string): number | null{
    const valorInt = parseInt(valor);
    const valorFinal = Number.isNaN(valorInt) ? null : valorInt;
    return valorFinal;
}

/**
 * Verifica si un número entero es positivo y mayor a 0 y en caso que tenga decimales lo trunca a entero. Si es 0 o negativo esta funcion retorna 1
 * @param numero Número a evaluar
 * @returns Entero positivo
 */
export function TruncarToEnteroPositivo(numero: number): number{
    const cant = numero && numero > 0 ? numero : 1; //Si numero es true (negativo tambien da true excepto 0) y es mayor a 0 da el numero, sino da 1
    const truncado = Math.trunc(cant); //Si el numero original tiene algun decimal lo cortamos y lo dejamos en entero
    return truncado;
}