<script lang="ts">

    import { fade, fly } from "svelte/transition";
    import { cubicOut } from "svelte/easing";

  interface Props {
    abierto: boolean;
    valor1: string;
    valor2: string;
    titulo: string;
    descripcion1?: string;
    descripcion2?: string;
    placeholderUno?: string;
    placeholderDos?: string;
    onConfirmar: (valor1: string | null, valor2: string | null) => void;
    onCancelar?: () => void;
  }

  let {
    abierto = $bindable(),
    valor1 = $bindable(""),
    valor2 = $bindable(""),
    titulo,
    descripcion1, descripcion2,
    placeholderUno = "", placeholderDos = "",
    onConfirmar,
    onCancelar,
  }: Props = $props();

    function autofocus(node: HTMLInputElement) {
        node.focus();
    }

  function confirmar() {
    const trimmedValor1 = valor1.trim();
    const trimmedValor2 = valor2.trim();
    if (trimmedValor1 == "" && trimmedValor2 == "") onConfirmar(null, null);
    else if(trimmedValor1 != "" && trimmedValor2 == "") onConfirmar(trimmedValor1, null);
    else if(trimmedValor1 == "" && trimmedValor2 != "") onConfirmar(null, trimmedValor2);
    else onConfirmar(trimmedValor1, trimmedValor2);
    valor1 = "";
    valor2 = "";
    abierto = false;
  }

  function cancelar() {
    valor1 = "";
    valor2 = "";
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

      {#if descripcion1}
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
          {descripcion1}
        </p>
      {/if}

      <input
        use:autofocus
        bind:value={valor1}
        placeholder={placeholderUno}
        class="w-full mt-2 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500"
        onkeydown={manejarTeclado}
      />

      {#if descripcion2}
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
          {descripcion2}
        </p>
      {/if}

      <input
        bind:value={valor2}
        placeholder={placeholderDos}
        class="w-full mt-2 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500"
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