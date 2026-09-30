<script lang="ts">
  import Menubar from "$lib/components/menubar/Menubar.svelte";
  import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { invoke } from "@tauri-apps/api/core";

  function onRegistrarOrden(){
    const value = `registrar-orden-${crypto.randomUUID()}`;
    const objRegisterOrden = {title: "Registrar orden", url: "/registrar-orden", height: 800, width: 500,};
    new WebviewWindow(value, {...objRegisterOrden, center: true, visible: true});
    const paramEnfocarVentana = {label: value};
    invoke("enfocar_ventana", paramEnfocarVentana);
  }

  function onVerOrdenesPendientes(){
    const value = `ver-ordenes-pendientes-${crypto.randomUUID()}`;
    const objVerOrdenesPendientes = {title: "Ver ordenes pendientes", url: "/ver-ordenes-pendientes", height: 800, width: 1200,};
    new WebviewWindow(value, {...objVerOrdenesPendientes, center: true, visible: true});
    const paramEnfocarVentana = {label: value};
    invoke("enfocar_ventana", paramEnfocarVentana);
  }

  function onVerTodasOrdenes(){
    const value = `ver-ordenes-general-${crypto.randomUUID()}`;
    const objVerOrdenesGeneral = {title: "Ver ordenes general", url: "/ver-ordenes-general", height: 800, width: 1200,};
    new WebviewWindow(value, {...objVerOrdenesGeneral, center: true, visible: true});
    const paramEnfocarVentana = {label: value};
    invoke("enfocar_ventana", paramEnfocarVentana);
  }

</script>

<main class="w-full h-screen flex flex-col overflow-hidden">
  <Menubar />
  <div class="flex-1 w-full bg-white dark:bg-black overflow-auto">

    <div>
      <button class="btn-realista" onclick={onRegistrarOrden}>Registrar orden</button>
      <button class="btn-realista" onclick={onVerOrdenesPendientes}>Ver ordenes pendientes</button>
      <button class="btn-realista" onclick={onVerTodasOrdenes}>Ver ordenes general</button>
    </div>

  </div>
</main>