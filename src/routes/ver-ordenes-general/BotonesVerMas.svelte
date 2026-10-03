<script lang="ts">

    import type { Pedido, FiltrosVerOrdenesProps} from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { toast } from "$lib/toast.svelte";
    import { quitarTodosLosEspacios } from "$lib/utils/string_utils";

    interface Props{
        totalPedidos: number; selectedId: number | null; pedidos: Pedido[]; filters: FiltrosVerOrdenesProps;
    }

    let {totalPedidos, selectedId, pedidos, filters}:Props = $props();


    async function onVerMas() {
        if(selectedId === null) return;
        const pedidoPendiente = pedidos.find(pp=>pp.id === selectedId);
        if(pedidoPendiente === undefined) return;
        const valueWindow = `ver-orden-info-${selectedId}`;
        const objVerOrdenTodo = {title: `Ver orden ID: ${selectedId}`, url: "/ver-orden-info", height: 800, width: 1200,};
        try{
            const params = {clave:valueWindow, valor: pedidoPendiente};
            await invoke("guardar_pedido_compartido", params);
            new WebviewWindow(valueWindow, {...objVerOrdenTodo, center: true, visible: false});
            const paramEnfocarVentana = {label: valueWindow};
            invoke("enfocar_ventana", paramEnfocarVentana);
        }catch (error) {
            const err = error as string;
            console.error("Error al abrir ventana", err);
            toast.rojo(`Error al abrir ventana: ${err}`);
        }
    }

    async function onVerGraficas(){
        if(pedidos.length === 0) return;

        //El value del windows no funciona si tiene espacios
        const stringFiltros = `${filters.id}-${filters.tipoPedido}-${quitarTodosLosEspacios(filters.nombreCliente)}-${quitarTodosLosEspacios(filters.fechaInicio)}-${quitarTodosLosEspacios(filters.fechaFin)}-${filters.totalDesde}-${filters.totalHasta}-${quitarTodosLosEspacios(filters.nota)}-${filters.estatus}-${filters.metodoPago}`;
        console.log(stringFiltros);
        const valueWindow = `ver-graficos-ordenes-${stringFiltros}`;
        const params = {clave: valueWindow, pedidos, filtros: filters};
        try{
            await invoke("guardar_pedidos_filtros_compartido", params);
            const objVerGraficos = {title: `Ver gráficos`, url: "/ver-graficos-ordenes", height: 800, width: 1200,};
            new WebviewWindow(valueWindow, {...objVerGraficos, center: true, visible: false});
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
    <button onclick={onVerGraficas} class="btn-realista py-1! disabled:cursor-not-allowed! disabled:opacity-50" disabled={pedidos.length === 0}>Gráficos</button>
</div>

