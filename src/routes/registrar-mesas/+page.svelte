<script lang="ts">

  import ModalAddName from "$lib/components/modals/ModalAddName.svelte";
  import ModalTwoInputs from "$lib/components/modals/ModalTwoInputs.svelte";
  import type { Piso, NodoArbol } from "$lib/types";
  import Tree from "$lib/components/tree/Tree.svelte";
  import { toast } from "$lib/toast.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import ModalAccept from "$lib/components/modals/ModalAccept.svelte";
  import { getCurrentWebviewWindow, WebviewWindow } from "@tauri-apps/api/webviewWindow";

  let modalValuePiso = $state("");
  let isModalAbiertoPiso = $state(false);

  let modalValue1Edit = $state("");
  let modalValue2Edit = $state("");
  let isModalAbiertoEdit = $state(false);

  let abietoAceptarEliminar = $state(false);

  let selectedNode = $state<string | null>(null);
  function getNodoObject(selectedId: string): NodoArbol<Piso> | undefined{
    const desnudo = raices.find(n=>n.id === selectedId);
    return desnudo;
  }

  let raices: NodoArbol<Piso>[] = $state([]);

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

  onMount(()=>{
    const getData = async()=>{
      try{
        const data = await invoke<Piso[]>("get_floor_and_mesas");
        for(const d of data){
          const pisito = d.piso;
          const mesitas = d.numero_mesas;
          const nodo: NodoArbol<Piso> = {id: `piso-${pisito}`, label: `Piso ${pisito} - Mesas: ${mesitas}`, data: d};
          raices.push(nodo);
        }
        window.addEventListener("keydown", onKeydownFueraSelectNull);
        return () => window.removeEventListener("keydown", onKeydownFueraSelectNull);
      }catch(error){
        const err = error as string;
        toast.rojo(`Error al obtener información: ${err}`);
      }
    }
    getData();
    onMostrarVentanaAlMontarse();
  });

  function abrirModalCrearPiso(){
    isModalAbiertoPiso = true;
    modalValuePiso = "";
  }

  function pisoYaRegistrado(pisoParam: number): boolean{
    let esta = false;
    for(const piso of raices){
      const pisoFor = piso.data.piso;
      if(pisoFor === pisoParam) esta = true;
    }
    return esta;
  }

  async function onConfirmrNumeroPiso(numero: string | null){
    if(numero == null || numero == "") return;
    const numeroNumber = Number(numero);
    if(Number.isNaN(numeroNumber)){
      toast.amarillo("Ingrese un número");
      return;
    }
    const integerNumber = Math.trunc(numeroNumber);
    if(pisoYaRegistrado(integerNumber)){
      toast.amarillo("Piso ya registrado");
      return;
    }

    try{
      await invoke("insert_floor", {floorNumber: integerNumber});
      const nuevoPiso: Piso = {piso: integerNumber, numero_mesas: 1};
      const nodoNuevoPiso: NodoArbol<Piso> = {id: `piso-${integerNumber}`, label: `Piso: ${integerNumber} - Mesas: 1`, data: nuevoPiso};
      raices.push(nodoNuevoPiso);
    }catch(error){
      const err = error as string;
      toast.rojo(`Error al insertar piso: ${err}`);
      return;
    }
    

  }

    function onKeydownFueraSelectNull(e: KeyboardEvent) {
      if (e.key === "Escape") {
        selectedNode = null;
      }
    }

  function onSelectedNode(nodo: NodoArbol<unknown>){
    if(selectedNode === nodo.id) return; // No hacer nada si el nodo seleccionado es el mismo
    selectedNode = nodo.id;
    const desnudo = getNodoObject(selectedNode);
    if(!desnudo) return;
  }

  function onEditar(){
    if(!selectedNode) return;
    let desnudo = getNodoObject(selectedNode);
    if(!desnudo) return;
    const pisito = desnudo.data.piso;
    const mesitas = desnudo.data.numero_mesas;
    modalValue1Edit = String(pisito);
    modalValue2Edit = String(mesitas);
    isModalAbiertoEdit = true;
  }

  async function onConfirmarEdicion(piso: string | null, mesas: string | null){
    if(selectedNode == null) return;
    let desnudo = getNodoObject(selectedNode);
    if(!desnudo) return;
    
    if(piso == null && mesas == null) return;
    else if(piso != null && mesas == null){
      toast.amarillo("Mesas no tiene valor");
      return;
    }else if(piso == null && mesas != null){
      toast.amarillo("Piso no tiene valor");
      return;
    }else if(piso != null && mesas != null){
      const pisoNumber = Number(piso);
      const mesasNumber = Number(mesas);
      if(Number.isNaN(pisoNumber) || Number.isNaN(mesasNumber)){
        toast.amarillo("Ingrese valores númericos");
        return;
      }
      const integerNumberPiso = Math.trunc(pisoNumber);
      const inetegerNumberMesas = Math.trunc(mesasNumber);
      if(inetegerNumberMesas < 0){
        toast.amarillo("No tiene sentido tener un número negativo de mesas");
        return;
      }
      const oldMesas = desnudo.data.numero_mesas;
      if(pisoYaRegistrado(integerNumberPiso) && oldMesas === inetegerNumberMesas){
        toast.amarillo("Piso ya registrado");
        return;
      }
      const oldPiso = desnudo.data.piso;
      const paramsFloor = {newFloorNumber: integerNumberPiso, oldFloorNumber: oldPiso};
      const paramsMesas = {floorNumber: integerNumberPiso, mesas: inetegerNumberMesas};
      try{
        await invoke("update_floor", paramsFloor);
        await invoke("update_mesas", paramsMesas);
        desnudo.data.piso = integerNumberPiso;
        desnudo.data.numero_mesas = inetegerNumberMesas;
        desnudo.id = `piso-${integerNumberPiso}`;
        desnudo.label = `Piso ${integerNumberPiso} - Mesas: ${inetegerNumberMesas}`;
        toast.verde("Actualización hecha");
      }catch(error){
        const err = error as string;
        toast.rojo(`Error al actualizar información: ${err}`);
      }
    }
  }

  async function onDelete(result: boolean){
    if(!result){
      abietoAceptarEliminar = false;
      return;
    }
    if(selectedNode == null) return;
    let desnudo = getNodoObject(selectedNode);
    if(!desnudo) return;
    const piso = desnudo.data.piso;
    const params = {floorNumber: piso};
    try{
      await invoke("delete_piso", params);
      const newPisos = raices.filter(p=>p.data.piso !== piso);
      raices = newPisos;
      selectedNode = null;
      toast.verde("Piso eliminado");
    }catch(error){
        const err = error as string;
        toast.rojo(`Error al eliminar piso ${err}`);
      }finally{
        abietoAceptarEliminar = false;
      }
  }
  

</script>


<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<main class="w-screen h-screen p-6 bg-white dark:bg-[#010101] text-gray-900 dark:text-white flex flex-col">

  <div>
    <p>Registra mesas</p>
  </div>

  <div class="mt-5 flex flex-row gap-2 flex-wrap">
    <button class="btn-realista" onclick={abrirModalCrearPiso}>Agregar piso</button>
    <button class="btn-realista" onclick={onEditar} class:opacity-50={selectedNode === null} class:cursor-not-allowed={selectedNode === null} disabled={selectedNode === null} >Editar</button>
    <button class="btn-realista" onclick={()=>abietoAceptarEliminar = true} class:opacity-50={selectedNode === null} class:cursor-not-allowed={selectedNode === null} disabled={selectedNode === null}>Eliminar</button>
  </div>

  <div class="mt-5 w-full flex-1 overflow-auto bg-gray-200 dark:bg-gray-900">
    <Tree nodos={raices} onSeleccionar={onSelectedNode} selectedId={selectedNode}/>
  </div>

</main>

<ModalAddName bind:abierto={isModalAbiertoPiso}
  bind:valor={modalValuePiso} 
  titulo={"Agregar piso"}
  descripcion="Número de piso"
  placeholder="3"
  onConfirmar={onConfirmrNumeroPiso}
/>

<ModalTwoInputs bind:abierto={isModalAbiertoEdit}
  bind:valor1={modalValue1Edit} bind:valor2={modalValue2Edit}
  descripcion1="Piso" descripcion2="Mesas"
  titulo="Editat"
  onConfirmar={onConfirmarEdicion}
/>

<ModalAccept abierto={abietoAceptarEliminar}
  title="Eliminar piso" message="¿Estás seguro que deseas eliminar el piso?"
  onResult={onDelete}
/>