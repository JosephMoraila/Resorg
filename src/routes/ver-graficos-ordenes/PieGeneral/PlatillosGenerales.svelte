<script lang="ts">
    import type { GeneralPlatillosPie } from "$lib/types";
    import { sum } from 'd3-array';
    import { format } from '@layerstack/utils';
    import { Arc, Chart, Layer, Pie, Text } from 'layerchart';

    interface Props {
        generalPlatillosPie: GeneralPlatillosPie[];
    }

    let { generalPlatillosPie }: Props = $props();

    // 1. Paleta de colores en Hexadecimal puro
    const paletaColores = [
        '#f59e0b', // Naranja
        '#10b981', // Verde
        '#3b82f6', // Azul
        '#ef4444', // Rojo
        '#8b5cf6', // Violeta
        '#ec4899', // Rosa
        '#14b8a6', // Teal
        '#f97316'  // Naranja oscuro
    ];

    let chartData = $derived(generalPlatillosPie.map((item, index) => ({
        label: item.nombre,
        value: item.cantidad_vendida,
        color: paletaColores[index % paletaColores.length] // Asigna color dinámico
    })));

    let dataSum = $derived(sum(chartData, (d) => d.value) || 1);
</script>

<div class="flex flex-col w-full mt-10 antialiased min-h-min pb-10">
    <h1 class="text-lg font-bold ml-10 mb-4">Distribución de platillos</h1>

    <!-- LEYENDA HORIZONTAL -->
    <div class="flex flex-row flex-wrap items-center gap-3 ml-10 mb-6">
        {#each chartData as d, i}
            <!-- Ítem de la leyenda -->
            <div class="flex flex-row items-center">
                <!-- Usamos style="background-color: ..." para inyectar el color dinámicamente -->
                <div class="w-4 h-4 rounded-full mr-2" style="background-color: {d.color}"></div>
                <span class="text-sm font-medium">{d.label}</span>
            </div>

            <!-- Separador '||' (No se dibuja después del último elemento) -->
            {#if i < chartData.length - 1}
                <span class="text-gray-400 font-bold">||</span>
            {/if}
        {/each}
    </div>

    <!-- GRÁFICO -->
    <Chart data={chartData} x="value" c="label" height={300}>
        <Layer center>
            <Pie>
                {#snippet children({ arcs })}
                    {#each arcs as arc}
                        <Arc
                            startAngle={arc.startAngle}
                            endAngle={arc.endAngle}
                            padAngle={arc.padAngle}
                            fill={arc.data.color}
                        >
                            {#snippet children({ getArcTextProps })}
                                {@const textProps = getArcTextProps('centroid')}
                                
                                {#if arc.data.value > 0}
                                    <Text
                                        value={format(arc.data.value / dataSum, 'percent')}
                                        {...textProps}
                                        dy={-8}
                                        class="text-base fill-white font-bold"
                                    />
                                    <Text
                                        value={arc.data.value}
                                        {...textProps}
                                        dy={12}
                                        class="text-sm fill-white opacity-90"
                                    />
                                {/if}
                            {/snippet}
                        </Arc>
                    {/each}
                {/snippet}
            </Pie>
        </Layer>
    </Chart>
</div>