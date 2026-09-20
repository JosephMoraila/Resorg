<script lang="ts">

    import { fade, fly } from "svelte/transition";
    import { cubicOut } from "svelte/easing";
    import type { Pedido } from "$lib/types";
    import { formatearMoneda, obtenerTextoTipoPedido, returnEmptyStringIfNullOrUndefined, obtenerTextoEstadoPedido, formatearFechaDB} from "$lib/utils/string_utils";

    interface Props{
        isAbierto: boolean;
        pedidoPendiente: Pedido;
        onCobrar(): Promise<void>;
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
        onCobrar();
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

            <div class="flex flex-row space-x-3.5 self-end mt-5">
                <button use:autofocus onclick={cancelar} class="border p-1 rounded bg-red-400 hover:bg-red-500 cursor-pointer">Cancelar</button>
                <button class="border p-1 rounded cursor-pointer bg-green-400 hover:bg-green-500" onclick={onAceptar}>Aceptar</button>
            </div>

        </div>
   </div>

{/if}
