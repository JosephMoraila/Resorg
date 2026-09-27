<script lang="ts">

    import { fade, fly } from "svelte/transition";
    import { cubicOut } from "svelte/easing";
    import type { Pedido, MetodoPago } from "$lib/types";
    import { sanitizeNonNegativeInput } from "$lib/utils/input_utils";
    import { formatearMoneda, obtenerTextoTipoPedido, obtenerTextoMetodoPago, returnEmptyStringIfNullOrUndefined, obtenerTextoEstadoPedido, formatearFechaDB} from "$lib/utils/string_utils";

    interface Props{
        isAbierto: boolean;
        pedidoPendiente: Pedido;
        onCobrar(metodoPago: MetodoPago, contenidoTicket: string): Promise<void>;
    }

    let {isAbierto = $bindable(), pedidoPendiente, onCobrar}:Props = $props();

    function cancelar(){
        isAbierto = false;
    }

    function manejarTeclado(event: KeyboardEvent) {
        if (isAbierto && event.key === "Escape") cancelar();
        //if (event.key === "Enter") confirmar();
    }

    function autofocus(node: HTMLButtonElement) {
        node.focus();
    }

    function onAceptar(){
        contenidoTicket = `Total: ${formatearMoneda(pedidoPendiente.total)} ${obtenerTextoTipoPedido(pedidoPendiente.tipo)}\nMétodo de pago: ${obtenerTextoMetodoPago(metodoPago)}\n`;
        if(metodoPago == "Efectivo"){
            contenidoTicket += `Pagado: ${formatearMoneda(montoPagadoEfectivo)}\nCambio: ${formatearMoneda(cambioPagadoEfectivo)}\n`;
        }else if(metodoPago == "Mixto"){
            contenidoTicket += `Pagado en efectivo: ${formatearMoneda(mixtoPagadoEfectivo)}\nPagado transferencia/tarjeta: ${formatearMoneda(mixtoPagadoVirtual)}\nambio: ${formatearMoneda(cambioPagadoMixto)}\n`;
        }
        contenidoTicket += `Nombre: ${returnEmptyStringIfNullOrUndefined(pedidoPendiente.nombre_cliente)}\n${formatearFechaDB(pedidoPendiente.fecha_hora)}\n`;
        if(pedidoPendiente.info_tipo_pedido.tipo == "Local"){
            contenidoTicket += `Mesero: ${returnEmptyStringIfNullOrUndefined(pedidoPendiente.info_tipo_pedido.mesero)}`;
        }else if(pedidoPendiente.info_tipo_pedido.tipo == "Domicilio"){
            contenidoTicket += `Colonia: ${returnEmptyStringIfNullOrUndefined(pedidoPendiente.info_tipo_pedido.colonia)}\nCalle: ${returnEmptyStringIfNullOrUndefined(pedidoPendiente.info_tipo_pedido.calle)}\nNúmero interior/exterior: ${returnEmptyStringIfNullOrUndefined(pedidoPendiente.info_tipo_pedido.numero_interior_exterior)}\nTeléfono: ${returnEmptyStringIfNullOrUndefined(pedidoPendiente.info_tipo_pedido.telefono)}\n`;
        }
        for(const pl of pedidoPendiente.platillos_pedidos){
            contenidoTicket += `${pl.name} - ${formatearMoneda(pl.precio)}\n`;
        }
        console.log(contenidoTicket);
        onCobrar(metodoPago, contenidoTicket);
    }

    let contenidoTicket = $state("");

    let metodoPago = $state<MetodoPago>("Efectivo"); 

    let montoPagadoEfectivo = $state(0);
    let cambioPagadoEfectivo = $derived(montoPagadoEfectivo - pedidoPendiente.total);
    function handleMontoEfectivoInput(e: Event) {
        const newMonto = sanitizeNonNegativeInput(e);
        montoPagadoEfectivo = newMonto;
    }
    function onAumentarEfectivoPagado(){
        montoPagadoEfectivo += 1;
    }
    function onDisminuirEfectivoPagado(){
        if(montoPagadoEfectivo > 0){
            montoPagadoEfectivo -= 1;
        }
    }

    let mixtoPagadoEfectivo = $state(0);
    let mixtoPagadoVirtual = $state(0);
    let cambioPagadoMixto = $derived((mixtoPagadoEfectivo + mixtoPagadoVirtual) - pedidoPendiente.total);
    function handleMontoMixtoEfectivoInput(e: Event) {
        const newMonto = sanitizeNonNegativeInput(e);
        mixtoPagadoEfectivo = newMonto;
    }
    function handleMontoMixtoVirtualInput(e: Event) {
        const newMonto = sanitizeNonNegativeInput(e);
        mixtoPagadoVirtual = newMonto;
    }
    function onAumentarMixtoEfectivoPagado(){
        mixtoPagadoEfectivo += 1;
    }
    function onDisminuirMixtoEfectivoPagado(){
        if(mixtoPagadoEfectivo > 0){
            mixtoPagadoEfectivo -= 1;
        }
    }
    function onAumentarMixtoVirtualPagado(){
        mixtoPagadoVirtual += 1;
    }
    function onDisminuirMixtoVirtualPagado(){
        if(mixtoPagadoVirtual > 0){
            mixtoPagadoVirtual -= 1;
        }
    }

</script>

<svelte:window onkeydown={manejarTeclado} />

{#if isAbierto}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div transition:fade={{ duration: 150 }} class="fixed inset-0 bg-black/50 flex items-start justify-center z-50 overflow-y-auto w-screen h-screen py-8">
        <div transition:fly={{ y: -15, duration: 200, easing: cubicOut }} class="bg-white dark:bg-[#1a1f2e] flex flex-col items-center justify-center rounded-lg shadow-xl p-6 w-full max-w-[80%] mx-4">

            <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Pedido ID: {pedidoPendiente.id}</h2>
            <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Total: {formatearMoneda(pedidoPendiente.total)}</h3>
            <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
            <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Tipo: {obtenerTextoTipoPedido(pedidoPendiente.tipo)}</h4>
            <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
            <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Nombre cliente: {returnEmptyStringIfNullOrUndefined(pedidoPendiente.nombre_cliente)}</h4>
            <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
            <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Estado: {obtenerTextoEstadoPedido(pedidoPendiente.estado)}</h4>
            <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
            <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Fecha y hora: {formatearFechaDB(pedidoPendiente.fecha_hora)}</h4>
            <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
            <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Nota: {returnEmptyStringIfNullOrUndefined(pedidoPendiente.nota)}</h4>
            <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>

            {#if pedidoPendiente.info_tipo_pedido.tipo == "Local"}
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Piso: {pedidoPendiente.info_tipo_pedido.piso}</h4>
                <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Mesa: {pedidoPendiente.info_tipo_pedido.mesa}</h4>
                <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Mesero: {pedidoPendiente.info_tipo_pedido.mesero}</h4>
                <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
            {:else if pedidoPendiente.info_tipo_pedido.tipo == "Domicilio"}
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Colonia: {returnEmptyStringIfNullOrUndefined(pedidoPendiente.info_tipo_pedido.colonia)}</h4>
                <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Calle: {returnEmptyStringIfNullOrUndefined(pedidoPendiente.info_tipo_pedido.calle)}</h4>
                <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Número interior/exterior: {returnEmptyStringIfNullOrUndefined(pedidoPendiente.info_tipo_pedido.numero_interior_exterior)}</h4>
                <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Número celular: {returnEmptyStringIfNullOrUndefined(pedidoPendiente.info_tipo_pedido.telefono)}</h4>
                <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Repartidor: {returnEmptyStringIfNullOrUndefined(pedidoPendiente.info_tipo_pedido.repartidor)}</h4>
                <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>
            {/if}

            <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Platillos pedidos:</h4>
            {#each pedidoPendiente.platillos_pedidos as platilloPedido (platilloPedido.id)}
                <p class="italic">{platilloPedido.name} - {formatearMoneda(platilloPedido.precio)}</p>
            {/each}

            <span class="inline-block w-full h-px bg-gray-400 align-middle mx-1"></span>

            <div class="text-center">
                <h1 class="text-lg font-semibold text-gray-900 dark:text-white">Método de pago</h1>
                <label><input type="radio" name="metodoDePago" value="Efectivo" bind:group={metodoPago} />Efectivo</label>
                <label><input type="radio" name="metodoDePago" value="Tarjeta" bind:group={metodoPago} />Tarjeta</label>
                <label><input type="radio" name="metodoDePago" value="Transferencia" bind:group={metodoPago} />Transferencia</label>
                <label><input type="radio" name="metodoDePago" value="Mixto" bind:group={metodoPago} />Mixto</label>
            </div>

            <div class="flex flex-col justify-center align-middle text-center">
                {#if metodoPago == "Efectivo"}
                    <h1 class="text-lg font-semibold text-gray-900 dark:text-white">Pagado en efectivo:</h1>
                        <div class="flex items-center justify-center">
                            <input oninput={handleMontoEfectivoInput}
                            type="number" bind:value={montoPagadoEfectivo}
                            class="border w-20 h-8 rounded-l px-2 disabled:opacity-50 disabled:cursor-not-allowed [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                            />
                            <button class="border h-8 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={onAumentarEfectivoPagado}>+</button>
                            <button class="border h-8 rounded-r cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={onDisminuirEfectivoPagado}>-</button>
                        </div>       
                    <p>Cambio: {formatearMoneda(cambioPagadoEfectivo)}</p>
                {:else if metodoPago == "Mixto"}
                    <h1 class="text-lg font-semibold text-gray-900 dark:text-white">Pagado parte en efectivo:</h1>
                        <div class="flex items-center justify-center">
                            <input oninput={handleMontoMixtoEfectivoInput}
                            type="number" bind:value={mixtoPagadoEfectivo}
                            class="border w-20 h-8 rounded-l px-2 disabled:opacity-50 disabled:cursor-not-allowed [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                            />
                            <button class="border h-8 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={onAumentarMixtoEfectivoPagado}>+</button>
                            <button class="border h-8 rounded-r cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={onDisminuirMixtoEfectivoPagado}>-</button>
                        </div>       
                        <h1 class="text-lg font-semibold text-gray-900 dark:text-white">Pagado parte en tarjeta/transferencia:</h1>
                        <div class="flex items-center justify-center">
                            <input oninput={handleMontoMixtoVirtualInput}
                            type="number" bind:value={mixtoPagadoVirtual}
                            class="border w-20 h-8 rounded-l px-2 disabled:opacity-50 disabled:cursor-not-allowed [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                            />
                            <button class="border h-8 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={onAumentarMixtoVirtualPagado}>+</button>
                            <button class="border h-8 rounded-r cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={onDisminuirMixtoVirtualPagado}>-</button>
                        </div>    
                    <p>Cambio: {formatearMoneda(cambioPagadoMixto)}</p>
                {/if}
            </div>

            <div class="flex flex-row space-x-3.5 self-end mt-5">
                <button use:autofocus onclick={cancelar} class="border p-1 rounded bg-red-400 hover:bg-red-500 cursor-pointer">Cancelar</button>
                <button class="border p-1 rounded cursor-pointer bg-green-400 hover:bg-green-500" onclick={onAceptar}>Aceptar</button>
            </div>

        </div>
   </div>

{/if}
