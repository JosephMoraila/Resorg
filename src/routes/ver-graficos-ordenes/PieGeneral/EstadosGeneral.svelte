<script lang="ts">
    import { sum } from 'd3-array';
    import { format } from '@layerstack/utils';
    import { Arc, Chart, Layer, Pie, Text, Tooltip } from 'layerchart';
    import type { GeneralEstadoPedidoPie } from '$lib/types';

    interface Props {
        generalEstadoPedidoPie: GeneralEstadoPedidoPie;
    }

    let { generalEstadoPedidoPie }: Props = $props();

    // Cambiamos a colores hexadecimales puros
    let chartData = $derived([
        { label: 'Pendiente', value: generalEstadoPedidoPie.pendiente, color: '#92400e' },
        { label: 'Finalizado', value: generalEstadoPedidoPie.finalizado, color: '#272120' },
        { label: 'Cancelado', value: generalEstadoPedidoPie.cancelado, color: '#da3912' },
        { label: 'Entregado', value: generalEstadoPedidoPie.entregado, color: '#0023ff' },
        { label: 'Cobrado', value: generalEstadoPedidoPie.cobrado, color: '#10b981' }
    ]);

    let dataSum = $derived(sum(chartData, (d) => d.value) || 1);
</script>

<div class="flex flex-col w-full mt-10 antialiased min-h-min pb-10">
    <h1 class="text-lg font-bold ml-10">Distribución de estados</h1>

    <div class="flex flex-col mt-4 mb-4">
        <div class="flex flex-row items-center">
            <div class="w-4 h-4 bg-[#da3912] rounded-full ml-10 mr-4"></div>
            <span>Cancelado</span>
        </div>
        <div class="flex flex-row items-center">
            <div class="w-4 h-4 bg-[#92400e] rounded-full ml-10 mr-4"></div>
            <span>Pendiente</span>
        </div>
        <div class="flex flex-row items-center">
            <div class="w-4 h-4 bg-[#10b981] rounded-full ml-10 mr-4"></div>
            <span>Cobrado</span>
        </div>
        <div class="flex flex-row items-center">
            <div class="w-4 h-4 bg-[#272120] rounded-full ml-10 mr-4"></div>
            <span>Finalizado</span>
        </div>
        <div class="flex flex-row items-center">
            <div class="w-4 h-4 bg-[#0023ff] rounded-full ml-10 mr-4"></div>
            <span>Entregado</span>
        </div>
    </div>

    <Chart data={chartData} x="value" c="label" height={300}>
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
                                aria-label={`${arc.data.label}: ${arc.data.value} pedidos`}
                                
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

            <Tooltip.Root {context}>
                {@const data = context.tooltip.data as { label: string, value: number, color: string }}
                
                {#if data}
                    <div class="p-2 min-w-32 bg-white text-black dark:bg-gray-800 dark:text-white rounded shadow-lg border border-gray-200 dark:border-gray-700">
                        <div class="flex items-center justify-between gap-4">
                            <div class="flex items-center gap-2">
                                <div class="w-3 h-3 rounded-full" style="background-color: {data.color}"></div>
                                <span class="font-medium">{data.label}</span>
                            </div>
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