<script lang="ts">
  import Apariencia from "./Apariencia.svelte";
  import Configuracion from "./Configuracion.svelte";
</script>

<nav
  class="w-full bg-gray-200 dark:bg-[#1a1f2e] text-gray-800 dark:text-white px-4 flex flex-row items-center flex-wrap min-h-12 transition-colors"
>
  <div
    class="font-medium text-[15px] px-4 pl-2 mr-2 border-r border-gray-400/40 dark:border-white/15 text-teal-600 dark:text-teal-300 h-12 flex items-center"
  >
    RESORG
  </div>

  <div class="flex flex-row flex-wrap items-stretch">
    <Apariencia />
    <Configuracion/>
  </div>
</nav>