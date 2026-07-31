<script lang="ts">
  import Tree from "$lib/components/tree/Tree.svelte";
  import type { NodoArbol, PlatilloCategoria, Platillo } from "$lib/types";
  import ModalAddName from "$lib/components/modals/ModalAddName.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { toast } from "$lib/toast.svelte";

  let modalAbiertoCategoria = $state(false);
  let selectedNode = $state<string | null>(null);

  let nodoPadre: NodoArbol<PlatilloCategoria> = $state({
    id: "cat-0",
    label: "Platillos",
    data: { tipo: "categoria",id: 0, nombre: "Platillos", platillos: [] },
    hijos: [],
  });

  function platilloANodo(platillo: Platillo): NodoArbol<Platillo> {
    return {
      id: `plat-${platillo.id}`,
      label: platillo.nombre,
      data: {...platillo, tipo: "platillo"},
    };
  }

  onMount(() => {
    async function getCategories() {
      try {
        const categoriasBackend = await invoke<PlatilloCategoria[]>("get_categories_platillo");

        const nuevosHijos: NodoArbol<PlatilloCategoria>[] = categoriasBackend.map((cat) => ({
          id: `cat-${cat.id}`,
          label: cat.nombre,
          data: {tipo: "categoria", id: cat.id, nombre: cat.nombre, platillos: cat.platillos },
          hijos: cat.platillos.map(platilloANodo),
        }));

        // Reasignación para activar la reactividad de Svelte 5
        nodoPadre.hijos = nuevosHijos;
      } catch (error) {
        const err = error as string;
        toast.rojo(`Error al obtener información: ${err}`);
      }
    }
    
    getCategories();
  });

  function getCategorySameName(h: NodoArbol<unknown>, nombre: string): PlatilloCategoria | undefined{
    const data = h.data as Platillo | PlatilloCategoria;
    if(data.tipo === "categoria"){
      if(data.nombre === nombre){
        return data;
      }
    }else{
      return undefined;
    }
  }

  async function addCategory(nombre: string | null) {
    if (!nombre) return;
    const nombreTrimmed = nombre.trim();
    const n = nodoPadre.hijos?.find(nodo=>getCategorySameName(nodo, nombreTrimmed));
    if(n){
      toast.rojo(`Ya existe una categoría con ese nombre`);
      return;
    }
    try {
      const lastId = await invoke<number>("insert_category_platillo", { name: nombreTrimmed });
      const nuevoNodo: NodoArbol<PlatilloCategoria> = {
        id: `cat-${lastId}`,
        label: nombreTrimmed,
        hijos: [],
        data: { tipo: "categoria",id: lastId, nombre: nombreTrimmed, platillos: [] },
      };
      
      // Actualización reactiva del array
      nodoPadre.hijos = [...(nodoPadre.hijos || []), nuevoNodo];
    } catch (error) {
      const err = error as string;
      toast.rojo(`Error al insertar categoría: ${err}`);
    }
  }

  function onSeleccionar(nodo: NodoArbol<unknown>){
    selectedNode = nodo.id;
  }

</script>

<main class="w-screen h-screen p-6 bg-white dark:bg-[#010101] text-gray-900 dark:text-white flex flex-col">
  <div>
    <p>Registra tus platillos</p>
  </div>

  <div class="mt-5 flex flex-row gap-2 flex-wrap">
    <button class="btn-realista" onclick={()=>modalAbiertoCategoria = true}>Agregar categoría</button>
    <button class="btn-realista">Agregar platillo</button>
  </div>

  <div class="mt-5 w-full flex-1 overflow-auto bg-gray-200 dark:bg-gray-900">
    <Tree nodo={nodoPadre} onSeleccionar={onSeleccionar} selectedId={selectedNode} />
  </div>
</main>

<ModalAddName
  bind:abierto={modalAbiertoCategoria}
  titulo="Nueva categoría"
  descripcion="Escribe el nombre de la categoría que quieres agregar."
  placeholder="Ej. Desayunos"
  onConfirmar={addCategory}
/>