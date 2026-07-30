export interface NodoArbol<T> {
  id: string;
  label: string;
  data: T;
  hijos?: NodoArbol<T>[];
}

export interface PropsArbol<T> {
  nodo: NodoArbol<T>;
  onSeleccionar?: (nodo: NodoArbol<T>) => void;
}

export interface PlatilloCategoria{
    id: number;
    platillos: Platillo[];
}

export interface Platillo{
    id: number;
    nombre: string;
    descripcion: string;
    precio: number;
}