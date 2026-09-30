<script lang="ts">

    import type { TipoPedido, Pedido, FiltrosVerOrdenesProps, EstadoPedido, MetodoPago } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { formatearFechaUTC, obtenerRangoDeHoy, procesarCadenaHora } from "$lib/utils/date_utils";
    import { parsearNumeroOpcional, calcularTotalPaginas } from "$lib/utils/math_utils";
    import { toast } from "$lib/toast.svelte";
    import { flatpickrAction } from "$lib/actions/flatpickr";
    import 'flatpickr/dist/flatpickr.min.css';
    import '$lib/styles/flatpickr-dark.css';

    interface Props{
        pedidos: Pedido[];
        filters: FiltrosVerOrdenesProps;
        totalPedidos: number;
        paginasTotales: number;
        selectedId: number | null;
    }

    const PAGINA_TAMANO = 50;

    let {pedidos = $bindable(), selectedId = $bindable(),filters = $bindable(), totalPedidos = $bindable(), paginasTotales = $bindable()}:Props = $props();

    let idInput: number | null = $state<number | null>(null);
    let tipoInput: TipoPedido | "" = $state<TipoPedido | "">("");
    let estadoInput: EstadoPedido | "" = $state<EstadoPedido | "">("");
    let metodoPagoInput: MetodoPago | "" = $state<MetodoPago | "">("");
    let nombreClienteInput = $state<string>("");
    const {inicio, fin} = obtenerRangoDeHoy();
    let dateInicio = $state<string>(formatearFechaUTC(inicio));
    let dateFin = $state<string>(formatearFechaUTC(fin));
    let totalDesde = $state("");
    let totalHasta = $state("");
    let nota = $state("");

    function incrementarId() {
        if (idInput !== null) {
            idInput++;
        }else{
            idInput = 1;
        }
    }

    function decrementarId() {
        if (idInput !== null && idInput > 0) {
            idInput--;
        }else{
            idInput = 1;
        }
    }

    async function buscarFiltros() {
        let definitiveIdInput: null | number = null;
        if(typeof idInput === "number"){
            if(idInput <= 0) definitiveIdInput = null;
            else definitiveIdInput = idInput;
        }

        let definitiveTipoInput: null | TipoPedido = null;
        if(tipoInput !== ""){ //Si no esta vacio queremos buscar por un tipo en especifico
            definitiveTipoInput = tipoInput;
        }

        let definitiveEstadoInput: null | EstadoPedido = null;
        if(estadoInput !== ""){
            definitiveEstadoInput = estadoInput;
        }

        let definitiveMetodoPago: null | MetodoPago = null;
        if(estadoInput == "Cobrado"){
            if(metodoPagoInput !== ""){
                definitiveMetodoPago = metodoPagoInput;
            }
        }
        console.log(`Estado: ${estadoInput}. DefinitiveMetodoPago: ${definitiveMetodoPago}`);

        let definitiveNombreCliente: string | null = null;
        if(nombreClienteInput.trim() != "") definitiveNombreCliente = nombreClienteInput.trim();

        let definitiveFechaInicio: string | null = null;
        if(dateInicio != "") definitiveFechaInicio = procesarCadenaHora(dateInicio);
        let definitiveFechaFin: string | null = null;
        if(dateFin != "") definitiveFechaFin = procesarCadenaHora(dateFin);

        const definitiveTotalDesde: number | null = parsearNumeroOpcional(totalDesde);
        const definitiveTotalHasta: number | null = parsearNumeroOpcional(totalHasta);

        let definitiveNota: string | null = null;
        if(nota.trim() != "") definitiveNota = nota.trim();

        const paramsFiltro: FiltrosVerOrdenesProps = {id: definitiveIdInput, tipoPedido: definitiveTipoInput, nombreCliente: definitiveNombreCliente, fechaInicio: definitiveFechaInicio, fechaFin: definitiveFechaFin, totalDesde: definitiveTotalDesde, totalHasta: definitiveTotalHasta, nota: definitiveNota, estatus: definitiveEstadoInput, metodoPago: definitiveMetodoPago};
        const paramsFunc = {paginaFrontend: 1, ...paramsFiltro}; //Como es buscar nuevos filtros se empieza del 1
        const paramsCount = {...paramsFiltro};
        try{
            const dataBckend = await invoke<Pedido[]>("obtener_ordenes", paramsFunc);
            pedidos = dataBckend;
            filters = paramsFiltro;
            selectedId = null;
            const resultadosTotalesBackend = await invoke<number>("count_ordenes", paramsCount);
            totalPedidos = resultadosTotalesBackend;
            paginasTotales = calcularTotalPaginas(totalPedidos, PAGINA_TAMANO);
        }catch (error) {
            const err = error as string;
            console.error("Error al obtener los pedidos:", err);
            toast.rojo(`Error al obtener los pedidos: ${err}`);
        }
    }

    function onEnterDown(event: KeyboardEvent){
        if(event.key === "Enter"){
            buscarFiltros();
        }
    }

</script>

<div class="flex flex-row items-center gap-4 flex-nowrap overflow-x-auto pb-2 w-full max-w-full">
    <div>
        <button onclick={buscarFiltros} class="btn-realista py-1!">Buscar</button>
    </div>
    <div class="flex flex-row items-center gap-0 shrink-0 whitespace-nowrap">
        <span class="mr-2">ID:</span>
        <input type="number" onkeydown={onEnterDown} bind:value={idInput} title="Buscar por ID anula los otros filtros" class="border w-20 rounded-l px-2 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none">
        <button onclick={incrementarId} class="border cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 px-2 ">
            +
        </button>
        <button onclick={decrementarId} class="border rounded-r cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 px-2">
            -
        </button>
    </div>
    <div class="flex flex-row items-center gap-2 shrink-0 whitespace-nowrap">
        <span>Tipo:</span>
        <select bind:value={tipoInput} class="form-select border rounded px-2 py-1 bg-white dark:bg-[#1a1f2e] text-gray-900 dark:text-white border-gray-300 dark:border-white/20">
            <option value="">Todos</option>
            <option value="Domicilio">Domicilio</option>
            <option value="Local">Local</option>
            <option value="Recoger">Recoger</option>
        </select>
    </div>
    <div class="flex flex-row items-center gap-2 shrink-0 whitespace-nowrap">
        <span>Estado:</span>
        <select bind:value={estadoInput} class="form-select border rounded px-2 py-1 bg-white dark:bg-[#1a1f2e] text-gray-900 dark:text-white border-gray-300 dark:border-white/20">
            <option value="">Todos</option>
            <option value="Pendiente">Pendiente</option>
            <option value="Finalizado">Finalizado</option>
            <option value="Cancelado">Cancelado</option>
            <option value="Entregado">Entregado</option>
            <option value="Cobrado">Cobrado</option>
        </select>
    </div>
    {#if estadoInput == "Cobrado"}
    <div class="flex flex-row items-center gap-2 shrink-0 whitespace-nowrap">
        <span>Método de pago:</span>
        <select bind:value={metodoPagoInput} class="form-select border rounded px-2 py-1 bg-white dark:bg-[#1a1f2e] text-gray-900 dark:text-white border-gray-300 dark:border-white/20">
            <option value="">Todos</option>
            <option value="Efectivo">Efectivo</option>
            <option value="Tarjeta">Tarjeta</option>
            <option value="Transferencia">Transferencia</option>
            <option value="Mixto">Mixto</option>
        </select>
    </div>    
    {/if}
    <div class="flex flex-row items-center gap-2 shrink-0 whitespace-nowrap">
        <span>Nombre cliente:</span>
        <input type="text" onkeydown={onEnterDown} bind:value={nombreClienteInput} class="border w-40 rounded px-2 shrink-0">
    </div>
    <div class="flex flex-row items-center gap-2 shrink-0 whitespace-nowrap">
        <span>Desde:</span>
        <input use:flatpickrAction={{ onChange: (dateStr) => dateInicio = dateStr, defaultDate: dateInicio }} type="text" class="border rounded px-2 shrink-0 w-40" />
        <span>Hasta:</span>
        <input use:flatpickrAction={{ onChange: (dateStr) => dateFin = dateStr, defaultDate:dateFin }} type="text" class="border rounded px-2 shrink-0 w-40" />
    </div>
    <div class="flex flex-row items-center gap-0 shrink-0 whitespace-nowrap">
        <span>Total desde:</span>
        <input type="number" onkeydown={onEnterDown} bind:value={totalDesde} class="border w-20 rounded px-2 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none">
        <span>Total hasta:</span>
        <input type="number" onkeydown={onEnterDown} bind:value={totalHasta} class="border w-20 rounded px-2 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none">
    </div>
    <div class="flex flex-row items-center gap-0 shrink-0 whitespace-nowrap">
        <span>Nota:</span>
        <input type="text" onkeydown={onEnterDown} bind:value={nota} class="border w-40 rounded px-2 shrink-0">
    </div>
</div>