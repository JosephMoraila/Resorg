<script lang="ts">
    import { toast } from '$lib/toast.svelte';
    import { WebviewWindow, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
    import { invoke } from '@tauri-apps/api/core';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { onMount } from 'svelte';
    import { obtenerTextoTipoPedido,formatearFechaTalComoViene, formatearMoneda, obtenerTextoEstadoPedido, obtenerTextoMetodoPago } from '$lib/utils/string_utils';
    import type { FiltrosVerOrdenesProps, Pedido, ConteoPedidosPorFecha, ConteoPedidosPorTipo, ConteoTotalRecaudadoPorFecha, ConteoTotalRecaudadoPorTipoPorFecha,ConteoEstadoPedidosPorFecha, ConteoPlatillosMasVendidosPorFecha, GeneralTipoPedidoPie, GeneralEstadoPedidoPie, GeneralPlatillosPie } from '$lib/types';
    import { getListTiposPedidosByDate, getListPedidosByDate,getTotalRecaudadoGeneral, getListTotalRecaudadoByDate, getListEstadosPedidosByDate, getListTotalRecaudadoByTipoByDate, getListPlatillosMasVendidosByDate, getGeneralTipoPedidoPie, getGeneralEstadoPedidoPie, getGeneralPlatillosPie } from '$lib/utils/grafica_utils';
    import PedidosPorDia from './PedidosPorDia.svelte';
    import TipoPorDia from './TipoPorDia.svelte';
    import TotalRecaudadoPorDia from './TotalRecaudadoPorDia.svelte';
    import EstadosPorFecha from './EstadosPorFecha.svelte';
    import TotalRecaudadoPorTipo from './TotalRecaudadoPorTipo.svelte';
    import PlatillosMasVendidosPorFecha from './PlatillosMasVendidosPorFecha.svelte';

    import TiposGeneral from './PieGeneral/TiposGeneral.svelte';
    import EstadosGeneral from './PieGeneral/EstadosGeneral.svelte';
    import PlatillosGenerales from './PieGeneral/PlatillosGenerales.svelte';

    let filtros = $state<FiltrosVerOrdenesProps | null>(null);
    let pedidos = $state<Pedido[]>([]);
    let textFiltros = $state("");

    let listPedidosFecha = $state<ConteoPedidosPorFecha[]>([]);
    let listTiposPedidosFecha = $state<ConteoPedidosPorTipo[]>([]);
    let listTotalRecaudadoFecha = $state<ConteoTotalRecaudadoPorFecha[]>([]);
    let listEstadosPedidosFecha = $state<ConteoEstadoPedidosPorFecha[]>([]);
    let listTotalRecaudadoPorTipoPorFecha = $state<ConteoTotalRecaudadoPorTipoPorFecha[]>([]);
    let listPlatillosMasVendidosFecha = $state<ConteoPlatillosMasVendidosPorFecha[]>([]);
    let totalRecaudadoGeneral = $state<number>(0);
    let generalTipoPedidoPie = $state<GeneralTipoPedidoPie>({tipoDomicilio: 0, tipoLocal: 0, tipoRecoger: 0});
    let generalEstadoPedidoPie = $state<GeneralEstadoPedidoPie>({pendiente: 0, finalizado: 0, cancelado: 0, entregado: 0, cobrado: 0});
    let generalPlatillosPie = $state<GeneralPlatillosPie[]>([]);
    

    async function obtenerDatosGraficaPedidos(clave: string): Promise<[FiltrosVerOrdenesProps, Pedido[]] | null>{
        const params = {clave};
        const resultado = await invoke<[FiltrosVerOrdenesProps, Pedido[]] | null>('obtener_pedidos_filtros_compartido', params);
        return resultado;
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
            listTiposPedidosFecha = getListTiposPedidosByDate(pedidos);
            listTotalRecaudadoFecha = getListTotalRecaudadoByDate(pedidos);
            listEstadosPedidosFecha = getListEstadosPedidosByDate(pedidos);
            listTotalRecaudadoPorTipoPorFecha = getListTotalRecaudadoByTipoByDate(pedidos);
            listPlatillosMasVendidosFecha = getListPlatillosMasVendidosByDate(pedidos);
            generalTipoPedidoPie = getGeneralTipoPedidoPie(pedidos);
            totalRecaudadoGeneral = getTotalRecaudadoGeneral(pedidos);
            generalEstadoPedidoPie = getGeneralEstadoPedidoPie(pedidos);
            generalPlatillosPie = getGeneralPlatillosPie(pedidos);
        };
        getData();
        onMostrarVentanaAlMontarse();
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
    <TipoPorDia listTiposPedidosFecha={listTiposPedidosFecha}/>
    <TotalRecaudadoPorDia listTotalRecaudadoFecha={listTotalRecaudadoFecha}/>
    <EstadosPorFecha listEstadosPedidosFecha={listEstadosPedidosFecha}/>
    <TotalRecaudadoPorTipo listTotalRecaudadoPorTipoPorFecha={listTotalRecaudadoPorTipoPorFecha}/>
    <PlatillosMasVendidosPorFecha listPlatillosMasVendidosFecha={listPlatillosMasVendidosFecha}/>

    <h1 class="text-4xl font-bold my-10">Generales</h1>
    <h2 class="text-2xl font-semibold mb-12">Total recaudado: {formatearMoneda(totalRecaudadoGeneral)}</h2>

    <!-- Contenedor principal que maneja la separación -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-16 md:gap-8 items-start w-full">
        <div class="w-full">
            <TiposGeneral generalTipoPedidoPie={generalTipoPedidoPie}/>
        </div>
        
        <div class="w-full">
            <EstadosGeneral generalEstadoPedidoPie={generalEstadoPedidoPie}/>
        </div>
        
        <div class="w-full">
            <PlatillosGenerales generalPlatillosPie={generalPlatillosPie}/>
        </div>
    </div>

</main>

