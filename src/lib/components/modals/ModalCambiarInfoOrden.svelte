<script lang="ts">

    import { fade, fly } from "svelte/transition";
    import { cubicOut } from "svelte/easing";
    import type { Pedido, EstadoPedido, TipoPedido, Mesero,PedidoLocal, PedidoDomicilio, PedidoRecoger } from "$lib/types";
    import { onMount } from "svelte";
    import { returnEmptyStringIfNullOrUndefined, returnNumberOrNullAsString, sanitizarInputTelefono, returnNullOrStringValue } from "$lib/utils/string_utils";
    import { invoke } from "@tauri-apps/api/core";
    import { toast } from "$lib/toast.svelte";
    import { untrack } from 'svelte';
    import { GetOriginalIdTipoIfSameOrZero } from "$lib/utils/object_utils";
    import { parsearStringToInt } from "$lib/utils/math_utils";

    interface Props{
        isAbierto: boolean;
        pedido: Pedido;
        onGuardadCambios(pedido: Pedido): Promise<void>;
    }

    let {
        isAbierto = $bindable(), pedido, onGuardadCambios,
    }: Props = $props();

    const originalPedido: Pedido = untrack(() => $state.snapshot(pedido));

    let nombreCliente = $state("");
    let notaCliente = $state("");
    let estadoPedido: EstadoPedido = $state("Pendiente");
    let tipoPedido: TipoPedido = $state("Local");

    let piso = $state(0);
    let mesa = $state(0);
    let meseroSeleccionado = $state("");
    function incrementarPiso() {
        piso++;
    }
    function decrementarPiso() {
        piso--;
    }
    function incrementarMesa() {
        mesa++;
    }
    function decrementarMesa() {
        if (mesa > 0) mesa--;
    }
    let meseros: Mesero[] = $state([]);
    function alConfirmarMesero() {
        const valido = meseros.some((m) => m.nombre === meseroSeleccionado);
        if (!valido) meseroSeleccionado = "";
    }

    let colonia = $state("");
    let calle = $state("");
    let numeroExteriorInterior = $state("");
    let telefono = $state("");
    let repartidor = $state("");

    onMount(()=>{
        nombreCliente = returnEmptyStringIfNullOrUndefined(pedido.nombre_cliente);
        notaCliente = returnEmptyStringIfNullOrUndefined(pedido.nota);
        estadoPedido = pedido.estado;
        tipoPedido = pedido.tipo;

        if(pedido.info_tipo_pedido.tipo == "Local"){
            piso = pedido.info_tipo_pedido.piso;
            mesa = pedido.info_tipo_pedido.mesa;
            meseroSeleccionado = pedido.info_tipo_pedido.mesero;
        }else if(pedido.info_tipo_pedido.tipo == "Domicilio"){
            colonia = returnEmptyStringIfNullOrUndefined(pedido.info_tipo_pedido.colonia);
            calle = returnEmptyStringIfNullOrUndefined(pedido.info_tipo_pedido.calle);
            numeroExteriorInterior = returnNumberOrNullAsString(pedido.info_tipo_pedido.numero_interior_exterior);
            telefono = returnEmptyStringIfNullOrUndefined(pedido.info_tipo_pedido.telefono);
            repartidor = returnEmptyStringIfNullOrUndefined(pedido.info_tipo_pedido.repartidor);
        }

        const getMesero = async()=>{
            try{
                const meserosBackend = await invoke<Mesero[]>("get_meseros");
                meseros = meserosBackend;
            }catch(err){
                const error = err as string;
                toast.rojo(`Error al conseguir meseros: ${error}`);
            }
        };
        getMesero();
    });

    function cancelar(){
        isAbierto = false;
        nombreCliente = returnEmptyStringIfNullOrUndefined(originalPedido.nombre_cliente);
        notaCliente = returnEmptyStringIfNullOrUndefined(originalPedido.nota);
        estadoPedido = originalPedido.estado;
        tipoPedido = originalPedido.tipo;
        if(originalPedido.info_tipo_pedido.tipo == "Local"){
            piso = originalPedido.info_tipo_pedido.piso;
            mesa = originalPedido.info_tipo_pedido.mesa;
            meseroSeleccionado = originalPedido.info_tipo_pedido.mesero;
        }
        else if(originalPedido.info_tipo_pedido.tipo == "Domicilio"){
            colonia = returnEmptyStringIfNullOrUndefined(originalPedido.info_tipo_pedido.colonia);
            calle = returnEmptyStringIfNullOrUndefined(originalPedido.info_tipo_pedido.calle);
            numeroExteriorInterior = returnEmptyStringIfNullOrUndefined(originalPedido.info_tipo_pedido.numero_interior_exterior);
            telefono = returnEmptyStringIfNullOrUndefined(originalPedido.info_tipo_pedido.telefono);
            repartidor = returnEmptyStringIfNullOrUndefined(originalPedido.info_tipo_pedido.repartidor);
        }
    }

    function manejarTeclado(event: KeyboardEvent) {
        if (event.key === "Escape") cancelar();
        //if (event.key === "Enter") confirmar();
    }

    function autofocus(node: HTMLInputElement) {
        node.focus();
    }

    async function onGuardar(){
        const nuevoNombreCliente = returnNullOrStringValue(nombreCliente);
        const nuevoNotaCliente = returnNullOrStringValue(notaCliente);
        const nuevoEstado: EstadoPedido = estadoPedido;
        const nuevoTipo: TipoPedido = tipoPedido;
        const fechaHora = originalPedido.fecha_hora;
        const total = originalPedido.total;
        const id = originalPedido.id;
        const platillosPedidosCopy = structuredClone(originalPedido.platillos_pedidos);
        const originalPedidoId = originalPedido.info_tipo_pedido.pedido_id;
        const sameOrZeroId = GetOriginalIdTipoIfSameOrZero(originalPedido, nuevoTipo);
        if(nuevoTipo == "Local"){
            if(mesa == null || mesa <= 0){
                toast.amarillo(`La mesa no es válida`);
                return;
            }
            if(!meseroSeleccionado){
                toast.amarillo(`Selecciona un mesero`);
                return;  
            }
            const pisoObj = {piso};
            const mesObj = {mesa};
            const params = {...mesObj, ...pisoObj};
            try{
                const isPisoExiste = await invoke<boolean>("is_piso_exists", pisoObj);
                if(!isPisoExiste){
                    toast.amarillo(`El piso ${piso} no está registrado`);
                    return;
                }
                const isMesaExiste = await invoke<boolean>("is_mesa_exists", mesObj);
                if(!isMesaExiste){
                    toast.amarillo(`La mesa ${mesa} no está registrada`);
                    return;
                }
                const isOcupada = await invoke<boolean>("is_mesa_ocupada", params);
                if(isOcupada){
                    toast.amarillo(`Esa mesa ya está ocupada`);
                    return;
                }
            }catch(error){
                const err = error as string;
                toast.rojo(`Error al verificar ocupamiento de mesa o su existencia: ${err}`);
                return;
            }
            
            const nuevoInfoTipoPedido: PedidoLocal = {pedido_id: originalPedidoId, mesa, piso,mesero: meseroSeleccionado,id: sameOrZeroId,tipo: "Local"};
            const pedidoActualizado: Pedido = {nombre_cliente: nuevoNombreCliente,nota: nuevoNotaCliente, estado: nuevoEstado, tipo: nuevoTipo,fecha_hora:fechaHora,total,id,platillos_pedidos:platillosPedidosCopy, info_tipo_pedido:nuevoInfoTipoPedido};
            onGuardadCambios(pedidoActualizado);
        }else if(nuevoTipo == "Domicilio"){
            const coloniaDefinitive = returnNullOrStringValue(colonia);
            const calleDefinitive = returnNullOrStringValue(calle);
            const numeroInteriorExteriorDefinitive = parsearStringToInt(numeroExteriorInterior);
            const telefonoDefinitive = returnNullOrStringValue(telefono);
            const repartidorDefinitive = returnNullOrStringValue(repartidor);
            const nuevoInfoTipoPedido: PedidoDomicilio = {pedido_id: originalPedidoId,colonia:coloniaDefinitive, calle:calleDefinitive,numero_interior_exterior:numeroInteriorExteriorDefinitive,telefono:telefonoDefinitive, repartidor:repartidorDefinitive, id:sameOrZeroId, tipo:"Domicilio"};
            const pedidoActualizado: Pedido = {nombre_cliente: nuevoNombreCliente,nota: nuevoNotaCliente, estado: nuevoEstado, tipo: nuevoTipo,fecha_hora:fechaHora,total,id,platillos_pedidos:platillosPedidosCopy, info_tipo_pedido:nuevoInfoTipoPedido};
            onGuardadCambios(pedidoActualizado);
        }else{
            const nuevoInfoTipoPedido: PedidoRecoger = {id:sameOrZeroId, pedido_id:originalPedidoId,tipo: "Recoger"};
            const pedidoActualizado: Pedido = {nombre_cliente: nuevoNombreCliente,nota: nuevoNotaCliente, estado: nuevoEstado, tipo: nuevoTipo,fecha_hora:fechaHora,total,id,platillos_pedidos:platillosPedidosCopy, info_tipo_pedido:nuevoInfoTipoPedido};
            onGuardadCambios(pedidoActualizado);
        }
        
    }

</script>

{#if isAbierto}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div transition:fade={{ duration: 150 }}
        class="fixed inset-0 bg-black/50 flex items-start justify-center z-50 overflow-y-auto w-screen h-screen py-8"
        onkeydown={manejarTeclado}
    >
        <div transition:fly={{ y: -15, duration: 200, easing: cubicOut }}
        class="bg-white dark:bg-[#1a1f2e] rounded-lg shadow-xl p-6 w-full max-w-sm mx-4"
        onclick={(e) => e.stopPropagation()}
        >

            <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Editar orden</h2>

            <label for="nombre" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Nombre cliente</label>
            <input id="nombre" class="w-full mb-4 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500"
                type="text" use:autofocus
                bind:value={nombreCliente}
                onkeydown={manejarTeclado}
            >

            <label for="nota" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Nota</label>
            <input bind:value={notaCliente} onkeydown={manejarTeclado} type="text" id="nota" class="w-full mb-4 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500">
            
            <div class="flex flex-row space-x-20">

                <div class="flex flex-col">
                    <label for="estado" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Estado</label>
                    <label><input type="radio" name="estadoPedido" value="Pendiente" bind:group={estadoPedido} />Pendiente</label>
                    <label><input type="radio" name="estadoPedido" value="Finalizado" bind:group={estadoPedido} />Finalizado</label>
                    <label><input type="radio" name="estadoPedido" value="Cancelado" bind:group={estadoPedido} />Cancelado</label>
                    <label><input type="radio" name="estadoPedido" value="Entregado" bind:group={estadoPedido} />Entregado</label>
                    <label><input type="radio" name="estadoPedido" value="Cobrado" bind:group={estadoPedido} />Cobrado</label>
                </div>

                <div class="flex flex-col">
                    <label for="tipo" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Tipo</label>
                    <label><input type="radio" name="tipoPedido" value="Local" bind:group={tipoPedido} />Local</label>
                    <label><input type="radio" name="tipoPedido" value="Domicilio" bind:group={tipoPedido} />Domicilio</label>
                    <label><input type="radio" name="tipoPedido" value="Recoger" bind:group={tipoPedido} />Recoger</label>
                </div>

            </div>

            <div>
                {#if tipoPedido == "Local"}
                    <label for="piso" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Piso</label>
                    <div class="flex flex-row w-fit">
                        <input type="number" bind:value={piso} id="piso" class="text-gray-900 dark:text-white border-gray-300 bg-transparent focus:outline-none focus:ring-2 focus:ring-teal-500 dark:border-white/20 border rounded-l px-2 py-1 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none">
                        <div class="flex flex-col">
                            <button onclick={decrementarPiso} class="border rounded-tr cursor-pointer border-gray-300 dark:border-white/20 hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex-1">-</button>
                            <button onclick={incrementarPiso} class="border rounded-br cursor-pointer border-gray-300 dark:border-white/20 hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex-1">+</button>
                        </div>
                    </div>
                    <label for="mesa" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Mesa</label>
                    <div class="flex flex-row w-fit">
                        <input type="number" bind:value={mesa} id="mesa" class="text-gray-900 dark:text-white bg-transparent focus:outline-none focus:ring-2 focus:ring-teal-500 border-gray-300 dark:border-white/20 border rounded-l px-2 py-1 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none">
                        <div class="flex flex-col">
                            <button onclick={decrementarMesa} class="border rounded-tr cursor-pointer border-gray-300 dark:border-white/20 hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex-1">-</button>
                            <button onclick={incrementarMesa} class="border rounded-br cursor-pointer border-gray-300 dark:border-white/20 hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex-1">+</button>
                        </div>
                    </div>
                    <label for="mesero" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Mesero</label>
                    <input list="opciones-meseros" onchange={alConfirmarMesero} bind:value={meseroSeleccionado} onkeydown={manejarTeclado} type="text" id="mesero" class="w-full mb-4 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500">
                    <datalist id="opciones-meseros">
                        {#each meseros as opcion}
                            <option value={opcion.nombre}></option>
                        {/each}
                    </datalist>
                {:else if tipoPedido == "Domicilio"}
                <label for="colonia" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Colonia</label>
                <input bind:value={colonia} onkeydown={manejarTeclado} type="text" id="colonia" class="w-full mb-4 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500">
                
                <label for="calle" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Calle</label>
                <input bind:value={calle} onkeydown={manejarTeclado} type="text" id="calle" class="w-full mb-4 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500">
                
                <label for="numeroExteriorInterior" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Número interior/exterior</label>
                <input bind:value={numeroExteriorInterior} onkeydown={manejarTeclado} id="numeroExteriorInterior" class="border border-gray-300 dark:border-white/20 rounded w-full mb-4 px-3 py-2 focus:ring-2 focus:ring-teal-500 bg-transparent [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none" type="number"/>
                
                <label for="telefono" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Teléfono</label>
                <input bind:value={telefono} id="telefono" onkeydown={manejarTeclado} class="border border-gray-300 dark:border-white/20 rounded w-full mb-4 px-3 py-2 bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500" type="tel" oninput={(e) => (telefono = sanitizarInputTelefono(e))}/>
                
                 <label for="repartidor" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">Repartidor</label>
                 <input bind:value={repartidor} onkeydown={manejarTeclado} type="text" id="repartidor" class="w-full mb-4 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500">
                {/if}
            </div>

            <div class="flex flex-row space-x-3.5 items-end justify-end">
                <button onclick={cancelar} class="border p-1 rounded bg-red-400 hover:bg-red-500 cursor-pointer">Cancelar</button>
                <button class="border p-1 rounded cursor-pointer bg-green-400 hover:bg-green-500" onclick={onGuardar}>Aceptar</button>
            </div>

        </div>

    </div>
    
{/if}
