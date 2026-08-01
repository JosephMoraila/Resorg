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