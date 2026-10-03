<script lang="ts">
  import Tree from "$lib/components/tree/Tree.svelte";
  import type { NodoArbol, PlatilloCategoria, Platillo } from "$lib/types";
  import ModalAddName from "$lib/components/modals/ModalAddName.svelte";
  import ModalAddPlatillo from "$lib/components/modals/ModalAddPlatillo.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { toast } from "$lib/toast.svelte";
  import { formatearMoneda } from "$lib/utils/string_utils";
  import { previewUrlAUint8 } from "$lib/utils/file_utils";
  import InfoBox from "$lib/components/infobox/InfoBox.svelte";
  import { obtenerImagenPlatillo } from "$lib/utils/file_utils";
  import ModalAccept from "$lib/components/modals/ModalAccept.svelte";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

  let modalCategoriaNombre = $state("");
  let modalAccionCategoria = $state<"editar" | "crear" | null>(null);
  let modalAbiertoCategoria = $state(false);
  let nodoCategoriaSiendoEditado: NodoArbol<PlatilloCategoria> | null = $state(null);

  let modalAbiertoPlatillo = $state(false);
  let modalAccionPlatillo = $state<"editar" | "crear" | null>(null);
  let nodoPlatilloSiendoEditado: NodoArbol<Platillo> | null = $state(null);
  let valorInicialPlatillo = $state<{nombre: string, descripcion: string | null, precio: number, imagenUrl: string | null} | null>(null);

  let selectedNode = $state<string | null>(null);
  let infoboxAbierto = $state(false);
  let textoInfoBox = $state("");
  let imagenInfoBox = $state<string | null>(null);

  let modalConfirmacionAbierto = $state(false);
  let titleModalConfirmacion = $state("");
  let messageModalConfirmacion = $state("");
  let nodoSiendoEliminado: NodoArbol<PlatilloCategoria | Platillo> | null = $state(null);

  let nodoPadre: NodoArbol<PlatilloCategoria> = $state({
    id: "cat-0",
    label: "Platillos",
    data: { tipo: "categoria",id: 0, nombre: "Platillos", platillos: [] },
    hijos: [],
  });

  function platilloANodo(platillo: Platillo): NodoArbol<Platillo> {
    return {
      id: `plat-${platillo.id}`,
      label: `${platillo.nombre} - $${formatearMoneda(platillo.precio)}`,
      data: {...platillo, tipo: "platillo"},
    };
  }

  function categoriaANodo(categoria: PlatilloCategoria): NodoArbol<PlatilloCategoria> {
    return {
      id: `cat-${categoria.id}`,
      label: categoria.nombre,
      data: {...categoria, tipo: "categoria"},
      hijos: categoria.platillos.map(platilloANodo),
    };
  }

  async function onMostrarVentanaAlMontarse() {
    try{
        const label = getCurrentWebviewWindow().label;
        const ventana = await WebviewWindow.getByLabel(label);
        await ventana?.show();
    }catch(err){
        const error = err as string;
        toast.rojo(`Error al mostrar ventana: ${error}`);
    }
  }

  onMount(() => {
    async function getCategories() {
      try {
        const categoriasBackend = await invoke<PlatilloCategoria[]>("get_categories_platillo");
        console.log(`Tipo cat: ${categoriasBackend[0].tipo}`);

        const raiz = categoriasBackend.find(cat => cat.id === 0);
        const otras = categoriasBackend.filter(cat => cat.id !== 0);

        const nodosPlatillosRaiz: NodoArbol<Platillo>[] = (raiz?.platillos ?? []).map(platilloANodo);
        const nodosCategorias: NodoArbol<PlatilloCategoria>[] = otras.map((cat) => categoriaANodo(cat));

        // Reasignación para activar la reactividad de Svelte 5
        nodoPadre.hijos = [...nodosPlatillosRaiz, ...nodosCategorias];

        window.addEventListener("keydown", onKeydownFueraSelectNull);
        return () => window.removeEventListener("keydown", onKeydownFueraSelectNull);
      } catch (error) {
        const err = error as string;
        toast.rojo(`Error al obtener información: ${err}`);
      }finally {
        await onMostrarVentanaAlMontarse();
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

  function getCategoryNodeById(h: NodoArbol<unknown>,id: number): NodoArbol<PlatilloCategoria> | undefined{
    const data = h.data as Platillo | PlatilloCategoria;
    if(data.tipo === "categoria"){
      if(data.id === id){
        return h as NodoArbol<PlatilloCategoria>;
      }
    }
    if(h.hijos){
      for(const hijo of h.hijos){
        const res = getCategoryNodeById(hijo,id);
        if(res) return res;
      }
    }
    return undefined;
  }

  function findNodeById(h: NodoArbol<unknown>, id: string): NodoArbol<unknown> | undefined {
    if (h.id === id) return h;
    if (h.hijos) {
      for (const hijo of h.hijos) {
        const res = findNodeById(hijo, id);
        if (res) return res;
      }
    }
    return undefined;
  }

  async function addCategory(nombre: string | null) {
    if (!nombre) return;
    const nombreTrimmed = nombre.trim();
    if(modalAccionCategoria === "crear"){
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
    }else if(modalAccionCategoria === "editar" && nodoCategoriaSiendoEditado){
      if(nodoCategoriaSiendoEditado.data.nombre === nombreTrimmed){
        toast.amarillo(`El nombre de la categoría no ha cambiado`);
        return;
      }
      const n = nodoPadre.hijos?.find(nodo=>getCategorySameName(nodo, nombreTrimmed));
      if(n){
        toast.rojo(`Ya existe una categoría con ese nombre`);
        return;
      }
      try{
        await invoke("update_category_platillo", { id: nodoCategoriaSiendoEditado.data.id, name: nombreTrimmed });
        toast.verde("Categoría editada correctamente");
        nodoCategoriaSiendoEditado.label = nombreTrimmed;
        nodoCategoriaSiendoEditado.data.nombre = nombreTrimmed;
      }catch(error){
        const err = error as string;
        toast.rojo(`Error al editar categoría: ${err}`);
      }
    }
  }

  async function addPlatillo(datos: {nombre: string, descripcion: string | null, precio: number, imagen: string | null} | null) {
    if(!datos) {
      modalAbiertoPlatillo = false;
      return;
    }
    if(modalAccionPlatillo === "crear"){
      try{
        const imageBytes = datos.imagen ? await previewUrlAUint8(datos.imagen) : null;
        if(datos.imagen) URL.revokeObjectURL(datos.imagen); // Liberar memoria si se creó un objeto URL
        if(!selectedNode || selectedNode === "cat-0"){
          //Cargarlo en raiz, es decir, dentro de la categoría "Platillos"
          const lastId = await invoke<number>("insert_platillo", { nombre: datos.nombre, descripcion: datos.descripcion, precio: datos.precio, id_categoria: null, image_bytes: imageBytes });
          const nodoPlatillo: NodoArbol<Platillo> = {
            id: `plat-${lastId}`, // Generar un ID temporal único
            label: `${datos.nombre} - $${formatearMoneda(datos.precio)}`,
            data: {tipo: "platillo", id: lastId, id_categoria:0, descripcion: datos.descripcion, nombre: datos.nombre, precio: datos.precio },
          };
          nodoPadre.hijos?.push(nodoPlatillo);
        }else{
          //Verificar si el nodo seleccionado es una categoría, en ese caso agrrgarlo ahi, o si es un platillo, agregarlo a la categoría padre de ese platillo
          const esCategoria = selectedNode.startsWith("cat-");
          if(esCategoria){
            const nodoCategoria = nodoPadre.hijos?.find(nodo=>nodo.id === selectedNode);
            if(!nodoCategoria){
              toast.rojo("No se encontró la categoría seleccionada");
              return;
            }
            const idFather = (nodoCategoria.data as PlatilloCategoria).id;
            const lastId = await invoke<number>("insert_platillo", { nombre: datos.nombre, descripcion: datos.descripcion, precio: datos.precio, id_categoria: idFather, image_bytes: imageBytes });
            const nodoPlatillo: NodoArbol<Platillo> = {
              id: `plat-${lastId}`, // Generar un ID temporal único
              label: `${datos.nombre} - $${formatearMoneda(datos.precio)}`,
              data: {tipo: "platillo", id: lastId, id_categoria:idFather, descripcion: datos.descripcion, nombre: datos.nombre, precio: datos.precio }, // ID temporal
            }
            nodoCategoria.hijos?.push(nodoPlatillo);
          }else{
            //Es un platillo, entonces buscar la categoría padre de ese platillo
            const nodoPlatillo = findNodeById(nodoPadre, selectedNode);
            if(!nodoPlatillo){
              toast.rojo("No se encontró el platillo seleccionado");
              return;
            }
            const idCategoria = (nodoPlatillo.data as Platillo).id_categoria;
            const cat = getCategoryNodeById(nodoPadre,idCategoria);
            if(!cat){
              toast.rojo("No se encontró la categoría del platillo seleccionado");
              return;
            }
            const lastId = await invoke<number>("insert_platillo", { nombre: datos.nombre, descripcion: datos.descripcion, precio: datos.precio, id_categoria: idCategoria, image_bytes: imageBytes });
            const nodoNuevoPlatillo: NodoArbol<Platillo> = {
              id: `plat-${lastId}`, // Generar un ID temporal único
              label: `${datos.nombre} - $${formatearMoneda(datos.precio)}`,
              data: {tipo: "platillo", id: lastId, id_categoria:idCategoria, descripcion: datos.descripcion, nombre: datos.nombre, precio: datos.precio }, // ID temporal
            }
            cat.hijos?.push(nodoNuevoPlatillo);
          }
        }
        toast.verde("Platillo agregado correctamente");
      }
        catch(error){
          const err = error as string;
          toast.rojo(`Error al insertar platillo: ${err}`);
          return;
        }
    }else if(modalAccionPlatillo === "editar" && nodoPlatilloSiendoEditado){
      try{
        const imageBytes = datos.imagen ? await previewUrlAUint8(datos.imagen) : null;
        if(datos.imagen) URL.revokeObjectURL(datos.imagen); // Liberar memoria si se creó un objeto URL

        const idPlatillo = (nodoPlatilloSiendoEditado.data as Platillo).id;
        await invoke("update_platillo", { id: idPlatillo, nombre: datos.nombre, descripcion: datos.descripcion, precio: datos.precio, image_bytes: imageBytes });

        nodoPlatilloSiendoEditado.label = `${datos.nombre} - $${formatearMoneda(datos.precio)}`;
        nodoPlatilloSiendoEditado.data.nombre = datos.nombre;
        (nodoPlatilloSiendoEditado.data as Platillo).descripcion = datos.descripcion;
        (nodoPlatilloSiendoEditado.data as Platillo).precio = datos.precio;

        toast.verde("Platillo editado correctamente");
      }catch(error){
        const err = error as string;
        toast.rojo(`Error al editar platillo: ${err}`);
        return;
      }
    }
    modalAbiertoPlatillo = false;
  }

  function onSeleccionar(nodo: NodoArbol<unknown>){
    if(selectedNode === nodo.id) return; // No hacer nada si el nodo seleccionado es el mismo
    selectedNode = nodo.id;
    const node = findNodeById(nodoPadre, selectedNode);

    // Siempre libera la imagen anterior antes de cualquier otra cosa
    if (imagenInfoBox) {
      URL.revokeObjectURL(imagenInfoBox);
      imagenInfoBox = null;
    }

    if(!node){
      infoboxAbierto = false;
      textoInfoBox = "";
      return;
    }

    const data = node.data as Platillo | PlatilloCategoria;

    if(data.tipo === "categoria"){
      if(data.id === 0){
        textoInfoBox = `Esta es la categoría raíz, no se puede eliminar ni modificar.`;
      } else {
        textoInfoBox = `Categoría: ${data.nombre}`;
      }
      infoboxAbierto = true;
    } else {
      textoInfoBox = `Platillo: ${data.nombre}\nPrecio: $${formatearMoneda(data.precio)}\nDescripción: ${data.descripcion ?? ""}`;
      infoboxAbierto = true;

      const idSeleccionadoAlPedir = selectedNode; // snapshot para evitar condición de carrera
      obtenerImagenPlatillo(data.id).then((url) => {
        if (selectedNode !== idSeleccionadoAlPedir) {
          // el usuario ya seleccionó otra cosa mientras cargaba; descarta este resultado
          if (url) URL.revokeObjectURL(url);
          return;
        }
        imagenInfoBox = url;
      });
    }
  }

  function onclickFueraSelectNull(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      selectedNode = null;
      infoboxAbierto = false;
      if(imagenInfoBox) {
        URL.revokeObjectURL(imagenInfoBox);
        imagenInfoBox = null;
      }
    }
  }

  function onKeydownFueraSelectNull(e: KeyboardEvent) {
    if (e.key === "Escape") {
      selectedNode = null;
      infoboxAbierto = false;
      if (imagenInfoBox) {
        URL.revokeObjectURL(imagenInfoBox);
        imagenInfoBox = null;
      }
    }
  }

  function onCrearCategoriaClick() {
    modalCategoriaNombre = "";
    modalAccionCategoria = "crear";
    nodoCategoriaSiendoEditado = null;
    modalAbiertoCategoria = true;
  }

  function onCrearPlatilloClick() {
    modalAccionPlatillo = "crear";
    nodoPlatilloSiendoEditado = null;
    valorInicialPlatillo = null;
    modalAbiertoPlatillo = true;
  }

  function onEditarClick() {
    if (!selectedNode) return;
    const node = findNodeById(nodoPadre, selectedNode);
    if (!node) return;
    selectedNode = null;
    infoboxAbierto = false;
    const data = node.data as Platillo | PlatilloCategoria;

    if (data.tipo === "categoria") {
      if(data.id === 0) return; // No se puede editar la categoría raíz
      modalCategoriaNombre = data.nombre;
      nodoCategoriaSiendoEditado = node as NodoArbol<PlatilloCategoria>;
      modalAccionCategoria = "editar";
      modalAbiertoCategoria = true;
    } else {
      nodoPlatilloSiendoEditado = node as NodoArbol<Platillo>;
      modalAccionPlatillo = "editar";
      valorInicialPlatillo = {
        nombre: data.nombre,
        descripcion: data.descripcion,
        precio: data.precio,
        imagenUrl: null,
      };
      modalAbiertoPlatillo = true;

      // La imagen se carga aparte porque es asíncrona; se le agrega al valorInicial cuando llegue
      const idAlAbrir = data.id;
      obtenerImagenPlatillo(idAlAbrir).then((url) => {
        if (nodoPlatilloSiendoEditado !== node) return; // el usuario ya cerró/cambió de edición
        if (valorInicialPlatillo) {
          valorInicialPlatillo = { ...valorInicialPlatillo, imagenUrl: url };
        }
      });
    }
  }

  function onEliminarClick() {
    if (!selectedNode) return;
    const node = findNodeById(nodoPadre, selectedNode);
    if (!node) return;
    const data = node.data as Platillo | PlatilloCategoria;

    if (data.tipo === "categoria") {
      if(data.id === 0) return;
      nodoSiendoEliminado = node as NodoArbol<PlatilloCategoria>;
      titleModalConfirmacion = `Eliminar categoría: ${data.nombre}`;
      messageModalConfirmacion = "¿Estás seguro de que quieres eliminar esta categoría? Todos los platillos dentro de ella también serán eliminados.";
      modalConfirmacionAbierto = true;
    } else {
      nodoSiendoEliminado = node as NodoArbol<Platillo>;
      titleModalConfirmacion = `Eliminar platillo: ${data.nombre}`;
      messageModalConfirmacion = "¿Estás seguro de que quieres eliminar este platillo?";
      modalConfirmacionAbierto = true;
    }
  }


function onModalConfirmacionResult(result: boolean) {
  if (result && nodoSiendoEliminado) {
    const nodoAEliminar = nodoSiendoEliminado; //snapshot, capturado ANTES de que se ponga en null
    const data = nodoAEliminar.data as Platillo | PlatilloCategoria;

    if (data.tipo === "categoria") {
      invoke("delete_category_platillo", { id: data.id })
        .then(() => {
          nodoPadre.hijos = nodoPadre.hijos?.filter(nodo => nodo !== nodoAEliminar) ?? [];
          toast.verde("Categoría eliminada correctamente");
        })
        .catch((error) => {
          const err = error as string;
          toast.rojo(`Error al eliminar categoría: ${err}`);
        });
    } else {
      invoke("delete_platillo", { id: data.id })
        .then(() => {
          const categoriaNode = getCategoryNodeById(nodoPadre, data.id_categoria);
          if (categoriaNode) {
            categoriaNode.hijos = categoriaNode.hijos?.filter(nodo => nodo !== nodoAEliminar) ?? [];
          } else {
            nodoPadre.hijos = nodoPadre.hijos?.filter(nodo => nodo !== nodoAEliminar) ?? [];
          }
          toast.verde("Platillo eliminado correctamente");
        })
        .catch((error) => {
          const err = error as string;
          toast.rojo(`Error al eliminar platillo: ${err}`);
        });
    }
  }
  selectedNode = null;
  nodoSiendoEliminado = null; // ahora sí puedes limpiarlo de inmediato sin romper nada
  modalConfirmacionAbierto = false;
}

</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<main onclick={onclickFueraSelectNull} onkeydown={onKeydownFueraSelectNull} class="w-screen h-screen p-6 bg-white dark:bg-[#010101] text-gray-900 dark:text-white flex flex-col">
  <div>
    <p>Registra tus platillos</p>
  </div>

  <div class="mt-5 flex flex-row gap-2 flex-wrap">
    <button class="btn-realista" onclick={onCrearCategoriaClick}>Agregar categoría</button>
    <button class="btn-realista"onclick={onCrearPlatilloClick} >Agregar platillo</button>
    <button class="btn-realista" class:opacity-50={selectedNode === null} class:cursor-not-allowed={selectedNode === null} disabled={selectedNode === null} onclick={onEditarClick}>Editar</button>
    <button class="btn-realista" class:opacity-50={selectedNode === null} class:cursor-not-allowed={selectedNode === null} disabled={selectedNode === null} onclick={onEliminarClick} >Eliminar</button>
  </div>

  <div class="mt-5 w-full flex-1 overflow-auto bg-gray-200 dark:bg-gray-900">
    <Tree nodo={nodoPadre} onSeleccionar={onSeleccionar} selectedId={selectedNode} />
  </div>
</main>

<ModalAddName
  bind:abierto={modalAbiertoCategoria}
  bind:valor={modalCategoriaNombre}
  titulo={modalAccionCategoria === "editar" ? "Editar categoría" : "Nueva categoría"}
  descripcion="Escribe el nombre de la categoría."
  placeholder="Ej. Desayunos"
  onConfirmar={addCategory}
/>

<ModalAddPlatillo 
  bind:abierto={modalAbiertoPlatillo}
  accion={modalAccionPlatillo ?? "crear"}
  valorInicial={valorInicialPlatillo}
  onAceptar={addPlatillo}
/>

<InfoBox
  visible ={infoboxAbierto}
  texto={textoInfoBox}
  imagen={imagenInfoBox}
/>

<ModalAccept
  abierto={modalConfirmacionAbierto}
  message={messageModalConfirmacion}
  title={titleModalConfirmacion}
  onResult={onModalConfirmacionResult}
/>