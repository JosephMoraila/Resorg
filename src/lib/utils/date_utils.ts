import { getArrayYearMonthDayFromStringDatetime } from "./math_utils";

/**
 * Obtiene el rango de fechas correspondiente al día de hoy,
 * desde las 00:00:00.000 hasta las 23:59:59.999, usando la
 * zona horaria local del sistema.
 *
 * Útil para filtros de "hoy" en rangos de fecha (ej. flatpickr),
 * donde se necesita cubrir el día completo sin importar la hora
 * exacta en que se ejecuta la función.
 *
 * @returns Un objeto con dos `Date`:
 * - `inicio`: hoy a las 00:00:00.000 (medianoche).
 * - `fin`: hoy a las 23:59:59.999 (último instante del día).
 *
 * @example
 * const { inicio, fin } = obtenerRangoDeHoy();
 * // inicio: 2026-09-04T00:00:00.000
 * // fin:    2026-09-04T23:59:59.999
 */
export function obtenerRangoDeHoy(): { inicio: Date; fin: Date } {
    const ahora = new Date();

    const inicio = new Date(Date.UTC(ahora.getUTCFullYear(), ahora.getUTCMonth(), ahora.getUTCDate(), 0, 0, 0, 0));
    const fin = new Date(Date.UTC(ahora.getUTCFullYear(), ahora.getUTCMonth(), ahora.getUTCDate(), 23, 59, 59, 999));

    return { inicio, fin };
}

/**
 * Convierte un objeto Date a una cadena de texto en formato UTC ('YYYY-MM-DD HH:mm')
 * apta para ser utilizada en inputs y configuraciones de fecha.
 *
 * @param fecha - Objeto Date que se desea formatear.
 * @returns Una cadena de texto con la fecha y hora en formato universal UTC.
 *
 * @example
 * const fechaStr = formatearFechaUTC(new Date());
 * // Resultado: "2026-09-04 14:02"
 */
export function formatearFechaUTC(fecha: Date): string {
    // Extrae el año utilizando el estándar universal UTC.
    const anio = fecha.getUTCFullYear();
    // Obtiene el mes en UTC (sumando 1 ya que enero es 0) y asegura dos dígitos rellenando con ceros.
    const mes = String(fecha.getUTCMonth() + 1).padStart(2, '0');
    // Extrae el día del mes en UTC garantizando un formato de dos dígitos.
    const dia = String(fecha.getUTCDate()).padStart(2, '0');
    // Extrae la hora en formato UTC asegurando dos dígitos.
    const hora = String(fecha.getUTCHours()).padStart(2, '0');
    // Extrae los minutos en formato UTC asegurando dos dígitos.
    const minuto = String(fecha.getUTCMinutes()).padStart(2, '0');

    // Retorna la cadena de texto combinando todos los componentes ordenados.
    return `${anio}-${mes}-${dia} ${hora}:${minuto}:00`;
}

/**
 * Procesa una cadena de fecha y hora para asegurar que incluya los segundos.
 * Si el bloque de hora cuenta únicamente con horas y minutos, agrega ':00'.
 * 
 * @param cadena - Cadena de texto con fecha y hora (ej. "2026-09-04 14:30")
 * @returns Cadena con el formato completo de segundos (ej. "2026-09-04 14:30:00")
 */
export function procesarCadenaHora(cadena: string): string {
    // Elimina espacios sobrantes y divide la cadena por espacios para separar la fecha de la hora
    const partes = cadena.trim().split(/\s+/);
    
    // Evalúa si el resultado de la división contiene al menos dos elementos; de lo contrario, devuelve la cadena intacta
    if (partes.length < 2) return cadena;

    // Almacena el segundo segmento de la división, el cual corresponde estrictamente a la hora
    const segundoTrozo = partes[1];
    
    // Divide la porción de hora utilizando los dos puntos (:) para separar horas, minutos y segundos
    const subPartes = segundoTrozo.split(':');

    // Comprueba si la hora consta exclusivamente de dos fragmentos (formato HH:mm sin segundos)
    if (subPartes.length === 2) {
        // Inserta '00' al final del arreglo para completar el formato estándar de segundos
        subPartes.push('00');
    }

    // Vuelve a unir los segmentos de la hora empleando dos puntos como delimitador
    partes[1] = subPartes.join(':');
    
    // Reconstruye la cadena completa uniendo la fecha y la hora procesada mediante un espacio
    return partes.join(' ');
}

/**
 * Regresa solo un objeto Date por las fechas sin horas
 * @param datetime Datetime tipo String
 * @returns Objeto Date solo con fecha
 */
export function getOnlyDateObjectByOnlyDatetime(fechaStr: string): Date {
  // 1. Convertir a formato ISO indicando que viene en UTC (Igual que tu tabla)
  const fechaISO = fechaStr.replace(" ", "T") + "Z";
  const fechaUTC = new Date(fechaISO);

  // 2. Extraer el año, mes y día pero ya en la HORA LOCAL de la PC
  const anio = fechaUTC.getFullYear();
  const mes = fechaUTC.getMonth();
  const dia = fechaUTC.getDate();

  // 3. Retornar un objeto Date a las 00:00:00 en tu zona horaria local
  return new Date(anio, mes, dia);
}