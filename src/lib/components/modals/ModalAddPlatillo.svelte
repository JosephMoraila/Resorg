<script lang="ts">

    import { fade, fly } from "svelte/transition";
    import { cubicOut } from "svelte/easing";
    import { formatearMonedaInput, stringANumero, formatearMoneda } from "$lib/utils/string_utils";
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { readFile } from "@tauri-apps/plugin-fs";
    import { onMount, onDestroy } from "svelte";
    import { toast } from "$lib/toast.svelte";


    interface ValorInicialPlatillo {
        nombre: string;
        descripcion: string | null;
        precio: number;
        imagenUrl: string | null;
    }

    interface Props{
        abierto: boolean;
        accion?: "crear" | "editar";
        valorInicial?: ValorInicialPlatillo | null;
        onCancelar?: () => void;
        onAceptar: (datos: {nombre: string, descripcion: string | null, precio: number, imagen: string | null} | null) => void;
    }

    let {
        abierto = $bindable(),
        accion = "crear",
        valorInicial = null,
        onCancelar,
        onAceptar

    }:Props = $props();

    let nombre = $state("");
    let descripcion = $state("");
    let precio = $state('');
    let inputArchivo = $state<HTMLInputElement>();

    // Guardamos la URL temporal de la imagen
    let previewUrl = $state<string | null>(null);
    let esArrastrando = $state(false);

    // Para no pisar la imagen si el usuario ya la cambió/quitó manualmente
    let imagenTocada = $state(false);
    let previoAbierto = false;

    $effect(() => {
        if (abierto && !previoAbierto) {
            // Se acaba de abrir el modal: inicializa el formulario
            imagenTocada = false;
            if (accion === "editar" && valorInicial) {
                nombre = valorInicial.nombre;
                descripcion = valorInicial.descripcion ?? "";
                precio = formatearMoneda(valorInicial.precio);
                previewUrl = valorInicial.imagenUrl;
            } else {
                nombre = "";
                descripcion = "";
                precio = "";
                previewUrl = null;
            }
        } else if (abierto && accion === "editar" && !imagenTocada) {
            // La imagen del platillo puede llegar después (se carga de forma asíncrona)
            if (valorInicial?.imagenUrl && previewUrl !== valorInicial.imagenUrl) {
                previewUrl = valorInicial.imagenUrl;
            }
        }
        previoAbierto = abierto;
    });

    let unlisten: (() => void) | null = null;

    onMount(async () => {
        // Escucha nativa de Tauri: los eventos ondragover/ondrop del DOM
        // no se disparan para drops de archivos del sistema operativo.
        unlisten = await getCurrentWebviewWindow().onDragDropEvent((event) => {
            if (event.payload.type === 'over') {
                esArrastrando = true;
            } else if (event.payload.type === 'leave') {
                esArrastrando = false;
            } else if (event.payload.type === 'drop') {
                esArrastrando = false;
                const ruta = event.payload.paths[0];
                if (ruta) procesarArchivoDesdeRuta(ruta);
            }
        });
    });

    onDestroy(() => {
        unlisten?.();
    });

    function cancelar() {
        nombre = "";
        descripcion = "";
        precio = "";
        onAceptar(null); //Se envía null para que no se haga la inserción
        eliminarImagen();
        abierto = false;
        onCancelar?.();
    }

    function manejarTeclado(event: KeyboardEvent) {
        if (event.key === "Escape") cancelar();
        //if (event.key === "Enter") confirmar();
    }

    function autofocus(node: HTMLInputElement) {
        node.focus();
    }

    // Función para redimensionar el textarea según su contenido
    function autoExpandir(e: Event) {
        const target = e.target as HTMLTextAreaElement;
        // Reseteamos primero la altura
        target.style.height = "auto";
        
        // Si no ha superado el scrollHeight real, expande hasta el límite max-h definido en CSS
        target.style.height = `${target.scrollHeight}px`;
    }

    function manejarInput(e: Event, actualizarEstado: (v: string) => void) {
        const input = e.target as HTMLInputElement;
        const valorFormateado = formatearMonedaInput(input.value);
        
        // Actualizamos tanto el valor HTML como la variable reactiva
        input.value = valorFormateado;
        actualizarEstado(valorFormateado);
    }

    function abrirBuscadorArchivos() {
        // Simulamos el clic en el input de verdad
        inputArchivo?.click();
    }

    function procesarArchivo(archivo: File) {
        if (!archivo.type.startsWith('image/')) return;

        if (previewUrl) {
            URL.revokeObjectURL(previewUrl);
        }
        imagenTocada = true;
        previewUrl = URL.createObjectURL(archivo);
    }

    // Válido solo por extensión, ya que desde onDragDropEvent no tenemos File.type
    function esImagenPorExtension(ruta: string) {
        return /\.(png|jpe?g|gif|webp|bmp)$/i.test(ruta);
    }

    async function procesarArchivoDesdeRuta(ruta: string) {
        if (!esImagenPorExtension(ruta)) return;

        const bytes = await readFile(ruta); // Uint8Array
        const blob = new Blob([bytes]);

        if (previewUrl) {
            URL.revokeObjectURL(previewUrl);
        }
        imagenTocada = true;
        previewUrl = URL.createObjectURL(blob);
    }

    function alSeleccionarArchivo(e: Event) {
        const input = e.target as HTMLInputElement;
        
        if (input.files && input.files[0]) {
            procesarArchivo(input.files[0]);
        }
    }

    function eliminarImagen() {
        imagenTocada = true;
        if (previewUrl) {
            URL.revokeObjectURL(previewUrl);
            previewUrl = null;
        }
        if (inputArchivo) {
            inputArchivo.value = '';
        }
    }

    function confirmar(){
        const nombreTrim = nombre.trim();
        const descripcionTrim = descripcion.trim();
        //Verificar que los cuatro ingresos no estén vacíos, si lo están, se envía null para que no se haga la inserción
        if(nombreTrim === "" && precio.trim() === "" && previewUrl === null && descripcionTrim === ""){
            onAceptar(null); //No hace falta hacer los valores como nombre vacios porque ya se validó que no se puede enviar un platillo sin nombre, precio o imagen
        }
        //Aqui se supone que al menos un input tiene valor, entonces verificar cada uno de los obligatorios y mandar mensaje de que falta
        if(nombreTrim === ""){
            toast.amarillo("El nombre del platillo no puede estar vacío");
            return;
        }
        if(precio.trim() === ""){
            toast.amarillo("El precio del platillo no puede estar vacío");
            return;
        }
        const precioNum = stringANumero(precio);
        const descripcionFinal = descripcionTrim === "" ? null : descripcionTrim;

        const datos = {nombre: nombreTrim, descripcion: descripcionFinal, precio: precioNum, imagen: previewUrl};
        nombre = "";
        descripcion = "";
        precio = "";
        onAceptar(datos);
    }

</script>

{#if abierto}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div transition:fade={{ duration: 150 }}
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
        onclick={cancelar}
        onkeydown={manejarTeclado}
    >

        <div transition:fly={{ y: -15, duration: 200, easing: cubicOut }}
        class="bg-white dark:bg-[#1a1f2e] rounded-lg shadow-xl p-6 w-full max-w-sm mx-4"
        onclick={(e) => e.stopPropagation()}
        >

            <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
                {accion === "editar" ? "Editar platillo" : "Agregar platillo"}
            </h2>

            <label for="nombre" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">
                Nombre
            </label>

            <input id="nombre"
                use:autofocus
                bind:value={nombre}
                placeholder="Ej. Chilaquiles"
                class="w-full mb-4 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500"
                onkeydown={manejarTeclado}
            />

            <label for="descripcion" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">
                Descripción (opcional)
            </label>

            <textarea
                id="descripcion"
                bind:value={descripcion}
                placeholder="Ej. Con pollo y queso"
                class="w-full mb-4 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500 resize-none max-h-[120px] overflow-y-auto"
                rows="1"
                oninput={autoExpandir}
                onkeydown={manejarTeclado}
            ></textarea>

            <label for="precio" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">
                Precio
            </label>

            <input
                id="precio"
                type="text"
                inputmode="decimal"
                placeholder="0.00"
                bind:value={precio}
                oninput={(e) => manejarInput(e, (v) => (precio = v))}
                class="w-full mb-4 px-3 py-2 border border-gray-300 dark:border-white/20 rounded-md bg-transparent text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-teal-500"
            />

            <label for="foto" class="text-sm text-gray-500 dark:text-gray-400 mt-1 block">
                Foto (opcional)
            </label>

            <input type="file"
                bind:this={inputArchivo}
                accept="image/*"
                class="hidden"
                onchange={alSeleccionarArchivo}
            >

            {#if previewUrl}
                <div class="relative mt-1 group w-full h-40 border border-gray-300 dark:border-white/20 rounded-lg overflow-hidden">
                    <img src={previewUrl} alt="Vista previa" class="w-full h-full object-contain">
                    <button
                        type="button"
                        onclick={eliminarImagen}
                        class="absolute top-2 right-2 bg-red-600/80 hover:bg-red-600 text-white p-1 rounded-full text-xs transition-colors cursor-pointer"
                        title="Quitar foto"
                    >
                        X
                    </button>
                </div>
            {:else}
                <div 
                    onclick={abrirBuscadorArchivos} 
                    class="cursor-pointer transition-colors justify-center items-center p-4 border border-dashed rounded-lg text-center select-none mt-1
                    {esArrastrando 
                        ? 'border-teal-500 bg-teal-500/10 text-teal-600 dark:text-teal-400' 
                        : 'border-gray-300 dark:border-white/20 hover:bg-gray-100 dark:hover:bg-gray-800 bg-gray-50 dark:bg-zinc-800 text-gray-900 dark:text-white'}"
                >
                    <p class="text-sm font-medium">
                        {esArrastrando ? '¡Suelta la foto aquí!' : 'Haz clic o arrastra una imagen aquí'}
                    </p>
                </div>
            {/if}

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