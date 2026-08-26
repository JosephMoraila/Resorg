/**
 * Formatea un string/entrada para que solo acepte números, 
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