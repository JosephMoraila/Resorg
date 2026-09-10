/**
 * Formatea un string/entrada para que solo acepte números, Se usa en inputs de tipo texto para que el usuario pueda escribir libremente, pero el valor final se formatea como moneda.
 * añade comas cada 3 dígitos y limita a máximo 2 decimales.
 */
export function formatearMonedaInput(valor: string): string {
  // 1. Quitar todo lo que no sea número o punto
  let limpio = valor.replace(/[^0-9.]/g, '');

  // 2. Permitir solo el primer punto decimal
  const partes = limpio.split('.');
  if (partes.length > 2) {
    limpio = partes[0] + '.' + partes.slice(1).join('');
  }

  // 3. Separar parte entera y decimal
  let [entera, decimal] = limpio.split('.');

  // 4. Formatear la parte entera con comas
  if (entera) {
    entera = entera.replace(/\B(?=(\d{3})+(?!\d))/g, ',');
  }

  // 5. Limitar a máximo 2 decimales
  if (decimal !== undefined) {
    decimal = decimal.slice(0, 2);
    return `${entera}.${decimal}`;
  }

  return entera;
}

/**
 * Formatea un número como moneda, añadiendo comas cada 3 dígitos y limitando a máximo 2 decimales. Se usa para mostrar valores ya procesados en la interfaz de usuario.
 * @param valor - El número a formatear.
 * @returns El número formateado como string con comas y 2 decimales.
 */
export function formatearMoneda(valor: number): string {
  // 1. Redondear a 2 decimales y convertir a string fijo (evita problemas de punto flotante)
  const fijo = valor.toFixed(2);

  // 2. Separar parte entera y decimal
  const [entera, decimal] = fijo.split('.');

  // 3. Formatear la parte entera con comas cada 3 dígitos
  const enteraFormateada = entera.replace(/\B(?=(\d{3})+(?!\d))/g, ',');

  return `${enteraFormateada}.${decimal}`;
}

/**
 * Función opcional: Convierte el texto formateado con comas (ej: "1,250.50") 
 * de vuelta a un 'number' limpio para guardar en la base de datos (ej: 1250.50).
 */
export function stringANumero(valorFormateado: string): number {
  if (!valorFormateado) return 0;
  const sinComas = valorFormateado.replace(/,/g, '');
  return parseFloat(sinComas) || 0;
}

/**
 * Limpia un texto dejando solo números, el signo '+' y espacios individuales.
 */
export function limpiarTelefono(valor: string): string {
	return valor
		.replace(/[^0-9+ ]/g, '') // Elimina caracteres no permitidos
		.replace(/ {2,}/g, ' ');   // Reduce múltiples espacios a uno solo
}

/**
 * Manejador de evento genérico para inputs de teléfono.
 * Limpia el valor del HTMLInputElement y retorna el texto resultante.
 */
export function sanitizarInputTelefono(e: Event): string {
	const target = e.target as HTMLInputElement;
	const valorLimpio = limpiarTelefono(target.value);
	target.value = valorLimpio;
	return valorLimpio;
}

/**
 * Retorna un string vacío si el valor es null o undefined, de lo contrario retorna el valor original.
 * @param value - El valor a evaluar, que puede ser un string, null o undefined.
 * @returns Un string vacío si el valor es null o undefined, de lo contrario retorna el valor original.
 */
export function returnEmptyStringIfNullOrUndefined(value: string | number | null | undefined): string {
    if (typeof value === "string") {
      return value;
    } else if (typeof value === "number") {
      return value.toString();
    } else {
      return "";
    }
}

/**
 * Formatea un string de fecha proveniente de la base de datos a un formato legible en la zona horaria local.
 * Ejemplo: "2024-06-15T14:30:00Z" -> "15 de junio de 2024, 14:30:00"
 * @param fechaStr - El string de fecha en formato ISO 8601 proveniente de la base de datos.
 * @returns Un string de fecha formateado según la zona horaria local y en un formato legible.
 */
export function formatearFechaDB(fechaStr: string): string {
  // 1. Convierte "2026-08-31 03:05:59" (formato de SQLite, sin indicar zona)
  //    a "2026-08-31T03:05:59Z" (formato ISO, la Z le dice a JS "esto es UTC")
  const fechaISO = fechaStr.replace(" ", "T") + "Z";
  const fecha = new Date(fechaISO);

  // 2. Ahora sí, toLocaleString convierte correctamente de UTC a la hora local de la PC
  const opciones: Intl.DateTimeFormatOptions = {
    day: "2-digit",
    month: "long",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: false,
  };
  return fecha.toLocaleString(undefined, opciones);
}

/**
 * Convierte un number en string y si es null una cadena vacía
 * @param value Número a verificar o null
 * @returns Si es null retorna vacio, de lo contario el numero convertido en string
 */
export function returnNumberOrNullAsString(value: number | null): string{
  let devolver = "";
  if(typeof value == "number"){
    const numberString = value.toString();
    devolver = numberString;
  }
  return devolver;
}

/**
 * Recorta el string sus espacios a los lados en blanco y si es vacio retorna null, sino el string recortado
 * @param value String a verificar
 * @returns Recorta el string sus espacios a los lados en blanco y si es vacio retorna null, sino el string recortado
 */
export function returnNullOrStringValue(value: string): null | string{
  const trimValue = value.trim();
  if(trimValue == "") return null;
  else return trimValue;
}