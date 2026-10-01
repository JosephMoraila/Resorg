import type { Pedido, ConteoPedidosPorFecha } from "$lib/types";

import { getOnlyDateObjectByOnlyDatetime } from "./date_utils";

export default function getListPedidosByDate(pedidos: Pedido[]): ConteoPedidosPorFecha[] {
  // Mapa para acumular usando la clave de texto YYYY-MM-DD (para agrupar correctamente)
  // Guardamos tanto el objeto Date como el total acumulado
  const conteoMap: Record<string, ConteoPedidosPorFecha> = {};

  for (const p of pedidos) {
    const date: Date = getOnlyDateObjectByOnlyDatetime(p.fecha_hora);

    // Creamos una clave única en string YYYY-MM-DD para agrupar en el objeto
    const anio = date.getFullYear();
    const mes = String(date.getMonth() + 1).padStart(2, '0');
    const dia = String(date.getDate()).padStart(2, '0');
    const fechaKey = `${anio}-${mes}-${dia}`;

    if (!conteoMap[fechaKey]) {
      conteoMap[fechaKey] = {
        fecha: date,
        total: 1,
      };
    } else {
      conteoMap[fechaKey].total += 1;
    }
  }

  // Convertimos los valores del mapa al arreglo final
  return Object.values(conteoMap).map((item) => ({
    fecha: item.fecha,
    total: item.total,
  }));
}