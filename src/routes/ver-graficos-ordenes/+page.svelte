<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { onMount } from 'svelte';
    import { obtenerTextoTipoPedido, formatearFechaBonito,formatearFechaTalComoViene, formatearMoneda, obtenerTextoEstadoPedido, obtenerTextoMetodoPago } from '$lib/utils/string_utils';
    import type { FiltrosVerOrdenesProps, Pedido, ConteoPedidosPorFecha } from '$lib/types';
	import { BarChart, Tooltip } from 'layerchart';
    import getListPedidosByDate from '$lib/utils/grafica_utils';
    import PedidosPorDia from './PedidosPorDia.svelte';
	
    let filtros = $state<FiltrosVerOrdenesProps | null>(null);
    let pedidos = $state<Pedido[]>([]);
    let textFiltros = $state("");

    let listPedidosFecha = $state<ConteoPedidosPorFecha[]>([]);

    async function obtenerDatosGraficaPedidos(clave: string): Promise<[FiltrosVerOrdenesProps, Pedido[]] | null>{
        const params = {clave};
        const resultado = await invoke<[FiltrosVerOrdenesProps, Pedido[]] | null>('obtener_pedidos_filtros_compartido', params);
        return resultado;
    }

    onMount(()=>{
        const ventanaActual = getCurrentWindow();
        const label = ventanaActual.label;
        const getData = async ()=>{
            const res = await obtenerDatosGraficaPedidos(label);
            if(res === null) return;
            filtros = res[0];
            pedidos = res[1];
            console.log(pedidos);
            getFiltrosString();
            listPedidosFecha = getListPedidosByDate(pedidos);
        };
        getData();
    });

    function getFiltrosString(){
        let text = "";
        if(filtros === null) return text;
        if(filtros.id !== null){
            text = `ID: ${filtros.id}`;
        }else{
            if(filtros.tipoPedido !== null) text += `Tipo pedido: ${obtenerTextoTipoPedido(filtros.tipoPedido)} | `;
            if(filtros.nombreCliente !== null) text += `Nombre cliente: ${filtros.nombreCliente} | `;
            if(filtros.fechaInicio !== null) text += `Fecha inicio: ${formatearFechaTalComoViene(filtros.fechaInicio)} | `;
            if(filtros.fechaFin !== null) text += `Fecha fin: ${formatearFechaTalComoViene(filtros.fechaFin)} | `;
            if(filtros.totalDesde !== null) text += `Monto desde: ${formatearMoneda(filtros.totalDesde)} | `;
            if(filtros.totalHasta !== null) text += `Monto hasta: ${formatearMoneda(filtros.totalHasta)} | `;
            if(filtros.nota !== null) text += `Nombre cliente: ${filtros.nota} | `;
            if(filtros.estatus !== null) text += `Tipo pedido: ${obtenerTextoEstadoPedido(filtros.estatus)} | `;
            if(filtros.metodoPago !== null) text += `Tipo pedido: ${obtenerTextoMetodoPago(filtros.metodoPago)} | `;
        }
        textFiltros = text;
    }

</script>

<main class="min-h-screen min-w-screen w-full bg-white dark:bg-black text-black dark:text-white flex flex-col items-center overflow-x-hidden">

    <div>
        <p>Filtros:</p>
        <p>{textFiltros}</p>
    </div>

    <PedidosPorDia listPedidosFecha={listPedidosFecha}/>
    

</main>

