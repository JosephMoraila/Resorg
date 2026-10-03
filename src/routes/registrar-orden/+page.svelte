<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { emit } from "@tauri-apps/api/event";
  import type { TipoPedido, Platillo, PlatilloCategoria, Mesero, PlatilloPedido, FilaPlatillo } from "$lib/types";
  import { sanitizarInputTelefono } from "$lib/utils/string_utils";
  import { toast } from "$lib/toast.svelte";
  import { getCurrentWebviewWindow, WebviewWindow } from "@tauri-apps/api/webviewWindow";

  let tipoPedido = $state<TipoPedido>("Local");
  let telefono = $state("");

  let meseros: Mesero[] = $state([]);
  let meseroSeleccionado = $state("");

  let platillos: PlatilloCategoria[] = $state([]);

  let filas: FilaPlatillo[] = $state([
    { id: crypto.randomUUID(), nombre: "", id_platillo: null, id_category: null, cantidad: 1 },
  ]);

  let piso = $state(0);
  let mesa = $state(0);
  let nombreCliente = $state("");
  let colonia = $state("");
  let calle = $state("");
  let numeroExteriorInterior = $state("");
  let numeroCelular = $state("");
  let notas = $state("");
  let repartidor = $state("");

  // Lista plana de TODOS los platillos, sin importar la categoría -- se recalcula sola
  let todosLosPlatillos = $derived(platillos.flatMap((c) => c.platillos));

  // Lo que se muestra en el datalist: nombres de categorías + nombres de platillos, juntos
  let opcionesPlatillo = $derived([
    ...platillos.map((c) => c.nombre),
    ...todosLosPlatillos.map((p) => p.nombre),
  ]);

  function buscarCategoria(nombre: string): PlatilloCategoria | undefined {
    return platillos.find((c) => c.nombre === nombre);
  }

  function buscarPlatillo(nombre: string): Platillo | undefined {
    return todosLosPlatillos.find((p) => p.nombre === nombre);
  }

  onMount(async () => {
    try{
      const platillosBackend = await invoke<PlatilloCategoria[]>("get_categories_platillo");
      platillos = platillosBackend;
    }catch(err){
      const error = err as string;
      toast.rojo(`Error al conseguir platillos de base de datos: ${error}`);
    }

    try{
      const meserosBackend = await invoke<Mesero[]>("get_meseros");
      meseros = meserosBackend;
    }catch(err){
      const error = err as string;
      toast.rojo(`Error al conseguir meseros de base de datos: ${error}`);
    }

    try{
      const label = getCurrentWebviewWindow().label;
      const ventana = await WebviewWindow.getByLabel(label);
      await ventana?.show();
    }catch(err){
      const error = err as string;
      toast.rojo(`Error al mostrar ventana: ${error}`);
    }

  });

  function agregarFila() {
    filas.push({
      id: crypto.randomUUID(),
      nombre: "",
      id_platillo: null,
      id_category: null,
      cantidad: 1,
    });
  }

  function eliminarFila(id: string) {
    if (filas.length === 1) return;
    const index = filas.findIndex((f) => f.id === id);
    if (index !== -1) filas.splice(index, 1);
  }

  // Se llama cuando el usuario confirma (sale del input) el "Platillo o categoría" de una fila
  function alConfirmarFila(fila: FilaPlatillo) {
    // Caso 1: es un platillo válido -- se queda tal cual, guarda su id y el id de su categoría
    const platillo = buscarPlatillo(fila.nombre);
    if (platillo) {
      fila.id_platillo = platillo.id;
      fila.id_category = platillo.id_categoria;
      return;
    }

    // Caso 2: es una categoría válida -- se expande en varias filas, una por platillo
    const categoria = buscarCategoria(fila.nombre);
    if (categoria) {
      const index = filas.findIndex((f) => f.id === fila.id);
      if (index === -1) return;

      const nuevasFilas: FilaPlatillo[] = categoria.platillos.map((p) => ({
        id: crypto.randomUUID(),
        nombre: p.nombre,
        id_platillo: p.id,
        id_category: p.id_categoria,
        cantidad: 1,
      }));

      if (nuevasFilas.length === 0) {
        // categoría sin platillos -- no hay nada que expandir, se rechaza
        fila.nombre = "";
        fila.id_platillo = null;
        fila.id_category = null;
        return;
      }

      filas.splice(index, 1, ...nuevasFilas);
      return;
    }

    // Caso 3: no coincide con nada -- se rechaza, se limpia el input y los IDs
    fila.nombre = "";
    fila.id_platillo = null;
    fila.id_category = null;
  }

  function alConfirmarMesero() {
    const valido = meseros.some((m) => m.nombre === meseroSeleccionado);
    if (!valido) meseroSeleccionado = "";
  }

  function incrementarPiso() {
    piso++;
  }
  function decrementarPiso() {
    piso--;
  }
  function incrementarMesa() {
    mesa++;
  }
  function decrementarMesa() {
    if (mesa > 0) mesa--;
  }

  async function onRegistrarOrden() {
    // 1. Guardias de validación
    if (filas.length === 0) {
      toast.amarillo('Registra al menos un platillo');
      return;
    }

    if (filas.some(fila => fila.id_platillo === null)) {
      toast.amarillo('Algún campo de platillo no es válido');
      return;
    }

    if(filas.some(fila=>fila.cantidad <= 0)){
      toast.amarillo('Algúna cantidad no es válida');
      return;
    }

    if(tipoPedido === "Local"){
      if(mesa == null || mesa <= 0){
        toast.amarillo(`La mesa no es válida`);
        return;
      }
      if(!meseroSeleccionado){
        toast.amarillo(`Selecciona un mesero`);
        return;  
      }
      const pisoObj = {piso};
      const mesObj = {mesa};
      const params = {...mesObj, ...pisoObj};
      try{
        const isPisoExiste = await invoke<boolean>("is_piso_exists", pisoObj);
        if(!isPisoExiste){
          toast.amarillo(`El piso ${piso} no está registrado`);
          return;
        }
        const isMesaExiste = await invoke<boolean>("is_mesa_exists", mesObj);
        if(!isMesaExiste){
          toast.amarillo(`La mesa ${mesa} no está registrada`);
          return;
        }
        const isOcupada = await invoke<boolean>("is_mesa_ocupada", params);
        if(isOcupada){
          toast.amarillo(`Esa mesa ya está ocupada`);
          return;
        }
      }catch(error){
        const err = error as string;
        toast.rojo(`Error al verificar ocupamiento de mesa o su existencia: ${err}`);
        return;
      }
    }else if(tipoPedido == "Domicilio"){
      if(repartidor.trim() == ""){
        toast.amarillo(`Escribe un repartidor`);
        return;      
      }
    }

    // Mapeo directo y limpio
    const platillosPedidos: PlatilloPedido[] = filas.map(fila => ({
      id_category: fila.id_category,
      id_platillo: fila.id_platillo!, // Seguro por la validación previa
      cantidad: Math.trunc(fila.cantidad && fila.cantidad > 0 ? fila.cantidad : 1)
    }));

    const nombreClienteFinal = nombreCliente.trim() == "" ? null : nombreCliente.trim();
    const notasFinal = notas.trim() == "" ? null : notas.trim();

    if(tipoPedido == "Local"){
      const params = {platillos: platillosPedidos, piso, mesa, meseroSeleccionado, nombreCliente: nombreClienteFinal, notas: notasFinal};
      try{
        await invoke("insert_pedido_local", params);
        toast.verde(`Pedido local registrado`);
        await emit("pedido-creado");//Solo avisamos que se creó un pedido, para que la ventana de ver-ordenes-pendientes se actualice si está abierta
        inicializarDeNuevo();
      }catch(error){
        const err = error as string;
        toast.rojo(`Error al registrar pedido de local: ${err}`);
      }
    }else if(tipoPedido == "Domicilio"){
      const coloniaFinal = colonia.trim() == "" ? null : colonia.trim();
      const calleFinal = calle.trim() == "" ? null : calle.trim();
      const numeroExteriorInteriorInt = parseInt(numeroExteriorInterior);
      const numeroExteriorInteriorFinal = Number.isNaN(numeroExteriorInteriorInt) ? null : numeroExteriorInteriorInt;
      const numeroCelularFinal = numeroCelular.trim() == "" ? null : numeroCelular.trim();
      const repartidorTrimmed = repartidor.trim();
      const params = {platillos: platillosPedidos, nombreCliente: nombreClienteFinal, notas: notasFinal, colonia: coloniaFinal, calle: calleFinal, numeroExteriorInterior: numeroExteriorInteriorFinal, numeroCelular: numeroCelularFinal, repartidor: repartidorTrimmed};
      try{
        await invoke("insert_pedido_domicilio", params);
        toast.verde(`Pedido domicilio registrado`);
        await emit("pedido-creado");
        inicializarDeNuevo();
      }catch(error){
        const err = error as string;
        toast.rojo(`Error al registrar pedido de domicilio: ${err}`);
      }
    }else if(tipoPedido == "Recoger"){
      const params = {platillos: platillosPedidos, nombreCliente: nombreClienteFinal, notas: notasFinal};
      try{
        await invoke("insert_pedido_recoger", params);
        toast.verde(`Pedido recoger registrado`);
        await emit("pedido-creado");
        inicializarDeNuevo();
      }catch(error){
        const err = error as string;
        toast.rojo(`Error al registrar pedido de recoger: ${err}`);
      }
    }
  }

  function inicializarDeNuevo(){
    filas.length = 0; //Vaciamos la filas de nuevo
    filas.push({ id: crypto.randomUUID(), nombre: "", id_platillo: null, id_category: null, cantidad: 1 });
    telefono = "";
    meseroSeleccionado = "";
    piso = 0;
    mesa = 0;
    nombreCliente = "";
    colonia = "";
    calle = "";
    numeroExteriorInterior = "";
    numeroCelular = "";
    notas = "";
    repartidor = "";
  }
</script>

<main class="min-h-screen w-screen text-black dark:text-white bg-white dark:bg-black flex flex-col items-center justify-center pt-10">
  <div class="flex flex-col items-center w-1/2">
    <p class="text-center mb-1 text-xl font-bold">Platillo o categoría</p>

    {#each filas as fila (fila.id)}
      <div class="flex flex-row gap-1 w-full mb-1">
        <input
          list="opciones-platillos"
          bind:value={fila.nombre}
          onchange={() => alConfirmarFila(fila)}
          class="w-full border rounded px-3 py-2"
        />
        <input
          type="number"
          bind:value={fila.cantidad}
          min="1"
          class="w-14 border rounded px-1 text-center [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
        />
        {#if filas.length > 1}
          <button
            onclick={() => eliminarFila(fila.id)}
            class="border px-2 rounded cursor-pointer text-red-600 hover:bg-red-50 dark:hover:bg-red-500/10"
          >
            ×
          </button>
        {/if}
      </div>
    {/each}

    <datalist id="opciones-platillos">
      {#each opcionesPlatillo as opcion}
        <option value={opcion}></option>
      {/each}
    </datalist>

    <button
      onclick={agregarFila}
      class="border px-1.5 rounded cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 mt-1"
    >
      +
    </button>

    <p class="text-xl font-bold mt-3">Tipo de orden</p>

    <label>
      <input type="radio" name="tipoOrden" value="Local" bind:group={tipoPedido} />
      Local
    </label>

    <label>
      <input type="radio" name="tipoOrden" value="Domicilio" bind:group={tipoPedido} />
      Domicilio
    </label>

    <label>
      <input type="radio" name="tipoOrden" value="Recoger" bind:group={tipoPedido} />
      Recoger
    </label>

    {#if tipoPedido === "Local"}
      <div>
        <p class="text-center mb-1 text-xl font-bold">Piso y mesa</p>
        <p class="text-center">Piso</p>
        <div class="flex flex-row w-fit">
          <input
            bind:value={piso}
            class="border rounded-l px-2 py-1 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            type="number"
          />
          <div class="flex flex-col">
            <button
              onclick={incrementarPiso}
              class="border rounded-tr cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex-1"
            >
              +
            </button>
            <button
              onclick={decrementarPiso}
              class="border rounded-br cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex-1"
            >
              -
            </button>
          </div>
        </div>

        <p class="text-center">Mesa</p>
        <div class="flex flex-row w-fit">
          <input
            bind:value={mesa}
            class="border rounded-l px-2 py-1 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            type="number"
          />
          <div class="flex flex-col">
            <button
              onclick={incrementarMesa}
              class="border rounded-tr cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex-1"
            >
              +
            </button>
            <button
              onclick={decrementarMesa}
              class="border rounded-br cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex-1"
            >
              -
            </button>
          </div>
        </div>

        <p class="text-center">Mesero</p>
        <input
          list="opciones-meseros"
          bind:value={meseroSeleccionado}
          onchange={alConfirmarMesero}
          class="w-full border rounded px-3 py-2"
        />
        <datalist id="opciones-meseros">
          {#each meseros as opcion}
            <option value={opcion.nombre}></option>
          {/each}
        </datalist>
      </div>
    {/if}

    <p class="text-center mb-1 text-xl font-bold">Información pedido</p>
    <p class="text-center">Nombre del cliente</p>
    <input class="border rounded w-full px-2 py-1" type="text" bind:value={nombreCliente} />
    <p class="text-center">Nota</p>
    <input bind:value={notas} class="border rounded w-full px-2 py-1" type="text" />

    {#if tipoPedido == "Domicilio"}
      <div class="w-full">
        <p class="text-center">Colonia</p>
        <input class="border rounded w-full px-2 py-1" type="text" bind:value={colonia} />
        <p class="text-center">Calle</p>
        <input class="border rounded w-full px-2 py-1" type="text" bind:value={calle} />
        <p class="text-center">Número interior/exterior</p>
        <input
          bind:value={numeroExteriorInterior}
          class="border rounded w-full px-2 py-1 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
          type="number"
        />
        <p class="text-center">Número celular</p>
        <input
          bind:value={numeroCelular}
          class="border rounded w-full px-2 py-1"
          type="tel"
          oninput={(e) => (telefono = sanitizarInputTelefono(e))}
        />
        <p class="text-center">Repartidor</p>
        <input class="border rounded w-full px-2 py-1" type="text" bind:value={repartidor} />
      </div>
    {:else if tipoPedido == "Recoger"}
      <div class="w-full"></div>
    {/if}

    <button
      onclick={onRegistrarOrden}
      class="border mt-2.5 mb-5 rounded p-1.5 cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700"
    >
      Registrar
    </button>
  </div>
</main>