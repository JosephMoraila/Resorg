<script lang="ts">

    import { fade, fly } from "svelte/transition";
    import { cubicOut } from "svelte/easing";
    import type { PedidoPlatillo, FilaPlatillo, PlatilloCategoria, Platillo, PlatilloPedido, UpdatePlatilloPedido } from "$lib/types";
    import { onMount, untrack } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { toast } from "$lib/toast.svelte";
    import { isContieneSoloNumeros } from "$lib/utils/string_utils";
    import { TruncarToEnteroPositivo } from "$lib/utils/math_utils";

    interface Props{
        isAbierto: boolean; platillosPedidos: PedidoPlatillo[], onGuardadCambios(pedido: UpdatePlatilloPedido[]): Promise<void>;
    }

    let {
        isAbierto = $bindable(),  platillosPedidos, onGuardadCambios
    }: Props = $props();

    let platillosCategoriaDb: PlatilloCategoria[] = $state([]);

    const originalPlatillosPedidos: PedidoPlatillo[] = untrack(() => $state.snapshot(platillosPedidos));
    const pedido_id_orignal = originalPlatillosPedidos[0].pedido_id;

    let filas: FilaPlatillo[] = $state([]);//Las filas que muestran la cantidad y nombre del platillo estan vacios porque se llenan en el onmount

    // Lista plana de TODOS los platillos, sin importar la categoría -- se recalcula sola
    let todosLosPlatillos = $derived(platillosCategoriaDb.flatMap((c) => c.platillos));
    // Lo que se muestra en el datalist: nombres de categorías + nombres de platillos, juntos
    let opcionesPlatillo = $derived([
        ...platillosCategoriaDb.map((c) => c.nombre),
        ...todosLosPlatillos.map((p) => p.nombre),
    ]);
    function buscarCategoria(nombre: string): PlatilloCategoria | undefined {
        return platillosCategoriaDb.find((c) => c.nombre === nombre);
    }
    function buscarPlatillo(nombre: string): Platillo | undefined {
        return todosLosPlatillos.find((p) => p.nombre === nombre);
    }
    function agregarFila() {
        filas.push({id: crypto.randomUUID(),nombre: "",id_platillo: null,id_category: null,cantidad: 1,});
        console.log(filas);
    }
    function eliminarFila(id: string) {
        if (filas.length === 1) return;
        const index = filas.findIndex((f) => f.id === id);
        if (index !== -1) filas.splice(index, 1);
    }
    // Se llama cuando el usuario confirma (sale del input) el "Platillo o categoría" de una fila
    function alConfirmarFila(fila: FilaPlatillo) {
        // Caso 1: es un platillo válido -- se queda tal cual, guarda su id y el id de su categoría
        const platillo = buscarPlatillo(fila.nombre);
        if (platillo) {
            fila.id_platillo = platillo.id;
            fila.id_category = platillo.id_categoria;
            return;
        }

        // Caso 2: es una categoría válida -- se expande en varias filas, una por platillo
        const categoria = buscarCategoria(fila.nombre);
        if (categoria) {
            const index = filas.findIndex((f) => f.id === fila.id);
            if (index === -1) return;

            const nuevasFilas: FilaPlatillo[] = categoria.platillos.map((p) => ({
                id: crypto.randomUUID(),
                nombre: p.nombre,
                id_platillo: p.id,
                id_category: p.id_categoria,
                cantidad: 1,
            }));

            if (nuevasFilas.length === 0) {
                // categoría sin platillos -- no hay nada que expandir, se rechaza
                fila.nombre = "";
                fila.id_platillo = null;
                fila.id_category = null;
                return;
            }

            filas.splice(index, 1, ...nuevasFilas);
            return;
        }

        // Caso 3: no coincide con nada -- se rechaza, se limpia el input y los IDs
        fila.nombre = "";
        fila.id_platillo = null;
        fila.id_category = null;
    }

    onMount(()=>{
        const cloneOnMount: PedidoPlatillo[] = structuredClone(originalPlatillosPedidos);
        for(const p of cloneOnMount){
            const newFila: FilaPlatillo = {id: p.id.toString(), nombre: p.name, cantidad: 1, id_category: p.categoria_id, id_platillo: p.platillo_id};
            filas.push(newFila);
        }

        const getPlatillosDb = async()=>{
            try{
                const platillosBackend = await invoke<PlatilloCategoria[]>("get_categories_platillo");
                platillosCategoriaDb = platillosBackend;
            }catch(err){
                const error = err as string;
                toast.rojo(`Error al conseguir platillos de base de datos: ${error}`);
            }
        };
        getPlatillosDb();
    });

    function cancelar(){
        isAbierto = false;
    }

    function manejarTeclado(event: KeyboardEvent) {
        if (event.key === "Escape") cancelar();
        //if (event.key === "Enter") confirmar();
    }
    function autofocus(node: HTMLInputElement, index: number) {
        if(index === 0) node.focus();
    }

    function onGuardar(){
        // 1. Guardias de validación
        if (filas.length === 0) {
            toast.amarillo('Registra al menos un platillo');
            return;
        }
        if (filas.some(fila => fila.id_platillo === null)) {
            toast.amarillo('Algún campo de platillo no es válido');
            return;
        }
        if(filas.some(fila=>fila.cantidad <= 0)){
            toast.amarillo('Algúna cantidad no es válida');
            return;
        }
        let platilldosPedir: UpdatePlatilloPedido[] = [];
        for(const fila of filas){
            //Si se agrega un nueva fila se usa crypto lo cual ese ID que es string tiene guión y letras y con esta función se sabe si se agregó un nuevo platillo a pedir o no se cambiaron los originales
            const isOnlyNumbers = isContieneSoloNumeros(fila.id);
            if(isOnlyNumbers){//Es un platillo original, checar si es mayor a 1 porque si una fila con ID de platillo pedido no puede tener 2 de cantidad porque es ilogico
                if(fila.cantidad > 1){
                    const up: UpdatePlatilloPedido = {cantidad: TruncarToEnteroPositivo(fila.cantidad) - 1, id_platillo_pedido_original: null, id_category: fila.id_category, id_platillo: fila.id_platillo!, pedido_id: pedido_id_orignal}; //Es menos 1 porque al menos uno es el original
                    platilldosPedir.push(up);
                }
                //Ahora agregar el original
                platilldosPedir.push({cantidad: 1, id_platillo_pedido_original: Number(fila.id), id_category: fila.id_category, id_platillo: fila.id_platillo!, pedido_id: pedido_id_orignal});
            }else{//Se agregó uno nuevo que no estaba originalmente
                const up: UpdatePlatilloPedido = {cantidad: TruncarToEnteroPositivo(fila.cantidad), id_platillo_pedido_original: null, id_category: fila.id_category, id_platillo: fila.id_platillo!, pedido_id: pedido_id_orignal};
                platilldosPedir.push(up);
            }
        }
        onGuardadCambios(platilldosPedir);
    }

</script>

{#if isAbierto}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div transition:fade={{ duration: 150 }} class="fixed inset-0 bg-black/50 flex items-start justify-center z-50 overflow-y-auto w-screen h-screen py-8">
        <div transition:fly={{ y: -15, duration: 200, easing: cubicOut }} class="bg-white flex flex-col items-center justify-center dark:bg-[#1a1f2e] rounded-lg shadow-xl p-6 w-full max-w-sm mx-4" onclick={(e) => e.stopPropagation()}>

            <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Editar platillos (Pedido ID: {pedido_id_orignal})</h2>

            {#each filas as fila, index (fila.id)}
                <div class="flex flex-row gap-1 w-full mb-1">
                    <input list="opciones-platillos" bind:value={fila.nombre} onchange={() => alConfirmarFila(fila)} class="focus:outline-none focus:ring-2 focus:ring-teal-500 w-full border rounded px-3 py-2" onkeydown={manejarTeclado} use:autofocus={index}/>
                    <input type="number" bind:value={fila.cantidad} min="1" class="focus:outline-none focus:ring-2 focus:ring-teal-500 w-14 border rounded px-1 text-center [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none" onkeydown={manejarTeclado}/>
                    {#if filas.length > 1}
                        <button onclick={() => eliminarFila(fila.id)} class="border px-2 rounded cursor-pointer text-red-600 hover:bg-red-50 dark:hover:bg-red-500/10">
                            ×
                        </button>
                    {/if} 
                </div>
            {/each}

            <datalist id="opciones-platillos">
            {#each opcionesPlatillo as opcion}
                <option value={opcion}></option>
            {/each}
            </datalist>

            <button onclick={agregarFila} class="border px-1.5 rounded cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 mt-1">
                +
            </button>

            <div class="flex flex-row space-x-3.5 self-end mt-5">
                <button onclick={cancelar} class="border p-1 rounded bg-red-400 hover:bg-red-500 cursor-pointer">Cancelar</button>
                <button class="border p-1 rounded cursor-pointer bg-green-400 hover:bg-green-500" onclick={onGuardar}>Aceptar</button>
            </div>

        </div>
    </div>

{/if}