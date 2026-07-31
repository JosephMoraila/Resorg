<script lang="ts">
  import Tree from "$lib/components/tree/Tree.svelte";
  import type { NodoArbol, PlatilloCategoria } from "$lib/types";
  import ModalAddName from "$lib/components/modals/ModalAddName.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let modalAbierto = $state(false);

  let nodoPadre: NodoArbol<PlatilloCategoria> = $state({
    id: "cat-0",
    label: "Platillos",
    data: { id: 0, platillos: [] },
    hijos: [], 
  });

  async function addCategory(nombre: string | null) {
    if(!nombre) return;
    try{
      const lastId = await invoke<number>("insert_category_platillo", {name: nombre});
      const nuevoNodo: NodoArbol<PlatilloCategoria> = {
        id: `cat-${lastId}`, label: nombre, hijos: [], 
        data: {id: lastId, platillos:[]}
      };
      nodoPadre.hijos?.push(nuevoNodo);
    }catch(error){
      const err = error as string;
    }
  }
</script>

<main class="w-screen h-screen p-6 bg-white dark:bg-[#010101] text-gray-900 dark:text-white flex flex-col">
  <div>
    <p>Registra tus platillos</p>
  </div>

  <div class="mt-5 flex flex-row gap-2 flex-wrap">
    <button class="btn-realista" onclick={()=>modalAbierto = true}>Agregar categoría</button>
    <button class="btn-realista">Agregar platillo</button>
  </div>

  <div class="mt-5 w-full flex-1 overflow-auto bg-gray-200 dark:bg-gray-900">
    <Tree nodo={nodoPadre} />
  </div>
</main>

<ModalAddName
  bind:abierto={modalAbierto}
  titulo="Nueva categoría"
  descripcion="Escribe el nombre de la categoría que quieres agregar."
  placeholder="Ej. Desayunos"
  onConfirmar={addCategory}
/>