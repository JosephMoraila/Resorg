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