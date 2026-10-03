<script lang="ts">
    import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';
    import { onMount } from 'svelte';
    // 1. Importamos getCurrentWindow (API de Ventanas, no de Webviews)
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { toast } from '$lib/toast.svelte';

    let arrancaConPC = $state(false);

    async function onMostrarVentanaAlMontarse() {
        try {
            // 2. Simplemente ordenamos a la ventana actual que se muestre
            await getCurrentWindow().show();
        } catch(err) {
            const error = err as string;
            toast.rojo(`Error al mostrar ventana: ${error}`);
        }
    }

    onMount(async () => {
        arrancaConPC = await isEnabled();
        onMostrarVentanaAlMontarse();
    });

    async function toggleAutostart() {
        if (arrancaConPC) {
            await disable();
            arrancaConPC = false;
        } else {
            await enable();
            arrancaConPC = true;
        }
    }
</script>

<main class="text-black dark:text-white flex flex-col min-h-screen dark:bg-black">
    <div class="flex items-center gap-3 flex-col">
        <h1 class="text-2xl font-bold">Iniciar al arrancar</h1>
        <button 
            onclick={toggleAutostart}
            class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
        >
            {arrancaConPC ? 'Desactivar inicio automático' : 'Activar inicio automático'}
        </button>
        <span>
            Estado: {arrancaConPC ? 'Activado' : 'Desactivado'}
        </span>
    </div>
</main>