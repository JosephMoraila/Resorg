<script lang="ts">
  import Tree from "$lib/components/tree/Tree.svelte";
  import type { NodoArbol, PlatilloCategoria } from "$lib/types";

  let nodoPadre: NodoArbol<PlatilloCategoria> = $state({
    id: "0",
    label: "Platillos",
    data: { id: 0, platillos: [] },
    hijos: [], 
  });

  function addCategory() {
    const newCategory: NodoArbol<PlatilloCategoria> = {
      id: crypto.randomUUID(),
      label: "Nuevo",
      data: { id: 0, platillos: [] },
    };
    nodoPadre.hijos?.push(newCategory);
  }
</script>

<main class="w-screen h-screen p-6 bg-white dark:bg-[#010101] text-gray-900 dark:text-white flex flex-col">
  <div>
    <p>Registra tus platillos</p>
  </div>

  <div class="mt-5 flex flex-row gap-2 flex-wrap">
    <button class="btn-realista" onclick={addCategory}>Agregar categoría</button>
    <button class="btn-realista">Agregar platillo</button>
  </div>

  <div class="mt-5 w-full flex-1 overflow-auto bg-gray-200 dark:bg-gray-900">
    <Tree nodo={nodoPadre} />
  </div>
</main>