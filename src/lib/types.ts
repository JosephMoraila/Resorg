// src/lib/types.ts
export interface NodoArbol<T = unknown> {
  id: string;
  label: string;
  data: T;
  hijos?: NodoArbol<unknown>[]; // ya no fuerza el mismo T que el padre
}

export interface PropsArbol<T> {
  nodo?: NodoArbol<T> | null;
  nodos?: NodoArbol<T>[];
  onSeleccionar?: (nodo: NodoArbol<unknown>) => void;
  selectedId?: string | null;
}

export interface PlatilloCategoria{
    tipo: "categoria";
    id: number;
    nombre: string;
    platillos: Platillo[];
}

export interface Platillo{
    tipo: "platillo";
    id: number;
    nombre: string;
    descripcion: string | null;
    precio: number;
    id_categoria:number;
}

export interface Piso{
  piso: number;
  numero_mesas: number;
}

export interface Mesero{
  id: number;
  nombre: string;
}

export type TipoPedido = "Local" | "Domicilio" | "Recoger";

/** Interface de un platillo que sea desea ordenar, NO es historico, solo es al ordenar*/
export interface PlatilloPedido{
  id_platillo: number;
  id_category: null | number;
  cantidad: number;
}

//Pedidos ya pedidos

export type EstadoPedido = "Pendiente" | "Finalizado" | "Cancelado" | "Entregado" | "Cobrado";

export interface PedidoLocal{
  tipo: "Local";
  id: number;
  piso: number;
  mesa: number;
  mesero: string;
  pedido_id: number;
}

export interface PedidoDomicilio{
  tipo: "Domicilio"
  id: number;
  colonia: string | null;
  calle: string | null;
  numero_interior_exterior: number | null;
  telefono: string | null;
  repartidor: string | null;
  pedido_id: number;
}

export interface PedidoRecoger{
  tipo: "Recoger"
  id: number;
  pedido_id: number;
}

/** Interface de un platillo pedido de manera historica*/
export interface PedidoPlatillo{
  id: number;
  name: string;
  precio: number;
  platillo_id: number;
  categoria_id: number | null;
  pedido_id: number;
  platillo: Platillo | null;
}

export type MetodoPago = "Efectivo" | "Tarjeta" | "Transferencia" | "Mixto";

export interface Pedido{
  id: number;
  total: number;
  tipo: TipoPedido;
  estado: EstadoPedido;
  nombre_cliente: string | null;
  nota: string | null;
  fecha_hora: string;
  platillos_pedidos: PedidoPlatillo[];
  info_tipo_pedido: PedidoLocal | PedidoDomicilio | PedidoRecoger;
  metodo_pago: MetodoPago | null;
}

export interface FiltrosVerOrdenesProps{
  id: null | number;
  tipoPedido: null | TipoPedido;
  nombreCliente: null | string;
  fechaInicio: null | string; fechaFin: null | string; 
  totalDesde: null | number; totalHasta: null | number;
  nota: null | string;
  estatus: null | EstadoPedido;
  metodoPago: MetodoPago | null;
}

export interface FilaPlatillo {
    id: string;
    nombre: string;
    id_platillo: number | null;
    id_category: null | number;
    cantidad: number;
  }

export interface UpdatePlatilloPedido{
  /**Si es null significa que el nuevo platillo a pedir es uno adicional, si contiene valor significa que conserva el ID de la DB y significa que no se eliminó ni agregó uno*/
  id_platillo_pedido_original: number | null;
  id_platillo: number;
  id_category: null | number;
  cantidad: number;
  pedido_id: number;
}

//Ticket

export interface TextCanvasElement{
  tipo: "Texto";
  texto: string;
  size: number;
  is_under_info: boolean;
}

export interface ImageCanvasElement{
  tipo: "Imagen";
  src: string;
  alto: number;
  ancho: number;
}

export interface InfoCanvasElement{
  tipo: "Info";
  texto: string;
  size: number;
}

export interface CanvasElement{
  id: number;
  x: number;
  y: number;
  element: TextCanvasElement | ImageCanvasElement | InfoCanvasElement;
}

