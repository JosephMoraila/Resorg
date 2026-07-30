<script lang="ts">
  import { themeStore } from "../../theme.svelte";

  let isOpen = $state(false);

  const themeOptions = [
    { value: "claro", label: "Claro" },
    { value: "oscuro", label: "Oscuro" },
    { value: "sistema", label: "Sistema" },
  ] as const;

  function seleccionar(valor: "claro" | "oscuro" | "sistema") {
    themeStore.setTheme(valor);
    isOpen = false;
  }

  function manejarTeclado(event: KeyboardEvent) {
    if (event.key === "Escape") {
      isOpen = false;
    }
  }
</script>

<div
  class="relative"
  role="menu"
  tabindex="-1"
  onmouseleave={() => (isOpen = false)}
  onkeydown={manejarTeclado}
>
  <button
    onclick={() => (isOpen = !isOpen)}
    class="bg-transparent border-none text-gray-600 dark:text-white/75 text-sm px-4 cursor-pointer h-12 hover:bg-black/10 dark:hover:bg-white/10 hover:text-gray-900 dark:hover:text-white transition-colors"
  >
    Apariencia {isOpen ? "▴" : "▾"}
  </button>

  {#if isOpen}
    <div class="absolute left-0 top-full pt-1 z-50">
      <div
        class="bg-gray-100 dark:bg-[#1a1f2e] border border-gray-300 dark:border-white/10 rounded-md min-w-36 shadow-lg"
      >
        {#each themeOptions as opt (opt.value)}
          <button
            role="menuitem"
            onclick={() => seleccionar(opt.value)}
            class="w-full text-left px-4 py-2 text-sm transition-colors cursor-pointer {themeStore.value ===
            opt.value
              ? 'text-teal-600 dark:text-teal-300 bg-black/10 dark:bg-white/10'
              : 'text-gray-600 dark:text-white/75 hover:bg-black/10 dark:hover:bg-white/10 hover:text-gray-900 dark:hover:text-white'}"
          >
            {opt.label}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>