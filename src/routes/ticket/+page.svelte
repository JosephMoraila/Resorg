<script lang="ts">
    import Canvas from "./Canvas.svelte";
    import EscPos from "./EscPos.svelte";
    import type { CanvasElement, TextCanvasElement,ImageCanvasElement, InfoCanvasElement } from "$lib/types";
    import { sanitizeNonNegativeInput } from "$lib/utils/input_utils";
    import { toast } from "$lib/toast.svelte";
    import { convertirBlobUrlABase64, convertirBase64ABlobUrl } from "$lib/utils/file_utils";
    import { invoke } from "@tauri-apps/api/core";
    import { prepararStringHtmlParaExportar } from "$lib/utils/escpos_utils";
    import { onMount } from "svelte";

    let elementsCanvas: CanvasElement[] = $state([]);
    let idElement = $state(1);
    let selectedCanvasElementText = $state<CanvasElement | null>(null);
    let selectedCanvasElementImage = $state<CanvasElement | null>(null);
    let selectedCanvasElementInfo = $state<CanvasElement | null>(null);
    function onAumentarSizeTexto() {
        if (selectedCanvasElementText?.element.tipo === "Texto") {
            selectedCanvasElementText.element.size += 1;
        } else if (selectedCanvasElementInfo?.element.tipo === "Info") {
            selectedCanvasElementInfo.element.size += 1;
        }
    }

    function onDisminuirSizeTexto() {
        if (selectedCanvasElementText?.element.tipo === "Texto" && selectedCanvasElementText.element.size > 0) {
            selectedCanvasElementText.element.size -= 1;
        } else if (selectedCanvasElementInfo?.element.tipo === "Info" && selectedCanvasElementInfo.element.size > 0) {
            selectedCanvasElementInfo.element.size -= 1;
        }
    }

    function handleSizeTextInput(e: Event) {
        const newSize = sanitizeNonNegativeInput(e);
        if (selectedCanvasElementText?.element.tipo === "Texto") {
            selectedCanvasElementText.element.size = newSize;
        } else if (selectedCanvasElementInfo?.element.tipo === "Info") {
            selectedCanvasElementInfo.element.size = newSize;
        }
    }

    let isCanvas = $state(true);
    function onChangeMode(){
        isCanvas = !isCanvas;
        selectedCanvasElementText = null;
        selectedCanvasElementImage = null;
        selectedCanvasElementInfo = null;
    }

    let width = $state(58); // Inicial de 58 mm
    let height = $state(100);
    let textoSize = $derived.by(() => {
        // Si hay un texto seleccionado
        if (selectedCanvasElementText && selectedCanvasElementText.element.tipo === "Texto") {
            return selectedCanvasElementText.element.size;
        }
        
        // Si hay un info seleccionado
        if (selectedCanvasElementInfo && selectedCanvasElementInfo.element.tipo === "Info") {
            return selectedCanvasElementInfo.element.size;
        }

        // Si ninguno está seleccionado (o no cumplen el tipo esperado)
        return 0;
    });
    let isDisabledTextSizeInput = $derived(!(selectedCanvasElementText?.element.tipo === "Texto" || selectedCanvasElementInfo?.element.tipo === "Info"));
    
    let isDisabledUnderInfo = $derived(!(selectedCanvasElementText && selectedCanvasElementText.element.tipo === "Texto"));
    let valueIsUnderInfo = $derived(selectedCanvasElementText && selectedCanvasElementText.element.tipo == "Texto" ? selectedCanvasElementText.element.is_under_info : false);
    function onChangeCheckUnderInfo(event: Event){
        if (selectedCanvasElementText && selectedCanvasElementText.element.tipo === "Texto") {
            const checkbox = event.target as HTMLInputElement;
            const isChecked = checkbox.checked;
            selectedCanvasElementText.element.is_under_info = isChecked;
        }
    }

    function onAgregarTexto(){
        if(isCanvas){
            const newTextElement: TextCanvasElement = {tipo: "Texto", texto: "Ingresa\ntexto", size: 16, is_under_info: false};
            const newElement: CanvasElement = {x: 10, y: 10, element: newTextElement, id: idElement};
            idElement += 1;
            elementsCanvas.push(newElement);
        }
    }

    let inputArchivo = $state<HTMLInputElement>();
    function abrirBuscadorArchivos() {
        // Simulamos el clic en el input de verdad
        inputArchivo?.click();
    }
    function procesarArchivo(archivo: File) {
        if (!archivo.type.startsWith('image/')) return;
        const imgUrl = URL.createObjectURL(archivo);
        const imageClass = new Image();
        
        //Definimos qué hacer cuando la imagen termine de cargar
        imageClass.onload = () => {
            const anchoImagen = imageClass.naturalWidth;
            const altoImagen = imageClass.naturalHeight;
            
            if (isCanvas) {
                const newImage: ImageCanvasElement = {alto: altoImagen, ancho: anchoImagen, src: imgUrl, tipo: "Imagen" };
                const newElement: CanvasElement = {id: idElement, x: 10, y: 10, element: newImage };
                idElement += 1;
                
                elementsCanvas = [...elementsCanvas, newElement];
            }else{
                const seleccion = window.getSelection();
                if (seleccion && seleccion.rangeCount > 0) {
                    const rango = seleccion.getRangeAt(0);
                    const img = document.createElement("img");
                    img.src = imgUrl;
                    img.className = "inline-block max-w-[100px] h-auto align-middle mx-1";
                    rango.deleteContents();
                    // 5. Insertar la imagen en esa posición
                    rango.insertNode(img);

                    // 6. Mover el cursor justo DESPUÉS de la imagen para que el usuario pueda seguir escribiendo
                    rango.setStartAfter(img);
                    rango.setEndAfter(img);
                    seleccion.removeAllRanges();
                    seleccion.addRange(rango);
                }
            }
        };

        // Asignar la ruta para que la imagen empiece a cargar
        imageClass.src = imgUrl;
    }
    function onAgregarImagen(e: Event){
        const input = e.target as HTMLInputElement;
        
        if (input.files && input.files[0]) {
            procesarArchivo(input.files[0]);
            input.value = ""; //Limpiar para tomar otra vez la imagen
        }
    }

    let infoExample = "Total: 500.00 Local\nMétodo de pago: Efectivo\nPagado: 700.00\nCambio: 200.00\nNombre: José\n07 de septiembre de 2026, 10:34:23\nMesero: Pepe\nPizza - 250\nNuggets - 250";
    function onAgregarInfo(){
        if(isCanvas){
            if(elementsCanvas.some(el=>el.element.tipo == "Info")){
                toast.amarillo(`Ya hay información agregada`);
                return;
            }
            const newInfoElement: InfoCanvasElement = {size: 16, texto: infoExample, tipo: "Info"};
            const newElement: CanvasElement = {element: newInfoElement, id: idElement, x: 10, y: 10};
            idElement += 1;
            elementsCanvas.push(newElement);
        }else{
            // 1. Validar si ya existe la información en el modo EscPos
            const indexStart = contenidoEscPos.indexOf("[INFO_START]");
            const indexEnd = contenidoEscPos.indexOf("[INFO_END]");

            // Si ambos existen (-1 significa que no se encontró) y el START está antes que el END
            if (indexStart !== -1 && indexEnd !== -1 && indexStart < indexEnd) {
                toast.amarillo(`Ya hay información agregada`);
                return;
            }

            // 2. Continuar con la inserción normal
            const seleccion = window.getSelection();
            if (seleccion && seleccion.rangeCount > 0) {
                const rango = seleccion.getRangeAt(0);

                const infoHTML = `
                    <div>[INFO_START]</div>
                    <div>Total: 500.00 Local</div>
                    <div>Método de pago: Efectivo</div>
                    <div>Pagado: 700.00</div>
                    <div>Cambio: 200.00</div>
                    <div>Nombre: José</div>
                    <div>07 de septiembre de 2026, 10:34:23</div>
                    <div>Mesero: Pepe</div>
                    <div>Pizza - 250</div>
                    <div>Nuggets - 250</div>
                    <div>[INFO_END]</div>
                `;

                const fragmento = rango.createContextualFragment(infoHTML);
                const ultimoNodo = fragmento.lastChild;

                let lineaActual = rango.startContainer as HTMLElement;
                while (lineaActual && lineaActual.tagName !== "DIV" && lineaActual.contentEditable !== "true") {
                    lineaActual = lineaActual.parentElement as HTMLElement;
                }

                if (lineaActual && lineaActual.tagName === "DIV") {
                    if (lineaActual.textContent?.trim() === "" && !lineaActual.querySelector('img')) {
                        lineaActual.replaceWith(fragmento);
                    } else {
                        lineaActual.after(fragmento);
                    }
                } else {
                    rango.deleteContents();
                    rango.insertNode(fragmento);
                }

                if (ultimoNodo) {
                    const nuevoRango = document.createRange();
                    nuevoRango.setStartAfter(ultimoNodo);
                    nuevoRango.collapse(true);
                    seleccion.removeAllRanges();
                    seleccion.addRange(nuevoRango);
                }

                const editor = document.querySelector('[contenteditable="true"]');
                if (editor) editor.dispatchEvent(new Event("input", { bubbles: true }));
            }
        }
    }

    //EscPos
    //Inicializar con un div que contiene un <br> (así es como el navegador entiende un renglón vacío)
    let contenidoEscPos = $state("<div><br></div>");

    async function onSave(){
        console.log(elementsCanvas);
        let canvasElementsToBackend: CanvasElement[] = [];
        try{
            for(let ec of elementsCanvas){
                if(ec.element.tipo == "Imagen"){
                    const base64Image = await convertirBlobUrlABase64(ec.element.src);
                    const convertedImageElement: ImageCanvasElement = {alto: ec.element.alto, ancho: ec.element.ancho, src: base64Image, tipo: "Imagen"};
                    const convertedCanvasElement: CanvasElement = {element: convertedImageElement, id: ec.id, x: ec.x, y: ec.y};
                    canvasElementsToBackend.push(convertedCanvasElement);
                }else if(ec.element.tipo == "Info"){
                    const convertedInfoElement: InfoCanvasElement = {size: ec.element.size, texto: ec.element.texto, tipo: "Info"};
                    const convertedCanvasElement: CanvasElement = {element: convertedInfoElement, id: ec.id, x: ec.x, y: ec.y};
                    canvasElementsToBackend.push(convertedCanvasElement);
                }else if(ec.element.tipo == "Texto"){
                    const convertedTextoElement: TextCanvasElement = {is_under_info: ec.element.is_under_info, size: ec.element.size, texto: ec.element.texto, tipo: "Texto"};
                    const convertedCanvasElement: CanvasElement = {element: convertedTextoElement, id: ec.id, x: ec.x, y: ec.y};
                    canvasElementsToBackend.push(convertedCanvasElement);
                }
            }
        }catch(error){
            toast.rojo(`Error al convertir imagenes de canvas: ${error}`);
            return;
        }

        //Guardar información del ticket
        try{
            let paramsTicket = {width, height, isCanvas};
            invoke("save_ticket_measurement", paramsTicket);
            toast.verde(`Información del ticket guardada correctamente`);
        }catch(error){
            const err = error as string;
            toast.rojo(`Error guardando información del ticket: ${err}`);
        }

        //Guardar canvas:
        try{
            let paramsCanvas = {canvasElements: canvasElementsToBackend};
            invoke("save_canvas", paramsCanvas);
            toast.verde(`Elementos de canvas guardados`);
        }catch(error){
            const err = error as string;
            toast.rojo(`Error guardando canvas: ${err}`);
        }

        //Guardar escpos html:
        try{
            const htmlConvertidoImagenes = await prepararStringHtmlParaExportar(contenidoEscPos);
            let paramsEscPos = {contenidoHtml: htmlConvertidoImagenes};
            invoke("save_escpos_html", paramsEscPos);
            toast.verde(`Elementos de escpos guardados`);
        }catch(error){
            const err = error as string;
            toast.rojo(`Error guardando escpos: ${err}`);
        }
    }

    onMount(async ()=>{
        try{
            const [anchoBackend, altoBackend, isCanvasBackend] = await invoke<[number, number, boolean]>("get_ticket_measurement");
            width = anchoBackend;
            height = altoBackend;
            isCanvas = isCanvasBackend;
        }catch(error){
            const err = error as string;
            toast.rojo(`Error cargando información del ticket: ${err}`);
        }

        try{
            let canvasElementsBackend = await invoke<CanvasElement[]>("get_canvas");
            let idMasAlto = 0; //Por si el usuario quiere agregar más
            for(let element of canvasElementsBackend){
                if(element.id > idMasAlto){
                    idMasAlto = element.id;
                }
                if(element.element.tipo == "Imagen"){
                    element.element.src = convertirBase64ABlobUrl(element.element.src);
                }
            }
            idMasAlto += 1; //Porque el ultimo ID ya existe le ponemos más uno para que el proximo se cree con ese ID
            idElement = idMasAlto;
            elementsCanvas = canvasElementsBackend;
        }catch(error){
            const err = error as string;
            toast.rojo(`Error cargando información de canvas: ${err}`);
        }

        try{
            const htmlEscposBackend = await invoke<string>("get_escpos_html");
            contenidoEscPos = htmlEscposBackend;
        }catch(error){
            const err = error as string;
            toast.rojo(`Error cargando información de escpos: ${err}`);
        }
    });

    function onProbar(){
        try{
            invoke("print_prueba");
        }catch(error){
            const err = error as string;
            toast.rojo(`Error al imprimir prueba: ${err}`);
        }
    }

</script>

<main class="w-full h-screen flex flex-col bg-white dark:bg-black text-black dark:text-white overflow-x-hidden">

    <div class="flex flex-row flex-wrap items-center w-full py-3 bg-gray-200 dark:bg-gray-900 gap-4 px-4">
        <button onclick={onChangeMode} class="btn-realista">{isCanvas ? "Canvas" : "EscPos"}</button>

        <!-- Ancho -->
        <div class="flex flex-row items-center gap-0 whitespace-nowrap">
            <span class="mr-2">Ancho (mm):</span>
            <div class="flex items-center">
                <input
                  type="number" bind:value={width}
                  class="border w-20 h-8 rounded-l px-2 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                />
                <button class="border h-8 cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={()=>width=width+1}>+</button>
                <button class="border h-8 rounded-r cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={()=>width=width-1}>-</button>
            </div>
        </div>

        <!-- Alto -->
        <div class="flex flex-row items-center gap-0 whitespace-nowrap">
            <span class="mr-2">Alto (mm):</span>
            <div class="flex items-center">
                <input
                  type="number" bind:value={height}
                  class="border w-20 h-8 rounded-l px-2 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                />
                <button class="border h-8 cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={()=>height=height+1}>+</button>
                <button class="border h-8 rounded-r cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={()=>height=height-1}>-</button>
            </div>
        </div>

        <button class="btn-realista" onclick={onAgregarTexto}>Agregar texto</button>

        <!-- Tamaño texto -->
        <div class="flex flex-row items-center gap-0 whitespace-nowrap">
            <span class="mr-2">Tamaño texto:</span>
            <div class="flex items-center">
                <input oninput={handleSizeTextInput}
                  type="number" bind:value={textoSize} disabled={isDisabledTextSizeInput}
                  class="border w-20 h-8 rounded-l px-2 disabled:opacity-50 disabled:cursor-not-allowed [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                />
                <button disabled={isDisabledTextSizeInput} class="border h-8 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={onAumentarSizeTexto}>+</button>
                <button disabled={isDisabledTextSizeInput} class="border h-8 rounded-r cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-300 dark:hover:bg-gray-700 px-2 flex items-center justify-center" onclick={onDisminuirSizeTexto}>-</button>
            </div>
        </div>

        <button class="btn-realista" onclick={abrirBuscadorArchivos}>Agregar imagen</button>
        <input type="file" bind:this={inputArchivo} accept="image/*" class="hidden" onchange={onAgregarImagen}>

        <button class="btn-realista" onclick={onAgregarInfo}>Agregar información</button>

        <span>Abajo de info</span>
        <input bind:checked={valueIsUnderInfo} onchange={onChangeCheckUnderInfo} type="checkbox" title="La info es variable hacia abajo por lo que activar esto en un texto lo pondrá abajo" class="size-5 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed" disabled={isDisabledUnderInfo}>

        <button class="btn-realista" onclick={onSave}>Guardar</button>

        <button class="btn-realista" onclick={onProbar}>Probar</button>

    </div>
    

    <!-- Ajustado p-8 a p-6 y removido mt-2.5 para corregir la posición del canvas -->
    <div class="w-full flex-1 bg-gray-200 dark:bg-gray-900 overflow-auto">
        {#if isCanvas}
            <Canvas width={width} height={height} bind:elementsCanvas={elementsCanvas} bind:selectedCanvasElementText={selectedCanvasElementText} bind:selectedCanvasElementImage={selectedCanvasElementImage} bind:selectedCanvasElementInfo={selectedCanvasElementInfo} bind:idElement={idElement}/>
        {:else}
            <EscPos width={width} height={height} bind:contenidoEscPos={contenidoEscPos}/>
        {/if}
    </div>

</main>