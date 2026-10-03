<script lang="ts">
    import "../app.css";
    import { themeStore } from "../lib/theme.svelte";
    import Toast from "$lib/components/notification/Toast.svelte";

    let { children } = $props();

    $effect(() => {
        themeStore.inicializar();
    });

    function handleGlobalContextMenu(e: MouseEvent) {
        const target = e.target as HTMLElement;
        const tag = target.tagName.toLowerCase();
        
        const isInput = tag === 'input' || tag === 'textarea' || target.isContentEditable;
        
        //Agregamos ( || '') para que nunca sea undefined al hacer .trim()
        const hasSelection = (window.getSelection()?.toString() || '').trim().length > 0;

        if (!isInput && !hasSelection) {
            e.preventDefault();
        }
    }
</script>

<!-- poner el svelte:window para que la función realmente escuche los clicks -->
<svelte:window oncontextmenu={handleGlobalContextMenu} />

<Toast />
{@render children()}