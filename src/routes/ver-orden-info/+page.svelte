<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWindow} from "@tauri-apps/api/window";
    import { onMount } from "svelte";
    import type { Pedido, PedidoPlatillo, UpdatePlatilloPedido } from "$lib/types";
    import { formatearMoneda, returnEmptyStringIfNullOrUndefined, formatearFechaDB, obtenerTextoTipoPedido, obtenerTextoEstadoPedido } from "$lib/utils/string_utils";
    import { HayAlMenosUnPlatilloExistente, HayAlMenosUnaDescripcionNoNullPlatilloOriginal, GetDescripcionPlatilloOriginal } from "$lib/utils/object_utils";
    import ModalCambiarInfoOrden from "$lib/components/modals/ModalCambiarInfoOrden.svelte";
    import { toast } from "$lib/toast.svelte";
    import { emit } from "@tauri-apps/api/event";
    import ModalCambiarPlatillosOrden from "$lib/components/modals/ModalCambiarPlatillosOrden.svelte";
    
    let pedido = $state<Pedido | null>(null);
    let selectedId: number | null = $state<number | null>(null);

    let isAbrirModalChangeInfo = $state(false);
    let isAbrirModalChangePlatillos = $state(false);

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

    function onAbrirModalChangeInfo(){
        isAbrirModalChangeInfo = true;
    }
    async function onGuardadCambiosChangeInfo(pedidoParam: Pedido){
        if(!pedido) return;
        const params = {pedido: pedidoParam};
        //Si bien pedido ya contiene la información actualizada, en el caso que se cambie su tipo de orden no contiene realmente el nuevo ID de registro de su nueva tabla de tipo y por eso ese invoke lo retorna ya con ese ID en caso que se haya cambiado su tipo
        try{
            const newInfoPedido = await invoke<Pedido>("update_pedido", params);
            Object.assign(pedido, structuredClone(newInfoPedido));
            isAbrirModalChangeInfo = false;
            //Guardamos en Rust su nuevo valor por si se actualiza no se vuelva a cargar con la información anterior
            const value = `ver-orden-info-${newInfoPedido.id}`;
            const paramsRam = {clave:value, valor: newInfoPedido};
            await invoke("guardar_pedido_compartido", paramsRam);
            toast.verde(`Pedido actualizado correctamente`);
            await emit("pedido-creado");//Si esta la ventana de ver ordenes pendientes esta abierta actualizar
        }catch(err){
            const error = err as string;
            toast.rojo(`Error al actualizar pedido: ${error}`);
        }
    }

    function onAbrirModalChangePlatillos(){
        isAbrirModalChangePlatillos = true;
    }
    async function onGuardarCambiosPlatillos(platillosPedidos:UpdatePlatilloPedido[]) {
        if(!pedido) return;
        isAbrirModalChangePlatillos = false;
        const pedidoId = platillosPedidos[0].pedido_id;
        const params = {platillosActualizadoParam: platillosPedidos, pedidoId};
        try{
            const platillosActualizados = await invoke<PedidoPlatillo[]>("update_platillos_orden", params);
            pedido.platillos_pedidos = platillosActualizados;
            //Lo que viene de backend no contiene el total actuializado asi que lo hacemos aqui
            let nuevoTotal = 0;
            for(const pp of platillosActualizados){
                nuevoTotal += pp.precio;
            }
            pedido.total = nuevoTotal;
            const value = `ver-orden-info-${pedido.id}`;
            const paramsRam = {clave:value, valor: pedido};
            await invoke("guardar_pedido_compartido", paramsRam);
            toast.verde(`Platillos actualizados`);
            await emit("pedido-creado");//Si esta la ventana de ver ordenes pendientes esta abierta actualizar
        }catch(err){
            const error = err as string;
            toast.rojo(`Error al actualizar platillos: ${error}`);
        }
    }

    function imprimirOrden(){
        if(pedido === null) return;
        let param = {pedido};
        try{
            invoke("print_again_order_escpos", param);
        }catch(err){
            const error = err as string;
            toast.rojo(`Error al imprimir orden: ${error}`);
        }
    }
</script>

<main class="min-h-screen w-full bg-white dark:bg-black text-black dark:text-white flex flex-col items-center overflow-x-hidden">

    {#if pedido}
        <div class="flex flex-row items-center gap-4 flex-nowrap overflow-x-auto pb-2 w-full max-w-full">
            {#if pedido.tipo == "Local"}
                <span><strong>Tipo:</strong> {obtenerTextoTipoPedido(pedido.tipo)}</span>
            {:else if pedido.tipo == "Domicilio"}
                <span><strong>Tipo:</strong> {obtenerTextoTipoPedido(pedido.tipo)}</span>
            {:else if pedido.tipo == "Recoger"}
                <span><strong>Tipo:</strong> {obtenerTextoTipoPedido(pedido.tipo)}</span>
            {/if}
            <span>|</span>
            <span><strong>Orden ID:</strong> {pedido.id}</span>
            <span>|</span>
            <span><strong>Nombre cliente:</strong> {returnEmptyStringIfNullOrUndefined(pedido.nombre_cliente)}</span>
            <span>|</span>
            <span><strong>Total:</strong> {formatearMoneda(pedido.total)}</span>
            <span>|</span>
            {#if pedido.estado == "Pendiente"}
                <span><strong>Estado:</strong> {obtenerTextoEstadoPedido(pedido.estado)}</span>
            {:else if pedido.estado == "Cancelado"}
                <span><strong>Estado:</strong> {obtenerTextoEstadoPedido(pedido.estado)}</span>
            {:else if pedido.estado == "Cobrado"}
                <span><strong>Estado:</strong> {obtenerTextoEstadoPedido(pedido.estado)}</span>
            {:else if pedido.estado == "Entregado"}
                <span><strong>Estado:</strong> {obtenerTextoEstadoPedido(pedido.estado)}</span>
            {:else if pedido.estado == "Finalizado"}
                <span><strong>Estado:</strong> {obtenerTextoEstadoPedido(pedido.estado)}</span>
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
            <button onclick={onAbrirModalChangeInfo} class="btn-realista items-start! justify-start!">Cambiar información</button>
            <button onclick={onAbrirModalChangePlatillos} class="btn-realista items-start! justify-start!">Cambiar platillos</button>
            <button onclick={imprimirOrden} class="btn-realista items-start! justify-start!">Imprimir orden</button>
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

        {#if isAbrirModalChangeInfo}
            <ModalCambiarInfoOrden bind:isAbierto={isAbrirModalChangeInfo} pedido={pedido} onGuardadCambios={onGuardadCambiosChangeInfo}/>
        {/if}
        
        {#if isAbrirModalChangePlatillos}
            <ModalCambiarPlatillosOrden bind:isAbierto={isAbrirModalChangePlatillos} platillosPedidos={pedido.platillos_pedidos} onGuardadCambios={onGuardarCambiosPlatillos}/>
        {/if}

    {/if}

</main>

