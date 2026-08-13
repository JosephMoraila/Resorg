<script lang="ts">
  import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { invoke } from "@tauri-apps/api/core";

  let isOpenMenu = $state(false);
  let submenuAbierto = $state<string | null>(null);

  const recordVentanas: Record<string, {url:string, title: string, width: number, height: number}> = {
    "registrar-platillos": {title: "Registrar platillos", url: "/registrar-platillos", height: 800, width: 800, },
    "registrar-mesas": {title: "Registrar mesas", url: "/registrar-mesas", height: 800, width: 800, },
    "meseros": {title: "Meseros", url: "/meseros", height: 800, width: 800,},
    "impresora": {title: "Impresora", url: "/impresora", height: 800, width: 800},
  };

  const menuOptions: { value: string; label: string; suboptions: { value: string; label: string }[] }[] = [
    {
      value: "platillos",
      label: "Platillos",
      suboptions: [{ value: "registrar-platillos", label: "Registrar platillos"}]
    },
    {
      label: "Mesas",
      value: "mesas",
      suboptions: [{ value: "registrar-mesas", label: "Registrar mesas"}]
    },
    {
      label: "Meseros",
      value: "meseros",
      suboptions: [{value: "meseros", label: "Meseros"}]
    },
    {
      value: "impresora",
      label: "Ticket",
      suboptions: [{label: "Impresora", value: "impresora"}]
    }

  ] as const;

  function manejarTeclado(event: KeyboardEvent) {
    if (event.key === "Escape") {
      isOpenMenu = false;
      submenuAbierto = null;
    }
  }

  async function abrirVentana(value: string){
    const info = recordVentanas[value];
    new WebviewWindow(value, {...info, center: true, visible: true});
    const paramEnfocarVentana = {label: value};
    invoke("enfocar_ventana", paramEnfocarVentana);
  }

</script>

<div
  class="relative"
  role="menu"
  tabindex="-1"
  onmouseleave={() => {
    isOpenMenu = false;
    submenuAbierto = null;
  }}
  onkeydown={manejarTeclado}
>
  <button
    onclick={() => (isOpenMenu = !isOpenMenu)}
    class="bg-transparent border-none text-gray-600 dark:text-white/75 text-sm px-4 cursor-pointer h-12 hover:bg-black/10 dark:hover:bg-white/10 hover:text-gray-900 dark:hover:text-white transition-colors"
  >
    Configuración {isOpenMenu ? "▴" : "▾"}
  </button>

  {#if isOpenMenu}
    <div class="absolute left-0 top-full pt-1 z-50">
      <div
        class="bg-gray-100 dark:bg-[#1a1f2e] border border-gray-300 dark:border-white/10 rounded-md min-w-36 shadow-lg"
      >
        {#each menuOptions as opcion (opcion.value)}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="relative"
            role=""
            onmouseenter={() => (submenuAbierto = opcion.value)}
            onmouseleave={() => (submenuAbierto = null)}
          >
            <button
              class="w-full text-left px-4 py-2 text-sm transition-colors cursor-pointer text-gray-600 dark:text-white/75 hover:bg-black/10 dark:hover:bg-white/10 hover:text-gray-900 dark:hover:text-white"
              role="menuitem"
            >
              {opcion.label} {submenuAbierto === opcion.value ? "▸" : ""}
            </button>

            {#if submenuAbierto === opcion.value}
              <div class="absolute left-full top-0 pl-1 z-50">
                <div
                  class="bg-gray-100 dark:bg-[#1a1f2e] border border-gray-300 dark:border-white/10 rounded-md min-w-36 shadow-lg"
                >
                  {#each opcion.suboptions as sub (sub.value)}
                    <button
                      class="w-full text-left px-4 py-2 text-sm transition-colors cursor-pointer text-gray-600 dark:text-white/75 hover:bg-black/10 dark:hover:bg-white/10 hover:text-gray-900 dark:hover:text-white"
                      role="menuitem" onclick={()=>abrirVentana(sub.value)}
                    >
                      {sub.label}
                    </button>
                  {/each}
                </div>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>