<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWindow} from "@tauri-apps/api/window";
    import { onMount } from "svelte";
    import type { Pedido } from "$lib/types";
    import { formatearMoneda, returnEmptyStringIfNullOrUndefined, formatearFechaDB } from "$lib/utils/string_utils";
    import { HayAlMenosUnPlatilloExistente, HayAlMenosUnaDescripcionNoNullPlatilloOriginal, GetDescripcionPlatilloOriginal } from "$lib/utils/object_utils";
    import ModalCambiarInfoOrden from "$lib/components/modals/ModalCambiarInfoOrden.svelte";
    
    let pedido = $state<Pedido | null>(null);
    let selectedId: number | null = $state<number | null>(null);

    let isAbrirModal = $state(false);

    async function obtenerPedido(clave: string): Promise<Pedido | null> {
        const params = {clave};
        const resultado = await invoke<Pedido | null>('obtener_pedido_compartido', params);
        return resultado;
    }

    onMount(()=>{
        const ventanaActual = getCurrentWindow();
        const label = ventanaActual.label;

        const getData = async ()=>{
            const res = await obtenerPedido(label);
            pedido = res;
        };
        getData();
    });

    function onAbrirModal(){
        isAbrirModal = true;
    }
    async function onGuardadCambios(pedido: Pedido){
        console.log(pedido);
    }
</script>

<main class="min-h-screen w-full bg-white dark:bg-black text-black dark:text-white flex flex-col items-center overflow-x-hidden">

    {#if pedido}
        <div class="flex flex-row items-center gap-4 flex-nowrap overflow-x-auto pb-2 w-full max-w-full">
            {#if pedido.tipo == "Local"}
                <span><strong>Tipo:</strong> Local</span>
            {:else if pedido.tipo == "Domicilio"}
                <span><strong>Tipo:</strong> Domicilio</span>
            {:else if pedido.tipo == "Recoger"}
                <span><strong>Tipo:</strong> Recoger</span>
            {/if}
            <span>|</span>
            <span><strong>Orden ID:</strong> {pedido.id}</span>
            <span>|</span>
            <span><strong>Nombre cliente:</strong> {returnEmptyStringIfNullOrUndefined(pedido.nombre_cliente)}</span>
            <span>|</span>
            <span><strong>Total:</strong> {formatearMoneda(pedido.total)}</span>
            <span>|</span>
            {#if pedido.estado == "Pendiente"}
                <span><strong>Estado:</strong> Pendiente</span>
            {:else if pedido.estado == "Cancelado"}
                <span><strong>Estado:</strong> Cancelado</span>
            {:else if pedido.estado == "Cobrado"}
                <span><strong>Estado:</strong> Cobrado</span>
            {:else if pedido.estado == "Entregado"}
                <span><strong>Estado:</strong> Entregado</span>
            {:else if pedido.estado == "Finalizado"}
                <span><strong>Estado:</strong> Finalizado</span>
            {/if}
            <span>|</span>
            <span><strong>Nota:</strong> {returnEmptyStringIfNullOrUndefined(pedido.nota)}</span>
            <span>|</span>
            <span><strong>Fecha:</strong> {formatearFechaDB(pedido.fecha_hora)}</span>
            <span>|</span>
            {#if pedido.info_tipo_pedido.tipo == "Local"}
                <span><strong>ID tipo local:</strong> {pedido.info_tipo_pedido.id}</span>
                <span>|</span>
                <span><strong>Mesa:</strong> {pedido.info_tipo_pedido.mesa}</span>
                <span>|</span>
                <span><strong>Piso:</strong> {pedido.info_tipo_pedido.piso}</span>
                <span>|</span>
                <span><strong>Mesero:</strong> {pedido.info_tipo_pedido.mesero}</span>
            {:else if pedido.info_tipo_pedido.tipo == "Domicilio"}
                <span><strong>ID tipo domicilio:</strong> {pedido.info_tipo_pedido.id}</span>
                <span>|</span>
                <span><strong>Colonia:</strong> {returnEmptyStringIfNullOrUndefined(pedido.info_tipo_pedido.colonia)}</span>
                <span>|</span>
                <span><strong>Calle:</strong> {returnEmptyStringIfNullOrUndefined(pedido.info_tipo_pedido.calle)}</span>
                <span>|</span>
                <span><strong>Número interior/exterior:</strong> {returnEmptyStringIfNullOrUndefined(pedido.info_tipo_pedido.numero_interior_exterior)}</span>
                <span>|</span>
                <span><strong>Teléfono:</strong> {returnEmptyStringIfNullOrUndefined(pedido.info_tipo_pedido.telefono)}</span>
                <span>|</span>
                <span><strong>Repartidor:</strong> {returnEmptyStringIfNullOrUndefined(pedido.info_tipo_pedido.repartidor)}</span>
            {:else if pedido.info_tipo_pedido.tipo == "Recoger"}
                <span><strong>ID tipo recoger:</strong> {pedido.info_tipo_pedido.id}</span>
            {/if}
        </div>

        <div class="flex flex-row items-center gap-4 flex-nowrap overflow-x-auto pb-2 w-full max-w-full">
            <button onclick={onAbrirModal} class="btn-realista items-start! justify-start!">Cambiar información</button>
        </div>

        <div class="w-full mt-4">
            <table class="tabla-estilizada">
                <thead class="tabla-head">
                    <tr>
                        <th class="tabla-th">ID platillo pedido</th>
                        <th class="tabla-th">Nombre platillo</th>
                        <th class="tabla-th">Precio</th>
                        {#if HayAlMenosUnPlatilloExistente(pedido.platillos_pedidos) && HayAlMenosUnaDescripcionNoNullPlatilloOriginal(pedido.platillos_pedidos)}
                            <th class="tabla-th">Descripcion </th>
                        {/if}
                    </tr>
                </thead>
                <tbody class="tabla-body">
                    {#each pedido.platillos_pedidos as platilloPedido (platilloPedido.id)}
                        <tr class="tabla-tr cursor-pointer {selectedId === platilloPedido.id ? 'dark:bg-blue-900 dark:text-white bg-blue-400' : 'hover:bg-gray-100 dark:hover:bg-gray-700'}"  onclick={()=>selectedId = platilloPedido.id}>
                            <td class="tabla-td-primary">{platilloPedido.id}</td>
                            <td class="tabla-td-primary">{platilloPedido.name}</td>
                            <td class="tabla-td-primary">{formatearMoneda(platilloPedido.precio)}</td>
                            {#if HayAlMenosUnPlatilloExistente(pedido.platillos_pedidos) && HayAlMenosUnaDescripcionNoNullPlatilloOriginal(pedido.platillos_pedidos)}
                                <td class="tabla-td-primary">{GetDescripcionPlatilloOriginal(platilloPedido)}</td>
                            {/if}
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>

        {#if isAbrirModal}
            <ModalCambiarInfoOrden bind:isAbierto={isAbrirModal} pedido={pedido} onGuardadCambios={onGuardadCambios}/>
        {/if}
        

    {/if}

</main>

