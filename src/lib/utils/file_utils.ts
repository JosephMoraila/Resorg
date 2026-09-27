import { invoke } from "@tauri-apps/api/core";

export async function obtenerImagenPlatillo(id: number): Promise<string | null> {
  try {
    const bytes = await invoke<number[] | null>("get_imagen_platillo", { id });
    if (!bytes) return null; // no había imagen, caso normal
    const blob = new Blob([new Uint8Array(bytes)], { type: "image/png" });
    return URL.createObjectURL(blob);
  } catch (error) {
    console.error(`Error al obtener imagen del platillo ${id}:`, error);
    return null;
  }
}

export async function previewUrlAUint8(previewUrl: string): Promise<Uint8Array> {
  // 1. Haces fetch a la URL temporal del Blob
  const respuesta = await fetch(previewUrl);

  // 2. Obtienes el Blob/Buffer de la respuesta
  const buffer = await respuesta.arrayBuffer();

  // 3. Creas el Uint8Array
  return new Uint8Array(buffer);
}

export async function convertirBlobUrlABase64(blobUrl: string): Promise<string> {
    //Hacemos fetch al blob local de la memoria RAM
    const respuesta = await fetch(blobUrl);
    const blob = await respuesta.blob();

    //Lo transformamos a Base64 usando FileReader
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result as string);
        reader.onerror = (error) => reject(error);
        reader.readAsDataURL(blob);
    });
}

export function convertirBase64ABlobUrl(base64Data: string): string {
    // Separar la cabecera (ej: "data:image/png;base64,") del texto puro
    const partes = base64Data.split(',');
    const tipoMime = partes[0].match(/:(.*?);/)?.[1] || 'image/png';
    const base64Puro = partes.length > 1 ? partes[1] : partes[0];

    // Decodificar el texto Base64 a binario crudo
    const caracteresBinarios = atob(base64Puro);

    // Escribir los caracteres en un arreglo de bytes reales
    const arregloBytes = new Uint8Array(caracteresBinarios.length);
    for (let i = 0; i < caracteresBinarios.length; i++) {
        arregloBytes[i] = caracteresBinarios.charCodeAt(i);
    }

    // Empaquetar los bytes en un Blob y crear la URL local segura
    const blob = new Blob([arregloBytes], { type: tipoMime });
    return URL.createObjectURL(blob);
}