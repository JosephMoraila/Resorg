import type { CanvasElement } from "$lib/types";

export function getTextElementFromCanvasElement(element: CanvasElement){
    let texto = "";
    if(element.element.tipo == "Texto"){
        texto = element.element.texto;
    }else if(element.element.tipo == "Info") texto = element.element.texto;
    return texto;
}