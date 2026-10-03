<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "$lib/toast.svelte";
  import { getCurrentWebviewWindow, WebviewWindow } from "@tauri-apps/api/webviewWindow";

  let impresoras = $state<string[]>([]);
  let impresoraSeleccionada = $state("");
  let imprimirAlCrearOrden: boolean = $state(false);
  let imprimirAlCobrar: boolean = $state(false);

  async function onMostrarVentanaAlMontarse() {
    try{
        const label = getCurrentWebviewWindow().label;
        const ventana = await WebviewWindow.getByLabel(label);
        await ventana?.show();
    }catch(err){
        const error = err as string;
        toast.rojo(`Error al mostrar ventana: ${error}`);
    }
  }

  onMount(async() => {
    const getPrinters = async () => {
      const impresorasBackend = await invoke<string[]>("obtener_impresoras");
      impresoras = impresorasBackend;
    };
    const loadData = async () => {
      const [printerNameBackend, isPrintOrderBackend, isImprimirAlCobrar] = await invoke<[string, boolean, boolean]>("get_printting_settings");
      impresoraSeleccionada = printerNameBackend;
      imprimirAlCrearOrden = isPrintOrderBackend;
      imprimirAlCobrar = isImprimirAlCobrar;
    };
    getPrinters();
    try{
      loadData();
    }catch(error){
      const err = error as string;
      toast.rojo(`Error al obtener datos: ${err}`);
    }
    onMostrarVentanaAlMontarse();
  });

  async function guardarConfiguracion() {
    const params = { printerName: impresoraSeleccionada, isPrintOrder: imprimirAlCrearOrden, isImprimirAlCobrar: imprimirAlCobrar};
    try {
      await invoke("save_printting_settings", params);
    } catch (error) {
      const err = error as string;
      toast.rojo(`Error al guardar datos: ${err}`);
    }
  }
</script>

<main class="bg-white dark:bg-black min-h-screen flex flex-col items-center justify-center p-6">
  <div class="w-full max-w-sm">
    <p class="text-lg font-semibold text-gray-900 dark:text-white mb-3">Impresoras</p>

    <select
      bind:value={impresoraSeleccionada}
      onchange={guardarConfiguracion}
      class="form-select w-full px-4 py-2.5 rounded-lg border bg-white dark:bg-[#1a1f2e] text-gray-900 dark:text-white border-gray-300 dark:border-white/20 focus:outline-none focus:ring-2 focus:ring-teal-500 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
    >
      <option value="" disabled selected class="text-gray-500">
        Selecciona una impresora
      </option>
      {#each impresoras as impresora}
        <option class="bg-white dark:bg-[#1a1f2e] text-gray-900 dark:text-white" value={impresora}>{impresora}</option>
      {/each}
    </select>

    <label class="flex items-center gap-2 mb-5 cursor-pointer select-none mt-5">
      <input
        type="checkbox"
        bind:checked={imprimirAlCrearOrden}
        onchange={guardarConfiguracion}
        class="w-4 h-4 rounded border-gray-300 dark:border-white/20 text-teal-600 focus:ring-teal-500 focus:ring-2 accent-teal-600"
      />
      <span class="text-sm font-medium text-gray-900 dark:text-white">
        Imprimir al crear orden
      </span>
    </label>

    <label class="flex items-center gap-2 mb-5 cursor-pointer select-none mt-5">
      <input
        type="checkbox"
        bind:checked={imprimirAlCobrar}
        onchange={guardarConfiguracion}
        class="w-4 h-4 rounded border-gray-300 dark:border-white/20 text-teal-600 focus:ring-teal-500 focus:ring-2 accent-teal-600"
      />
      <span class="text-sm font-medium text-gray-900 dark:text-white">
        Imprimir al cobrar
      </span>
    </label>

  </div>
</main>