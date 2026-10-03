<script lang="ts">
    import "../app.css";
    import { themeStore } from "../lib/theme.svelte";
    import Toast from "$lib/components/notification/Toast.svelte";
    
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMount } from "svelte";

    let { children } = $props();

    $effect(() => {
        themeStore.inicializar();
    });

    //Le decimos a la app que apenas termine de montar toda la estructura, se muestre
    onMount(async () => {
        try {
            await getCurrentWindow().show();
        } catch (error) {
            console.error("Error al mostrar la ventana principal:", error);
        }
    });

    function handleGlobalContextMenu(e: MouseEvent) {
        const target = e.target as HTMLElement;
        const tag = target.tagName.toLowerCase();
        
        const isInput = tag === 'input' || tag === 'textarea' || target.isContentEditable;
        const hasSelection = (window.getSelection()?.toString() || '').trim().length > 0;

        if (!isInput && !hasSelection) {
            e.preventDefault();
        }
    }
</script>

<svelte:window oncontextmenu={handleGlobalContextMenu} />

<Toast />
{@render children()}