<script lang="ts">
    import type { ConteoPedidosPorFecha } from "$lib/types";
    import { formatearFechaBonito } from "$lib/utils/string_utils";
    import { BarChart, Tooltip } from 'layerchart';

    interface Props{
        listPedidosFecha: ConteoPedidosPorFecha[];
    }

    let {listPedidosFecha}:Props = $props();

</script>


<div class="h-full w-full mt-10 antialiased">
    <h1 class="text-lg font-bold ml-10">Pedidos por día</h1>
    <BarChart data={listPedidosFecha} x="fecha" y="total" height={300} labels={{ format: (v: number) => String(Math.round(v)), }}>
        {#snippet tooltip({ context })}
            {@const data = context.tooltip.data as ConteoPedidosPorFecha}
    
            {#if data}
            <!-- 1. Usa Tooltip.Root en lugar de Tooltip solo -->
            <Tooltip.Root {context}>
                
                <div class="bg-white text-black dark:bg-gray-600 dark:text-white p-2 rounded shadow-lg">
                    <p>Número pedidos: {data.total}</p>
                    <p>Fecha: {formatearFechaBonito(data.fecha)}</p>
                </div>
                
            </Tooltip.Root>
            {/if}
        {/snippet}
    </BarChart>
</div>