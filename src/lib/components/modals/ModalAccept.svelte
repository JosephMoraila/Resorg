<script lang="ts">

    import { fade, fly } from "svelte/transition";
    import { cubicOut } from "svelte/easing";

    interface ModalAcceptProps {
        abierto: boolean;
        title: string;
        message: string;
        onResult: (result: boolean) => void;
    }

    let {title, message, onResult, abierto}:ModalAcceptProps = $props();

    function aceptar() {
        onResult(true);
    }

    function cancelar() {
        onResult(false);
    }

    function manejarTeclado(event: KeyboardEvent) {
        if (event.key === "Escape") cancelar();
    }

    function autofocus(node: HTMLDivElement) {
        node.focus();
    }

</script>

{#if abierto}

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  transition:fade={{ duration: 150 }}
  class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
  onclick={cancelar}
  onkeydown={manejarTeclado}
>
    <div use:autofocus transition:fly={{ y: -15, duration: 200, easing: cubicOut }} class="bg-white dark:bg-[#1a1f2e] text-black dark:text-white rounded-lg shadow-xl p-6 w-full max-w-sm mx-4" onclick={(e) => e.stopPropagation()}>
        <h2 class="text-lg font-semibold mb-4">{title}</h2>
        <p class="mb-6">{message}</p>
        <div class="flex justify-end space-x-4">
            <button class="btn-realista" onclick={cancelar}>Cancelar</button>
            <button class="btn-realista" onclick={aceptar}>Aceptar</button>
        </div>
    </div>
</div>
{/if}