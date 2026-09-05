<script lang="ts">

    import type { EstadoPedido, Pedido, FiltrosVerOrdenesProps } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { toast } from "$lib/toast.svelte";
    import { formatearMoneda, returnEmptyStringIfNullOrUndefined, formatearFechaDB } from "$lib/utils/string_utils";
    import { onMount } from "svelte";
    import { calcularTotalPaginas } from "$lib/utils/math_utils";
    import { listen } from "@tauri-apps/api/event";
    import Filtros from "./Filtros.svelte";


    let paginaActual = $state(1);  
    let filters: FiltrosVerOrdenesProps = $state({id: null, estatus: "Pendiente", fechaFin: null, fechaInicio: null, nombreCliente: null, nota: null, tipoPedido: null, totalDesde: null, totalHasta: null});
    let paginasTotales = $state(1);
    let totalPedidosPendientes = $state(0);
    const PAGINA_TAMANO = 50;

    let pedidosPendientes: Pedido[] = $state([]);

    let selectedId: number | null = $state<number | null>(null);
 
    /**
     * Llama a la función de Rust para tarer los elementos de DB
     * @param pagina Página (Si no se le pasa parametro se usará paginaActual como defecto y al montarse es 1)
     * @param filtros Filtros (Si no se le pasa parametro se usará filters como defecto y al montarse es todo null)
     */
    async function obtenerPedidosPendientes(pagina: number = paginaActual, filtros: FiltrosVerOrdenesProps = filters) {
        try{
            const params = { paginaFrontend: pagina, ...filtros };
            const response = await invoke<Pedido[]>("obtener_ordenes", params);
            pedidosPendientes = response;
        } catch (error) {
            const err = error as string;
            console.error("Error al obtener los pedidos pendientes:", err);
            toast.rojo(`Error al obtener los pedidos pendientes: ${err}`);
        }
    }

    async function obtenerCountPedidosPendientes(filtros: FiltrosVerOrdenesProps = filters) {
        try {
            const params = {...filtros};
            const totalPedidos = await invoke<number>("count_ordenes", params);
            totalPedidosPendientes = totalPedidos;
            paginasTotales = calcularTotalPaginas(totalPedidosPendientes, PAGINA_TAMANO);
        } catch (error) {
            const err = error as string;
            console.error("Error al obtener el conteo de pedidos pendientes:", err);
            toast.rojo(`Error al obtener el conteo de pedidos pendientes: ${err}`);
        }
    }

    onMount(() => {
        obtenerPedidosPendientes();
        obtenerCountPedidosPendientes();

        const unlisten = listen("pedido-creado", () => {
            obtenerPedidosPendientes();
            obtenerCountPedidosPendientes();
        });

        return () => {
            unlisten.then((fna) => fna());
        };
    });

    function irPaginaAnterior() {
        if (paginaActual > 1) {
            paginaActual--;
            obtenerPedidosPendientes(paginaActual, filters);
        }
    }

    function irPaginaSiguiente() {
        if (paginaActual < paginasTotales) {
            paginaActual++;
            obtenerPedidosPendientes(paginaActual, filters);
        }
    }

</script>

<main class="min-h-screen w-full bg-white dark:bg-black text-black dark:text-white flex flex-col items-center overflow-x-hidden">

    <Filtros bind:pedidosPendientes={pedidosPendientes} bind:filters={filters} bind:totalPedidosPendientes={totalPedidosPendientes} bind:paginasTotales={paginasTotales}/>

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
                {#each pedidosPendientes as pedido (pedido.id)}
                    <tr class="tabla-tr cursor-pointer {selectedId === pedido.id ? 'dark:bg-blue-900 dark:text-white bg-blue-400' : 'hover:bg-gray-100 dark:hover:bg-gray-700'}" onclick={() => selectedId = pedido.id}>
                        <td class="tabla-td-primary">{pedido.id}</td>
                        <td class="tabla-td text-gray-800 dark:text-gray-200">{formatearMoneda(pedido.total)}</td>
                        <td class="tabla-td">{pedido.tipo}</td>
                        <td class="py-3 px-4">
                            <span class="tabla-badge">{pedido.estado}</span>
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
        
        <button class="btn-realista disabled:opacity-50 disabled:cursor-not-allowed" onclick={irPaginaAnterior} disabled={paginaActual === 1}>Anterior</button>
        <span>{paginaActual} de {paginasTotales}</span>
        <button class="btn-realista disabled:opacity-50 disabled:cursor-not-allowed" onclick={irPaginaSiguiente} disabled={paginaActual === paginasTotales}>Ver más</button>

    </div>

</main>