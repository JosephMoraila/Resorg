<!-- Canvas.svelte -->
<script lang="ts">
    import type { CanvasElement } from "$lib/types";
    import { getTextElementFromCanvasElement } from "$lib/utils/canvas_utils";
    import { tick } from "svelte";

    interface Props {
        width: number;
        height: number;
        elementsCanvas: CanvasElement[];
        selectedCanvasElementText: CanvasElement | null;
        selectedCanvasElementImage: CanvasElement | null;
        selectedCanvasElementInfo: CanvasElement | null;
    }

    let { width, height, elementsCanvas = $bindable(), selectedCanvasElementText = $bindable(), selectedCanvasElementImage = $bindable(), selectedCanvasElementInfo = $bindable() }: Props =$props();

    // 384px / 58mm = 6.620689655 px/mm
    const PX_PER_MM = 6.620689655;

    let widthPx = $derived(width * PX_PER_MM);
    let heightPx = $derived(height * PX_PER_MM);

    function getCanvasStyle(wPx: number, hPx: number): string {
        return `width: ${wPx}px; min-width: ${wPx}px; height: ${hPx}px; min-height: ${hPx}px; background-color: white;`;
    }

    // Control de arrastre
    let activeId = $state<string | number | null>(null);
    let startX = $state(0);
    let startY = $state(0);
    let initialElemX = $state(0);
    let initialElemY = $state(0);

    function handleMouseDown(event: MouseEvent, element: CanvasElement) {
        event.stopPropagation();
        
        activeId = element.id;
        startX = event.clientX;
        startY = event.clientY;
        initialElemX = element.x;
        initialElemY = element.y;

        window.addEventListener("mousemove", handleMouseMove);
        window.addEventListener("mouseup", handleMouseUp);
    }

    function handleMouseMove(event: MouseEvent) {
        if (activeId === null) return;

        const dx = event.clientX - startX;
        const dy = event.clientY - startY;

        const targetElement = elementsCanvas.find((item) => item.id === activeId);
        if (targetElement) {
            // Calculamos la nueva posición asegurando que no se salga del lienzo
            const newX = initialElemX + dx;
            const newY = initialElemY + dy;

            targetElement.x = Math.max(0, Math.min(newX, widthPx - 10));
            targetElement.y = Math.max(0, Math.min(newY, heightPx - 10));
        }
    }

    function handleMouseUp() {
        activeId = null;
        window.removeEventListener("mousemove", handleMouseMove);
        window.removeEventListener("mouseup", handleMouseUp);
    }

    function manejarTeclado(event: KeyboardEvent) {
        if (event.key === "Escape") {
            activeDoubleClick = null;
            textToEdit = "";
            textArea?.blur();
            selectedCanvasElementText = null;
            selectedCanvasElementImage = null;
            selectedCanvasElementInfo = null; // <-- Solución al problema del Escape
        } else if (event.key === "Delete") {
            if (selectedCanvasElementText !== null) {
                const target = selectedCanvasElementText;
                elementsCanvas = elementsCanvas.filter(el => el.id !== target.id);
                selectedCanvasElementText = null;
            } else if (selectedCanvasElementImage !== null) {
                const target = selectedCanvasElementImage;
                elementsCanvas = elementsCanvas.filter(el => el.id !== target.id);
                selectedCanvasElementImage = null;
            } else if (selectedCanvasElementInfo !== null) {
                // <-- Agregado para que también puedas borrar los elementos Info
                const target = selectedCanvasElementInfo;
                elementsCanvas = elementsCanvas.filter(el => el.id !== target.id);
                selectedCanvasElementInfo = null;
            }
        }
    }

    //Double clock
    let textArea = $state<HTMLTextAreaElement | null>(null);
    let activeDoubleClick = $state<string | number | null>(null);
    let textToEdit = $state("");
    async function handleDoubleClickText(event: MouseEvent, element: CanvasElement){
        if(element.element.tipo != "Texto") return;
        activeDoubleClick = element.id;
        textToEdit = element.element.texto;
        selectedCanvasElementText = element;
        selectedCanvasElementImage = null;
        selectedCanvasElementInfo = null;
        textArea?.focus();

        await tick(); //Como textArea aqui se esperó que se dibuje en el DOM abajo ya se puede trabajar con él
        
        if (textArea) {
            adjustTextAreaSize(); // Ajustar tamaño inicial al abrir
            textArea.focus();
            textArea.select(); // Opcional: selecciona todo el texto al hacer doble clic
        }
    }
    // Acción para dar foco automáticamente al montarse
    function autoFocus(node: HTMLTextAreaElement) {
        node.focus();
    }
    function adjustTextAreaSize() {
        if (!textArea) return;

        // Reseteamos dimensiones para forzar la reevaluación completa
        textArea.style.width = "auto";
        textArea.style.height = "auto";

        // Asignamos el ancho del contenido completo (+20px de margen de seguridad para el cursor)
        textArea.style.width = `${textArea.scrollWidth + 20}px`;
        textArea.style.height = `${textArea.scrollHeight}px`;
    }
    $effect(() => {
        if (activeDoubleClick && selectedCanvasElementText && selectedCanvasElementText.element.tipo == "Texto") {
            selectedCanvasElementText.element.texto = textToEdit;
            adjustTextAreaSize();
        }
    });

    function handleClickImage(event: MouseEvent, element: CanvasElement){
        if(element.element.tipo != "Imagen") return;
        selectedCanvasElementImage = element;
        selectedCanvasElementText = null;
        activeDoubleClick = null;
        selectedCanvasElementInfo = null;
        
    }

    let startXreziseImage = $state(0);
    let startYreziseImage = $state(0);
    let initialElemWidthReziseImage = $state(0);
    let initialElemHeightReziseImage = $state(0);
    function handleMouseDownResizeImage(event: MouseEvent){
        event.stopPropagation();
        if(selectedCanvasElementImage === null || selectedCanvasElementImage.element.tipo != "Imagen") return; //Tiene que haber seleccionado una imagen primero
        startXreziseImage = event.clientX;
        startYreziseImage = event.clientY;
        initialElemHeightReziseImage = selectedCanvasElementImage.element.alto;
        initialElemWidthReziseImage = selectedCanvasElementImage.element.ancho
        window.addEventListener("mousemove", handleMouseMoveResizeImage);
        window.addEventListener("mouseup", handleMouseUpResizeImage);
    }
    function handleMouseMoveResizeImage(event: MouseEvent){
        if(selectedCanvasElementImage === null || selectedCanvasElementImage.element.tipo != "Imagen") return;

        const dx = event.clientX - startXreziseImage; //Si el mouse está en 20 y se hizo presión en 15 entonces dx es 5
        const dy = event.clientY - startYreziseImage;


        let newWith = initialElemWidthReziseImage + dx; //Si el ancho inicial era de 10 + 5 = 15
        let newHeight = initialElemHeightReziseImage + dy;

        if(newWith < 50) newWith = 50; //Una imagen no puede tener medidas negativas asi que si es menos que 50 ponerle 50 como ancho minimo
        if(newHeight < 50) newHeight = 50;

        selectedCanvasElementImage.element.alto = newHeight;
        selectedCanvasElementImage.element.ancho = newWith;
    }
    function handleMouseUpResizeImage(){
        window.removeEventListener("mousemove", handleMouseMoveResizeImage);
        window.removeEventListener("mouseup", handleMouseUpResizeImage);
    }

    function handleClickInfo(element: CanvasElement){
        if(element.element.tipo != "Info") return;
        selectedCanvasElementInfo = element;
        selectedCanvasElementImage = null;
        selectedCanvasElementText = null;
    }

</script>

<svelte:window onkeydown={manejarTeclado} />

<div class="inline-block p-6">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div style={getCanvasStyle(widthPx, heightPx)} class="shadow-2xl relative select-none overflow-x-auto overflow-y-hidden">
        {#each elementsCanvas as element (element.id)}
            {#if element.element.tipo == "Texto"}
                <span 
                    role="button"
                    tabindex="0"
                    onmousedown={(e) => handleMouseDown(e, element)}
                    ondblclick={e=>handleDoubleClickText(e, element)}
                    class="text-black absolute whitespace-pre hover:cursor-grab active:cursor-grabbing border border-transparent hover:border-dashed hover:border-blue-400 p-0.5 rounded transition-colors {activeId === element.id ? 'border-blue-500 z-20' : 'z-10'}"
                    style={`left: ${element.x}px; top: ${element.y}px; font-size: ${element.element.size}px;`}
                >
                    {getTextElementFromCanvasElement(element)}
                </span>
            {:else if element.element.tipo == "Imagen"}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->      
                <img 
                    src={element.element.src} 
                    draggable="false" 
                    onmousedown={(e) => handleMouseDown(e, element)} 
                    alt=""
                    class="absolute max-w-none hover:cursor-grab active:cursor-grabbing border hover:border-solid hover:border-blue-400 p-0.5 rounded transition-colors {selectedCanvasElementImage?.id === element.id ? 'border-solid border-blue-500 z-20' : 'border-transparent z-10'}" 
                    style={`left: ${element.x}px; top: ${element.y}px; height: ${element.element.alto}px; width: ${element.element.ancho}px;`}
                    onclick={e => handleClickImage(e, element)}
                >
                {#if selectedCanvasElementImage && selectedCanvasElementImage.element.tipo == "Imagen" && element.id === selectedCanvasElementImage.id}
                    <div style={`left: ${element.element.ancho + element.x}px; top: ${element.element.alto + element.y}px;`} onmousedown={e=>handleMouseDownResizeImage(e)} class="absolute max-w-none w-2 h-2 border-2 border-blue-500 bg-transparent z-20 cursor-nwse-resize"></div>
                {/if}
            {:else if element.element.tipo == "Info"}
                <span role="button" tabindex="0" class="text-black absolute whitespace-pre hover:cursor-grab active:cursor-grabbing border hover:border-dashed hover:border-blue-400 p-0.5 rounded transition-colors {selectedCanvasElementInfo?.id === element.id ? 'border-solid border-red-500 z-20' : 'border-transparent z-10'}"
                    style={`left: ${element.x}px; top: ${element.y}px; font-size: ${element.element.size}px;`} onmousedown={(e) => handleMouseDown(e, element)} onclick={e=>handleClickInfo(element)}
                >
                    {getTextElementFromCanvasElement(element)}
                </span>
            {/if}
        {/each}
        {#if activeDoubleClick && selectedCanvasElementText && selectedCanvasElementText.element.tipo == "Texto"}
            <textarea style={`left: ${selectedCanvasElementText.x+3}px; top: ${selectedCanvasElementText.y+3}px; font-size: ${selectedCanvasElementText.element.size}px;`} bind:value={textToEdit} name="" id="" 
                class="absolute text-black bg-transparent border-none outline-none resize-none overflow-visible whitespace-pre p-0 m-0 z-30" bind:this={textArea} onfocus={()=>console.log("Tiene foco")} onblur={()=>console.log("No tiene foco")}
                use:autoFocus oninput={adjustTextAreaSize}
            ></textarea>
        {/if}
    </div>
</div>