<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { convertFileSrc } from "@tauri-apps/api/core";

    interface Props {
        width: number;
        height: number;
        contenidoEscPos: string;
    }

    let {height, width, contenidoEscPos = $bindable()}: Props = $props();

    const PX_PER_MM = 6.620689655;

    let widthPx = $derived(width * PX_PER_MM);
    let heightPx = $derived(height * PX_PER_MM);

    function getEscPosStyle(wPx: number, hPx: number): string {
        return `width: ${wPx}px; min-width: ${wPx}px; height: ${hPx}px; min-height: ${hPx}px; background-color: white;`;
    }

    let contentEditableEl = $state<HTMLDivElement | null>(null);

    onMount(() => {
        if (contentEditableEl) {
            contentEditableEl.innerHTML = contenidoEscPos; // solo se escribe UNA vez, al montar
        }
    });

    function normalizarLineas() {
        if (!contentEditableEl) return;

        Array.from(contentEditableEl.children).forEach((hijo) => {
            if (
                hijo.tagName === "DIV" &&
                hijo.childNodes.length === 1 &&
                hijo.firstElementChild?.tagName === "DIV"
            ) {
                const divInterno = hijo.firstElementChild;
                hijo.replaceWith(divInterno);
            }
        });
    }

    function sincronizarEstado() {
        if (!contentEditableEl) return;
        contenidoEscPos = contentEditableEl.innerHTML; // solo lectura DOM -> variable, nunca al revés
    }

    function protegerEstructura() {
        if (!contentEditableEl) return;

        if (contentEditableEl.innerHTML === "" || contentEditableEl.innerHTML === "<br>") {
            contentEditableEl.innerHTML = "<div><br></div>";
        } else {
            normalizarLineas();
        }
        sincronizarEstado();
    }

    function encontrarLineaActual(nodo: Node): HTMLElement | null {
        let actual: Node | null = nodo;
        while (actual && actual !== contentEditableEl) {
            if (actual.parentElement === contentEditableEl) {
                return actual as HTMLElement;
            }
            actual = actual.parentNode;
        }
        return null;
    }
    function handleKeyDown(event: KeyboardEvent) {
        if (event.key !== "Enter") return;
        event.preventDefault();

        const seleccion = window.getSelection();
        if (!seleccion || seleccion.rangeCount === 0 || !contentEditableEl) return;

        const rango = seleccion.getRangeAt(0);
        const lineaActual = encontrarLineaActual(rango.startContainer);
        if (!lineaActual) return;

        const rangoResto = document.createRange();
        rangoResto.setStart(rango.endContainer, rango.endOffset);
        rangoResto.setEndAfter(lineaActual.lastChild ?? lineaActual);
        
        // Al extraer, le quitamos todo a la derecha del cursor a la línea actual
        const contenidoRestante = rangoResto.extractContents();

        // SOLUCIÓN: Si la línea de arriba quedó totalmente vacía al extraer el contenido, 
        // le ponemos un <br> para que no pierda su altura y el cursor realmente baje.
        if (lineaActual.innerHTML === "") {
            lineaActual.innerHTML = "<br>";
        }

        const nuevaLinea = document.createElement("div");
        
        // Validamos qué poner en la nueva línea
        if (contenidoRestante.textContent?.trim() === "" && !contenidoRestante.querySelector('img')) {
            nuevaLinea.innerHTML = "<br>";
        } else {
            nuevaLinea.appendChild(contenidoRestante);
        }

        lineaActual.after(nuevaLinea);

        contentEditableEl.focus(); 

        const nuevoRango = document.createRange();
        nuevoRango.setStart(nuevaLinea, 0);
        nuevoRango.collapse(true);
        seleccion.removeAllRanges();
        seleccion.addRange(nuevoRango);

        sincronizarEstado();
    }

    let unlisten: (() => void) | null = null;
    onMount(async () => {
        const currentWebView = getCurrentWebviewWindow();
        
        unlisten = await currentWebView.onDragDropEvent(event => {
            if (event.payload.type === "drop" && event.payload.paths.length > 0) {
                const rutaFisica = event.payload.paths[0];
                const imgUrl = convertFileSrc(rutaFisica);
                
                // Coordenadas globales de la ventana
                const dropX = event.payload.position.x;
                const dropY = event.payload.position.y;
                
                const imageClass = new Image();
                
                imageClass.onload = () => {

                    // MODO ESCPOS: Convertir coordenadas en una posición de texto
                    let rango = null;
                    
                    // Intentar usar el estándar moderno primero
                    if (document.caretPositionFromPoint) {
                        const posicion = document.caretPositionFromPoint(dropX, dropY);
                        if (posicion && posicion.offsetNode) {
                            rango = document.createRange();
                            rango.setStart(posicion.offsetNode, posicion.offset);
                            rango.collapse(true);
                        }
                    } 
                    // Fallback para navegadores/webviews que aún no implementan el nuevo estándar
                    else if (document.caretRangeFromPoint) {
                        rango = document.caretRangeFromPoint(dropX, dropY);
                    }

                    if (rango) {
                        const img = document.createElement("img");
                        img.src = imgUrl;
                        img.className = "inline-block max-w-[100px] h-auto align-middle mx-1";
                        
                        // 1. Borramos contenido si el usuario arrastró hacia texto sombreado
                        rango.deleteContents();
                        
                        // 2. Insertamos la imagen exactamente en la línea calculada
                        rango.insertNode(img);

                        // 3. Movemos el cursor real a la derecha de la imagen insertada
                        rango.setStartAfter(img);
                        rango.setEndAfter(img);
                        
                        const seleccion = window.getSelection();
                        if (seleccion) {
                            seleccion.removeAllRanges();
                            seleccion.addRange(rango);
                        }
                        
                        // 4. Forzamos a EscPos.svelte a actualizarse sincronizando el evento
                        const editor = document.querySelector('[contenteditable="true"]');
                        if (editor) editor.dispatchEvent(new Event("input", { bubbles: true }));
                    }
                };

                imageClass.src = imgUrl;
            }
        });
    });

    onDestroy(() => {
        unlisten?.();
    });
</script>

<div class="inline-block p-6">

    <div
        bind:this={contentEditableEl}
        oninput={protegerEstructura}
        onkeydown={handleKeyDown}
        contenteditable="true"
        role="textbox"
        aria-multiline="true"
        tabindex="0"
        style={getEscPosStyle(widthPx, heightPx)}
        class="text-center text-black shadow-2xl relative select-none overflow-x-auto overflow-y-auto"
    >
    </div>

</div>