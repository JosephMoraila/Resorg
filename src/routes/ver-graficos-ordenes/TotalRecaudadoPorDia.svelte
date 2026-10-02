<script lang="ts">
    import { BarChart, Tooltip } from 'layerchart';
    import type { ConteoTotalRecaudadoPorFecha } from '$lib/types';
    import { formatearFechaBonito, formatearMoneda } from '$lib/utils/string_utils';

    interface Props {
        listTotalRecaudadoFecha: ConteoTotalRecaudadoPorFecha[];
    }

    let { listTotalRecaudadoFecha }: Props = $props();

</script>

<div class="h-full w-full mt-10 antialiased ml-13">
    <h1 class="text-lg font-bold ml-10">Total recaudado por día</h1>
    <BarChart data={listTotalRecaudadoFecha} x="fecha" y="total" height={300} >
        {#snippet tooltip({ context })}
            {@const data = context.tooltip.data as ConteoTotalRecaudadoPorFecha}
    
            {#if data}
            <!-- 1. Usa Tooltip.Root en lugar de Tooltip solo -->
            <Tooltip.Root {context}>
                
                <div class="bg-white text-black dark:bg-gray-600 dark:text-white p-2 rounded shadow-lg">
                    <p>Total recaudado: {formatearMoneda(data.total)}</p>
                    <p>Fecha: {formatearFechaBonito(data.fecha)}</p>
                </div>
                
            </Tooltip.Root>
            {/if}
        {/snippet}
    </BarChart>

</div>