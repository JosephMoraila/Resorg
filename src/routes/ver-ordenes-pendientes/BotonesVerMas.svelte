<script lang="ts">

    import type { Pedido, FiltrosVerOrdenesProps, MetodoPago } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { toast } from "$lib/toast.svelte";
    import ModalCobrar from "$lib/components/modals/ModalCobrar.svelte";

    interface Props{
        totalPedidosPendientes: number; selectedId: number | null; pedidosPendientes: Pedido[]; obtenerPedidosPendientes(pagina?: number, filtros?: FiltrosVerOrdenesProps): Promise<void>; obtenerCountPedidosPendientes(filtros?: FiltrosVerOrdenesProps): Promise<void>;
    }

    let {totalPedidosPendientes, selectedId, pedidosPendientes, obtenerCountPedidosPendientes, obtenerPedidosPendientes}:Props = $props();

    let onModalCobrar = $state(false);
    let modalPedidoPendiente = $state<null | Pedido>(null);
    async function onCobrar(metodoPago: MetodoPago, contenidoTicket: string) {
        if(modalPedidoPendiente === null) return;
        const params = {pedido: modalPedidoPendiente, metodoPago, infoTocket: contenidoTicket};
        try{
            await invoke("cobrar_pedido", params);
            toast.verde("Cobrado correctamente");
            //Actualizamos la tabla
            obtenerCountPedidosPendientes();
            obtenerPedidosPendientes();
            selectedId = null;
        }catch (error) {
            const err = error as string;
            toast.rojo(`Error cobrar: ${err}`);
        }
    }

    $effect(()=>{
        const pedidoPendiente = pedidosPendientes.find(pp=>pp.id === selectedId);
        if(pedidoPendiente === undefined){
            modalPedidoPendiente = null;
            return;
        }
        modalPedidoPendiente = pedidoPendiente;
    });

    async function onVerMas() {
        if(selectedId === null) return;
        const pedidoPendiente = pedidosPendientes.find(pp=>pp.id === selectedId);
        if(pedidoPendiente === undefined) return;
        const valueWindow = `ver-orden-info-${selectedId}`;
        const objVerOrdenTodo = {title: `Ver orden ID: ${selectedId}`, url: "/ver-orden-info", height: 800, width: 1200,};
        try{
            const params = {clave:valueWindow, valor: pedidoPendiente};
            await invoke("guardar_pedido_compartido", params);
            new WebviewWindow(valueWindow, {...objVerOrdenTodo, center: true, visible: true});
            const paramEnfocarVentana = {label: valueWindow};
            invoke("enfocar_ventana", paramEnfocarVentana);
        }catch (error) {
            const err = error as string;
            console.error("Error al abrir ventana", err);
            toast.rojo(`Error al abrir ventana: ${err}`);
        }
    }

    function onPagar(){
        onModalCobrar = true;

    }

</script>

<div class="flex flex-row items-center gap-4 flex-nowrap overflow-x-auto pb-2 w-full max-w-full">
    <span>Encontrado: {totalPedidosPendientes}</span>
    <button onclick={onVerMas} class="btn-realista py-1! disabled:cursor-not-allowed! disabled:opacity-50" disabled={selectedId === null}>Ver</button>
    <button onclick={onPagar} class="btn-realista py-1! disabled:cursor-not-allowed! disabled:opacity-50" disabled={selectedId === null}>Cobrar</button>
</div>

{#if onModalCobrar && modalPedidoPendiente !== null}
    <ModalCobrar bind:isAbierto={onModalCobrar} pedidoPendiente={modalPedidoPendiente} onCobrar={onCobrar}/>
{/if}