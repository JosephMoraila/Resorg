<script lang="ts">
    import Filtros from "./Filtros.svelte";
    import BotonesVerMas from "./BotonesVerMas.svelte";
    import type { Pedido, FiltrosVerOrdenesProps, EstadoPedido } from "$lib/types";
    import { returnEmptyStringIfNullOrUndefined, formatearFechaDB, obtenerTextoTipoPedido, formatearMoneda, obtenerTextoEstadoPedido } from "$lib/utils/string_utils";
    import { invoke } from "@tauri-apps/api/core";
    import { calcularTotalPaginas } from "$lib/utils/math_utils";
    import { toast } from "$lib/toast.svelte";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import { getCurrentWebviewWindow, WebviewWindow } from "@tauri-apps/api/webviewWindow";

    let paginaActual = $state(1);  
    let pedidos: Pedido[] = $state([]);
    let filters: FiltrosVerOrdenesProps = $state({id: null, estatus: null, fechaFin: null, fechaInicio: null, nombreCliente: null, nota: null, tipoPedido: null, totalDesde: null, totalHasta: null, metodoPago: null});
    let paginasTotales = $state(1);
    let totalPedidos = $state(0);
    const PAGINA_TAMANO = 50;

    let selectedId: number | null = $state<number | null>(null);

    /* Llama a la función de Rust para tarer los elementos de DB
    * @param pagina Página (Si no se le pasa parametro se usará paginaActual como defecto y al montarse es 1)
    * @param filtros Filtros (Si no se le pasa parametro se usará filters como defecto y al montarse es todo null)
    */
    async function obtenerPedidos(pagina: number = paginaActual, filtros: FiltrosVerOrdenesProps = filters) {
        try{
            const params = { paginaFrontend: pagina, ...filtros };
            const response = await invoke<Pedido[]>("obtener_ordenes", params);
            console.log(response);
            pedidos = response;
        } catch (error) {
            const err = error as string;
            console.error("Error al obtener los pedidos:", err);
            toast.rojo(`Error al obtener los pedidos: ${err}`);
        }
    }

    async function obtenerCountPedidos(filtros: FiltrosVerOrdenesProps = filters) {
        try {
            const params = {...filtros};
            const totalPedidosBackend = await invoke<number>("count_ordenes", params);
            totalPedidos = totalPedidosBackend;
            paginasTotales = calcularTotalPaginas(totalPedidos, PAGINA_TAMANO);
        } catch (error) {
            const err = error as string;
            console.error("Error al obtener el conteo de pedidos pendientes:", err);
            toast.rojo(`Error al obtener el conteo de pedidos pendientes: ${err}`);
        }
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
        obtenerPedidos();
        obtenerCountPedidos();
        onMostrarVentanaAlMontarse();

        const unlisten = listen("pedido-creado", () => {
            obtenerPedidos();
            obtenerCountPedidos();
        });

        return () => {
            unlisten.then((fna) => fna());
        };
    });

    function irPaginaAnterior() {
        if (paginaActual > 1) {
            paginaActual--;
            obtenerPedidos(paginaActual, filters);
        }
    }

    function irPaginaSiguiente() {
        if (paginaActual < paginasTotales) {
            paginaActual++;
            obtenerPedidos(paginaActual, filters);
        }
    }

    function getColorEstado(estado: EstadoPedido){
        if(estado == "Pendiente") return "tabla-badge";
        else if(estado == "Cancelado") return "tabla-badged-cancelado";
        else if(estado == "Cobrado") return "tabla-badged-cobrado";
        else if(estado == "Entregado") return "tabla-badged-entregado";
        else return "tabla-badged-finalizado";
    }

</script>

<main class="min-h-screen w-full bg-white dark:bg-black text-black dark:text-white flex flex-col items-center overflow-x-hidden">

    <Filtros bind:selectedId={selectedId} bind:pedidos={pedidos} bind:filters={filters} bind:totalPedidos={totalPedidos} bind:paginasTotales={paginasTotales}/>
    <BotonesVerMas pedidos={pedidos} selectedId={selectedId} totalPedidos={totalPedidos} filters={filters}/>

    <div class="w-full overflow-x-auto border-y border-gray-200 dark:border-gray-800 shadow-sm">
        <table class="tabla-estilizada">
            <thead class="tabla-head">
                <tr>
                    <th class="tabla-th">ID</th>
                    <th class="tabla-th">Total</th>
                    <th class="tabla-th">Tipo</th>
                    <th class="tabla-th">Estado</th>
                    <th class="tabla-th">Nombre cliente</th>
                    <th class="tabla-th w-72">Notas</th>
                    <th class="tabla-th">Fecha hora registro</th>
                </tr>
            </thead>
            <tbody class="tabla-body">
                {#each pedidos as pedido (pedido.id)}
                    <tr class="tabla-tr cursor-pointer {selectedId === pedido.id ? 'dark:bg-blue-900 dark:text-white bg-blue-400' : 'hover:bg-gray-100 dark:hover:bg-gray-700'}" onclick={() => selectedId = pedido.id}>
                        <td class="tabla-td-primary">{pedido.id}</td>
                        <td class="tabla-td text-gray-800 dark:text-gray-200">{formatearMoneda(pedido.total)}</td>
                        <td class="tabla-td">{obtenerTextoTipoPedido(pedido.tipo)}</td>
                        <td class="py-3 px-4">
                            <span class={getColorEstado(pedido.estado)}>{obtenerTextoEstadoPedido(pedido.estado)}</span>
                        </td>
                        <td class="tabla-td">{returnEmptyStringIfNullOrUndefined(pedido.nombre_cliente)}</td>
                        <td class="tabla-td w-72 italic truncate">{returnEmptyStringIfNullOrUndefined(pedido.nota)}</td>
                        <td class="tabla-td whitespace-nowrap">{formatearFechaDB(pedido.fecha_hora)}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>

    <div class="flex flex-row justify-center items-center mt-4 space-x-3.5">
        
        <button class="btn-realista disabled:opacity-50 disabled:cursor-not-allowed!" onclick={irPaginaAnterior} disabled={paginaActual === 1}>Anterior</button>
        <span>{paginaActual} de {paginasTotales}</span>
        <button class="btn-realista disabled:opacity-50 disabled:cursor-not-allowed!" onclick={irPaginaSiguiente} disabled={paginaActual === paginasTotales}>Ver más</button>

    </div>

</main>