<script lang="ts">
    import type { GeneralPlatillosPie } from "$lib/types";
    import { sum } from 'd3-array';
    import { format } from '@layerstack/utils';
    import { Arc, Chart, Layer, Pie, Text, Tooltip } from 'layerchart'; // <- Importamos Tooltip

    interface Props {
        generalPlatillosPie: GeneralPlatillosPie[];
    }

    let { generalPlatillosPie }: Props = $props();

    const paletaColores = [
        '#f59e0b', '#10b981', '#3b82f6', '#ef4444', 
        '#8b5cf6', '#ec4899', '#14b8a6', '#f97316'
    ];

    let chartData = $derived(generalPlatillosPie.map((item, index) => ({
        label: item.nombre,
        value: item.cantidad_vendida,
        color: paletaColores[index % paletaColores.length]
    })));

    let dataSum = $derived(sum(chartData, (d) => d.value) || 1);
</script>

<div class="flex flex-col w-full mt-10 antialiased min-h-min pb-10">
    <h1 class="text-lg font-bold ml-10 mb-4">Distribución de platillos</h1>

    <!-- Leyenda horizontal -->
    <div class="flex flex-row flex-wrap items-center gap-3 ml-10 mb-6">
        {#each chartData as d, i}
            <div class="flex flex-row items-center">
                <div class="w-4 h-4 rounded-full mr-2" style="background-color: {d.color}"></div>
                <span class="text-sm font-medium">{d.label}</span>
            </div>
            {#if i < chartData.length - 1}
                <span class="text-gray-400 font-bold">||</span>
            {/if}
        {/each}
    </div>

    <!-- Gráfico -->
    <Chart data={chartData} x="value" c="label" height={300}>
        <!-- 1. Extraemos el 'context' del Chart -->
        {#snippet children({ context })}
            <Layer center>
                <Pie>
                    {#snippet children({ arcs })}
                        {#each arcs as arc}
                            <Arc
                                startAngle={arc.startAngle}
                                endAngle={arc.endAngle}
                                padAngle={arc.padAngle}
                                fill={arc.data.color}
                                
                                class="transition-opacity duration-200 hover:opacity-80 outline-none cursor-pointer"
                                role="graphics-symbol"
                                tabindex={-1}
                                aria-label={`${arc.data.label}: ${arc.data.value} ventas`}
                                
                                onpointermove={(e) => context.tooltip.show(e, arc.data)}
                                onpointerleave={() => context.tooltip.hide()}
                            >
                                {#snippet children({ getArcTextProps })}
                                    {@const textProps = getArcTextProps('centroid')}
                                    
                                    {#if arc.data.value > 0}
                                        <Text
                                            value={format(arc.data.value / dataSum, 'percent')}
                                            {...textProps}
                                            dy={-8}
                                            class="text-base fill-white font-bold pointer-events-none"
                                        />
                                        <Text
                                            value={arc.data.value}
                                            {...textProps}
                                            dy={12}
                                            class="text-sm fill-white opacity-90 pointer-events-none"
                                        />
                                    {/if}
                                {/snippet}
                            </Arc>
                        {/each}
                    {/snippet}
                </Pie>
            </Layer>

            <!-- 3. Diseño del Tooltip Flotante -->
            <Tooltip.Root {context}>
                {@const data = context.tooltip.data as { label: string, value: number, color: string }}
                
                {#if data}
                    <div class="p-2 min-w-32 bg-white text-black dark:bg-gray-800 dark:text-white rounded shadow-lg border border-gray-200 dark:border-gray-700">
                        <div class="flex items-center justify-between gap-4">
                            <!-- Color y Nombre -->
                            <div class="flex items-center gap-2">
                                <div 
                                    class="w-3 h-3 rounded-full" 
                                    style="background-color: {data.color}"
                                ></div>
                                <span class="font-medium">{data.label}</span>
                            </div>
                            
                            <!-- Cantidad y Porcentaje -->
                            <div class="flex flex-col items-end">
                                <span class="font-bold text-lg leading-none">{data.value}</span>
                                <span class="text-xs text-gray-500 dark:text-gray-400">
                                    {format(data.value / dataSum, 'percent')}
                                </span>
                            </div>
                        </div>
                    </div>
                {/if}
            </Tooltip.Root>

        {/snippet}
    </Chart>
</div>