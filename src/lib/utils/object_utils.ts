import type {PedidoPlatillo, Pedido, TipoPedido } from "$lib/types";

/**
 * Checa si hay al menos un platillo que aún existe en DB(backend retorna ese campo con valor y no es null) en una lista de platillos pedidos
 * @param platillosPedidos Lista de platillos pedidos a verificar
 * @returns True si al menos hay un platillo existente en toda la lista. False si en toda la lista el Platillo original fue null.
 */
export function HayAlMenosUnPlatilloExistente(platillosPedidos: PedidoPlatillo[]):boolean{
    let is = false;
    for(const platilloPedido of platillosPedidos){
        if(platilloPedido.platillo === null) continue; //Si el platillo pedido su platillo original ya no existe se deja pasar
        else{ //Si todavia existe su platillo en DB ponemos que sí
            is = true;
            break;
        }
    }
    return is;
}

/**
 * Checa si al menos hay una descripción con valor en el platillo original
 * @param platillosPedidos Lista de platillos pedidos a verificar, los cuales en su campo Platillo si existe se verificará si su campo `descripcion` no es null
 * @returns True si al menos un platillo original tiene una descripción con valor. False si todos los platillos originales fueron null o en los platillos originales si todos sus campos fueron null.
 */
export function HayAlMenosUnaDescripcionNoNullPlatilloOriginal(platillosPedidos: PedidoPlatillo[]): boolean{
    let is = false;
    for(const platilloPedido of platillosPedidos){
        if(platilloPedido.platillo === null) continue; 
        else{
            if(platilloPedido.platillo.descripcion === null) continue;
            else{
                is = true;
                break;
            }
        }
    }
    return is;
}

/**
 * El campo `platillo` tiene un campo `descripcion` y si este es null se retorna cadena vacía y si tiene valor ese string
 * @param platilloPedido Platillo pedido a verificar
 * @returns Cadena vacía si el platillo pedido su campo `platillo` es null o si dentro de `platillo` su campo `descripcion` es null. String con valor si `platillo` tiene valor y su campo `descripcion` también.
 */
export function GetDescripcionPlatilloOriginal(platilloPedido: PedidoPlatillo): string{
    let descripcionReturn = "";
    if(platilloPedido.platillo !== null){
        if(platilloPedido.platillo.descripcion !== null){
            descripcionReturn = platilloPedido.platillo.descripcion;
        }
    }
    return descripcionReturn;
}

/**
 * Verifica si el nuevo tipo es igual a su original en info_tipo y en ese caso retorna su ID original de info_tipo, de lo contrario 0
 * @param originalPedido El pedido original para indagar en su info_tipo
 * @param nuevoTipo Nuevo tipo para comparar con info_tipo
 * @returns Retorna 0 si los tipos no son iguales porque el usuario cambió de tipo, si coindicen lo mantuvó en el mismo tipo y se retorna el mismo ID de info_tipo
 */
export function GetOriginalIdTipoIfSameOrZero(originalPedido: Pedido, nuevoTipo: TipoPedido): number{
    let id = 0;
    //Si el nuevo tipo pasado es igual al info_tipo original significa que no se cambió el tipo por lo que mantiene su mismo ID
    if(nuevoTipo == "Local" && originalPedido.info_tipo_pedido.tipo == "Local"){
        id = originalPedido.info_tipo_pedido.id;
    }else if(nuevoTipo == "Domicilio" && originalPedido.info_tipo_pedido.tipo == "Domicilio"){
        id = originalPedido.info_tipo_pedido.id;
    }else if(nuevoTipo == "Recoger" && originalPedido.info_tipo_pedido.tipo == "Recoger"){
        id = originalPedido.info_tipo_pedido.id;
    }
    //En otros casos como si nuevoTipo es Domicilio y su original es Local significa que se cambió de tipo 

    return id;
}