<script lang="ts">

    import type { Mesero } from "$lib/types";
    import { toast } from "$lib/toast.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    let meseros = $state<Mesero[]>([]);
    let newMesero = $state("");
    let selectedMesero = $state<number | null>(null);

    onMount(()=>{
        const getData = async()=>{
            try{
                const data = await invoke<Mesero[]>("get_meseros");
                meseros = data;     
            }catch(error){
                const err = error as string;
                toast.rojo(`Error al obtener información: ${err}`);
            }
        };
        getData();
    });

    async function addMesero() {
        const trimmedMesero = newMesero.trim();
        if(trimmedMesero == "") return;
        const meseroEncontrado = meseros.find(m=>m.nombre == trimmedMesero);
        if(meseroEncontrado){
            toast.amarillo(`Ya existe un mesero con el nombre: ${trimmedMesero}`);
            newMesero = "";
            return;
        }
        try{
            const newId = await invoke<number>("insert_mesero", {nombre: trimmedMesero});
            const newMeseroTemp: Mesero = {id: newId, nombre: trimmedMesero};
            meseros.push(newMeseroTemp);
            newMesero = "";
        }catch(error){
            const err = error as string;
            toast.rojo(`Error insertando nuevo mesero: ${err}`);
        }
    }

    async function onDeleteMesero() {
        if(selectedMesero == null) return;
        try{
            await invoke("delete_mesero", {id: selectedMesero});
            const newList = meseros.filter(m=>m.id !== selectedMesero);
            meseros = newList;
            selectedMesero = null;
        }catch(error){
            const err = error as string;
            toast.rojo(`Error eliminando mesero: ${err}`);
        }
    }

</script>

<main class="text-black dark:text-white flex flex-col min-h-screen dark:bg-gray-700">
    <!-- Encabezado -->
    <div class="flex flex-col justify-center items-center py-4">
        <p class="font-bold text-xl">MESEROS</p>
    </div>

    <!-- Contenedor Principal: items-start pega los elementos arriba -->
    <div class="flex flex-row w-full flex-1 items-start pt-8">
        
        <!-- Div 1: Inputs y Botones (Alineado arriba y centrado horizontalmente) -->
        <div class="flex-1 flex flex-col items-center justify-start gap-3 px-4">
            <input class="border px-2 py-1 rounded w-64 text-black dark:text-white" bind:value={newMesero} type="text" placeholder="Nombre del mesero">
            <button class="btn-realista w-64" onclick={addMesero}>Agregar</button>
            <button class="btn-realista w-64" onclick={onDeleteMesero}>Eliminar</button>
        </div>

        <!-- Div 2: Tabla (Alineada arriba y centrada horizontalmente) -->
        <div class="flex-1 flex justify-center items-start px-4 overflow-y-auto">
            <table class="w-full max-w-xs border-collapse border border-gray-300 text-center">
                <thead>
                    <tr class="bg-gray-100 dark:bg-gray-800">
                        <th class="border p-2">Meseros</th>
                    </tr>
                </thead>
                <tbody>
                    {#each meseros as mesero (mesero.id) }
                        <tr>
                            <td class:bg-blue-600={selectedMesero == mesero.id} class:text-white={selectedMesero === mesero.id} onclick={()=>selectedMesero = mesero.id} class="border p-2 cursor-pointer hover:bg-gray dark:hover:bg-gray-500">{mesero.nombre}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>

    </div>
</main>