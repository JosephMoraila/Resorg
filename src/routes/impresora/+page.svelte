<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { toast } from "$lib/toast.svelte";

    let impresoras = $state<string[]>([]);
    let impresoraSeleccionada = $state("");
    let imprimirAlCrearOrden: boolean = $state(false);
    

    onMount(()=>{
        const getPrintes = async()=>{
            const impresorasBackend = await invoke<string[]>("obtener_impresoras");
            impresoras = impresorasBackend;
        };
        const loadData = async()=>{
            const [printerNameBackend, isPrintOrderBackend] = await invoke<[string, boolean]>("get_printting_settings");
            impresoraSeleccionada = printerNameBackend;
            imprimirAlCrearOrden = isPrintOrderBackend;
        };
        getPrintes();
        loadData();
    });

    $effect(()=>{
        const actualizar = async()=>{
            const params = {printerName: impresoraSeleccionada, isPrintOrder: imprimirAlCrearOrden};
            try{
                invoke("save_printting_settings", params);
            }catch(error){
                const err = error as string;
                toast.rojo(`Error al guardar datos: ${err}`);

            }
        };
        actualizar();
    });

</script>

<main class="bg-white dark:bg-black min-h-screen flex flex-col items-center justify-center p-6">
    <div class="w-full max-w-sm">


        <p class="text-lg font-semibold text-gray-900 dark:text-white mb-3">Impresoras</p>

        <select
        bind:value={impresoraSeleccionada}
        class="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-white/20 focus:outline-none focus:ring-2 focus:ring-teal-500 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        style="background-color: {imprimirAlCrearOrden ? '' : ''}; color: #111827;"
        >
        <option value="" disabled selected style="color: #6b7280;">
            Selecciona una impresora
        </option>
        {#each impresoras as impresora}
            <option value={impresora} style="color: #111827;">{impresora}</option>
        {/each}
        </select>

        <label class="flex items-center gap-2 mb-5 cursor-pointer select-none mt-5">
        <input
            type="checkbox"
            bind:checked={imprimirAlCrearOrden}
            class="w-4 h-4 rounded border-gray-300 dark:border-white/20 text-teal-600 focus:ring-teal-500 focus:ring-2 accent-teal-600"
        />
        <span class="text-sm font-medium text-gray-900 dark:text-white">
            Imprimir al crear orden
        </span>
        </label>

    </div>
</main>