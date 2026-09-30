<script lang="ts">

    import type { Pedido,} from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { toast } from "$lib/toast.svelte";

    interface Props{
        totalPedidos: number; selectedId: number | null; pedidos: Pedido[];
    }

    let {totalPedidos, selectedId, pedidos, }:Props = $props();


    async function onVerMas() {
        if(selectedId === null) return;
        const pedidoPendiente = pedidos.find(pp=>pp.id === selectedId);
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


</script>

<div class="flex flex-row items-center gap-4 flex-nowrap overflow-x-auto pb-2 w-full max-w-full">
    <span>Encontrado: {totalPedidos}</span>
    <button onclick={onVerMas} class="btn-realista py-1! disabled:cursor-not-allowed! disabled:opacity-50" disabled={selectedId === null}>Ver</button>
</div>

