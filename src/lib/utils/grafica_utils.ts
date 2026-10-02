import type { Pedido, ConteoPedidosPorFecha, ConteoPedidosPorTipo, DataSeries, ConteoTotalRecaudadoPorFecha, ConteoEstadoPedidosPorFecha, ConteoTotalRecaudadoPorTipoPorFecha, ConteoPlatillosMasVendidosPorFecha, GeneralTipoPedidoPie, GeneralEstadoPedidoPie, GeneralPlatillosPie } from "$lib/types";
import { getOnlyDateObjectByOnlyDatetime } from "./date_utils";

/**
 * Genera una serie de datos basados en fechas con valores aleatorios.
 * Ideal para rellenar gráficas de prueba.
 */
export function createDateSeries(options: DataSeries = {}): Record<string, any>[] {
  // 1. Asignar valores por defecto si no vienen en el objeto
  const count = options.count ?? 10;
  const min = options.min ?? 0;
  const max = options.max ?? 100;
  const isInteger = options.value === 'integer';
  const keys = options.keys ?? ['value']; // Llave por defecto si el arreglo está vacío

  const series = [];
  const fechaBase = new Date(); 

  // 2. Iterar la cantidad de veces (count) para crear los días
  for (let i = 0; i < count; i++) {
    const fechaActual = new Date(fechaBase);
    fechaActual.setDate(fechaBase.getDate() + i); // Avanza un día por cada iteración

    // Empezamos armando el objeto con la fecha
    const puntoDeDato: Record<string, any> = {
      date: fechaActual,
    };

    // 3. Generar un valor aleatorio para cada llave que se pidió
    for (const key of keys) {
      let valorAleatorio = Math.random() * (max - min) + min;
      
      if (isInteger) {
        valorAleatorio = Math.round(valorAleatorio);
      }
      
      puntoDeDato[key] = valorAleatorio;
    }

    series.push(puntoDeDato);
  }

  return series;
}

export function getListPedidosByDate(pedidos: Pedido[]): ConteoPedidosPorFecha[] {
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

export function getListTiposPedidosByDate(pedidos: Pedido[]): ConteoPedidosPorTipo[] {
  // Mapa para acumular usando la clave de texto YYYY-MM-DD (para agrupar correctamente)
  const conteoMap: Record<string, ConteoPedidosPorTipo> = {};

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
        tipoLocal: 0,      // Arrancar en 0
        tipoDomicilio: 0,  // Arrancar en 0
        tipoRecoger: 0     // Arrancar en 0
      };
    }

    // Incrementamos el contador según el tipo de pedido
    switch (p.tipo) {
      case "Local":
        conteoMap[fechaKey].tipoLocal += 1;
        break;
      case "Domicilio":
        conteoMap[fechaKey].tipoDomicilio += 1;
        break;
      case "Recoger":
        conteoMap[fechaKey].tipoRecoger += 1;
        break;
    }
  }

  // Convertimos los valores del mapa al arreglo final
  return Object.values(conteoMap).map((item) => ({
    fecha: item.fecha,
    tipoLocal: item.tipoLocal,
    tipoDomicilio: item.tipoDomicilio,
    tipoRecoger: item.tipoRecoger
  }));
}

export function getListTotalRecaudadoByDate(pedidos: Pedido[]): ConteoTotalRecaudadoPorFecha[] {
  // Mapa para acumular usando la clave de texto YYYY-MM-DD (para agrupar correctamente)
  const conteoMap: Record<string, ConteoTotalRecaudadoPorFecha> = {};

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
        total: 0, // Arrancar en 0
      };
    }

    // Incrementamos el total recaudado sumando el total del pedido
    conteoMap[fechaKey].total += p.total;
  }

  // Convertimos los valores del mapa al arreglo final
  return Object.values(conteoMap).map((item) => ({
    fecha: item.fecha,
    total: item.total,
  }));
}

export function getListEstadosPedidosByDate(pedidos: Pedido[]): ConteoEstadoPedidosPorFecha[] {
  // Mapa para acumular usando la clave de texto YYYY-MM-DD (para agrupar correctamente)
  const conteoMap: Record<string, ConteoEstadoPedidosPorFecha> = {};

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
        pendiente: 0,
        finalizado: 0,
        cancelado: 0,
        entregado: 0,
        cobrado: 0
      };
    }

    // Incrementamos el contador según el estado del pedido
    switch (p.estado) {
      case "Pendiente":
        conteoMap[fechaKey].pendiente += 1;
        break;
      case "Finalizado":
        conteoMap[fechaKey].finalizado += 1;
        break;
      case "Cancelado":
        conteoMap[fechaKey].cancelado += 1;
        break;
      case "Entregado":
        conteoMap[fechaKey].entregado += 1;
        break;
      case "Cobrado":
        conteoMap[fechaKey].cobrado += 1;
        break;
    }
  }

  // Convertimos los valores del mapa al arreglo final
  return Object.values(conteoMap).map((item) => ({
    fecha: item.fecha,
    pendiente: item.pendiente,
    finalizado: item.finalizado,
    cancelado: item.cancelado,
    entregado: item.entregado,
    cobrado: item.cobrado
  }));
}

export function getListTotalRecaudadoByTipoByDate(pedidos: Pedido[]): ConteoTotalRecaudadoPorTipoPorFecha[] {
  // Mapa para acumular usando la clave de texto YYYY-MM-DD (para agrupar correctamente)
  const conteoMap: Record<string, ConteoTotalRecaudadoPorTipoPorFecha> = {};

  for (const p of pedidos) {
    const date: Date = getOnlyDateObjectByOnlyDatetime(p.fecha_hora);

    // Creamos una clave única en string YYYY-MM-DD para agrupar en el objeto
    const anio = date.getFullYear();
    const mes = String(date.getMonth() + 1).padStart(2, '0');
    const dia = String(date.getDate()).padStart(2, '0');
    const fechaKey = `${anio}-${mes}-${dia}`;

    //Si la fecha no existe en el mapa, la inicializamos con valores en 0 para cada tipo de pedido
    if (!conteoMap[fechaKey]) {
      conteoMap[fechaKey] = {
        fecha: date,
        totalTipoLocal: 0,
        totalTipoDomicilio: 0,
        totalTipoRecoger: 0
      };
    }

    // Incrementamos el total recaudado según el tipo de pedido
    switch (p.tipo) {
      case "Local":
        conteoMap[fechaKey].totalTipoLocal += p.total;
        break;
      case "Domicilio":
        conteoMap[fechaKey].totalTipoDomicilio += p.total;
        break;
      case "Recoger":
        conteoMap[fechaKey].totalTipoRecoger += p.total;
        break;
    }
  }

  // Convertimos los valores del mapa al arreglo final
  const resultado: ConteoTotalRecaudadoPorTipoPorFecha[] = Object.values(conteoMap); //Convertimos los valores del mapa al arreglo final
  const object: ConteoTotalRecaudadoPorTipoPorFecha[] = resultado.map((item) => ({
    fecha: item.fecha,
    totalTipoLocal: item.totalTipoLocal,
    totalTipoDomicilio: item.totalTipoDomicilio,
    totalTipoRecoger: item.totalTipoRecoger
  }));  

  
  return object;

}

export function getListPlatillosMasVendidosByDate(pedidos: Pedido[]): ConteoPlatillosMasVendidosPorFecha[] {
  // Mapa para acumular usando la clave de texto YYYY-MM-DD (para agrupar correctamente)
  const conteoMap: Record<string, ConteoPlatillosMasVendidosPorFecha> = {};

  for (const p of pedidos) {
    const date: Date = getOnlyDateObjectByOnlyDatetime(p.fecha_hora);

    // Creamos una clave única en string YYYY-MM-DD para agrupar en el objeto
    const anio = date.getFullYear();
    const mes = String(date.getMonth() + 1).padStart(2, '0');
    const dia = String(date.getDate()).padStart(2, '0');
    const fechaKey = `${anio}-${mes}-${dia}`;

    if (!conteoMap[fechaKey]) { //Si la fecha no existe en el mapa, la inicializamos con un arreglo vacío para los platillos
      conteoMap[fechaKey] = {
        fecha: date,
        platillos: []
      };
    }

    
    for (const platilloPedido of p.platillos_pedidos) {//Iteramos sobre cada platillo pedido en el pedido actual
      const platilloExistente: ConteoPlatillosMasVendidosPorFecha = conteoMap[fechaKey]; //Tomamos el objeto de la fecha actual, si no exisitia ya lo inicializamos arriba
      //Verificar que el platillo ya exista en el arreglo de platillos de la fecha actual
      const platilloEnFecha = platilloExistente.platillos.find(p => p.nombre === platilloPedido.name);
      if (platilloEnFecha) {
        //Si ya existe, incrementamos la cantidad vendida
        platilloEnFecha.cantidad_vendida += 1;
        platilloEnFecha.total_recaudado += platilloPedido.precio;
      } else {
        //Si no existe, lo agregamos al arreglo de platillos de la fecha actual
        platilloExistente.platillos.push({
          nombre: platilloPedido.name,
          total_recaudado: platilloPedido.precio,
          cantidad_vendida: 1
        });
      }
    }
  }

  // Convertimos los valores del mapa al arreglo final
  const obj: ConteoPlatillosMasVendidosPorFecha[] = Object.values(conteoMap);
  const toReturn: ConteoPlatillosMasVendidosPorFecha[] = obj.map((item) => ({
    fecha: item.fecha,
    platillos: item.platillos
  }));  
  return toReturn;
}

//PIE:

export function getTotalRecaudadoGeneral(pedidos: Pedido[]): number {
  const total =  pedidos.reduce((total, p) => total + p.total, 0);
  return total;
}

export function getGeneralTipoPedidoPie(pedidos: Pedido[]): GeneralTipoPedidoPie {
  const conteo: GeneralTipoPedidoPie = {
    tipoLocal: 0,
    tipoDomicilio: 0,
    tipoRecoger: 0
  };

  for (const p of pedidos) {
    switch (p.tipo) {
      case "Local":
        conteo.tipoLocal += 1;
        break;
      case "Domicilio":
        conteo.tipoDomicilio += 1;
        break;
      case "Recoger":
        conteo.tipoRecoger += 1;
        break;
    }
  }

  return conteo;
}

export function getGeneralEstadoPedidoPie(pedidos: Pedido[]): GeneralEstadoPedidoPie {
  const conteo: GeneralEstadoPedidoPie = {
    pendiente: 0,
    finalizado: 0,
    cancelado: 0,
    entregado: 0,
    cobrado: 0
  };

  for (const p of pedidos) {
    switch (p.estado) {
      case "Pendiente":
        conteo.pendiente += 1;
        break;
      case "Finalizado":
        conteo.finalizado += 1;
        break;
      case "Cancelado":
        conteo.cancelado += 1;
        break;
      case "Entregado":
        conteo.entregado += 1;
        break;
      case "Cobrado":
        conteo.cobrado += 1;
        break;
    }
  }

  return conteo;
}

export function getGeneralPlatillosPie(pedidos: Pedido[]): GeneralPlatillosPie[] {
  const conteoMap: Record<string, GeneralPlatillosPie> = {};

  for (const p of pedidos) {
    for (const platilloPedido of p.platillos_pedidos) {
      const platilloExistente: GeneralPlatillosPie = conteoMap[platilloPedido.name] || {nombre: platilloPedido.name,total_recaudado: 0, cantidad_vendida: 0};

      // Incrementamos la cantidad vendida y el total recaudado
      platilloExistente.cantidad_vendida += 1;
      platilloExistente.total_recaudado += platilloPedido.precio;

      // Guardamos el objeto actualizado en el mapa
      conteoMap[platilloPedido.name] = platilloExistente;
    }
  }

  // Convertimos los valores del mapa al arreglo final
  return Object.values(conteoMap);
}