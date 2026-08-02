<script lang="ts">

    import { fade, fly } from "svelte/transition";
    import { cubicOut } from "svelte/easing";

  interface Props {
    abierto: boolean;
    valor: string;
    titulo: string;
    descripcion?: string;
    placeholder?: string;
    onConfirmar: (valor: string | null) => void;
    onCancelar?: () => void;
  }

  let {
    abierto = $bindable(),
    valor = $bindable(""),
    titulo,
    descripcion,
    placeholder = "",
    onConfirmar,
    onCancelar,
  }: Props = $props();

    function autofocus(node: HTMLInputElement) {
        node.focus();
    }

  function confirmar() {
    const trimmedValor = valor.trim();
    if (trimmedValor == "") onConfirmar(null);
    onConfirmar(valor);
    valor = "";
    abierto = false;
  }

  function cancelar() {
    valor = "";
    abierto = false;
    onCancelar?.();
  }

  function manejarTeclado(event: KeyboardEvent) {
    if (event.key === "Escape") cancelar();
    if (event.key === "Enter") confirmar();
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
    <div
        transition:fly={{ y: -15, duration: 200, easing: cubicOut }}
      class="bg-white dark:bg-[#1a1f2e] rounded-lg shadow-xl p-6 w-full max-w-sm mx-4"
      onclick={(e) => e.stopPropagation()}
    >
      <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
        {titulo}
      </h2>

      {#if descripcion}
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
          {descripcion}
        </p>
      {/if}

      <input
        use:autofocus
        bind:value={valor}
        {placeholder}
        class="w-full mt-4 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500"
        onkeydown={manejarTeclado}
      />

      <div class="flex flex-row justify-end gap-2 mt-5">
        <button
          onclick={cancelar}
          class="cursor-pointer px-4 py-2 text-sm rounded-md text-gray-600 dark:text-gray-300 hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
        >
          Cancelar
        </button>
        <button
          onclick={confirmar}
          class="cursor-pointer px-4 py-2 text-sm rounded-md bg-teal-600 hover:bg-teal-700 text-white transition-colors"
        >
          Confirmar
        </button>
      </div>
    </div>
  </div>
{/if}