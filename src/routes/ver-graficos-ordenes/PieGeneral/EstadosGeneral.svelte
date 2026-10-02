<script lang="ts">
    import { sum } from 'd3-array';
    import { format } from '@layerstack/utils';
    import { Arc, Chart, Layer, Pie, Text } from 'layerchart';
    import type { GeneralEstadoPedidoPie } from '$lib/types';

    interface Props {
        generalEstadoPedidoPie: GeneralEstadoPedidoPie;
    }

    let { generalEstadoPedidoPie }: Props = $props();

    // 1. Transformamos el objeto único en un arreglo que LayerChart pueda entender
    // y le asignamos los colores exactos que usaste en el gráfico anterior
    let chartData = $derived([
        { label: 'Pendiente', value: generalEstadoPedidoPie.pendiente, color: 'fill-[#92400e]' },
        { label: 'Finalizado', value: generalEstadoPedidoPie.finalizado, color: 'fill-[#272120]' },
        { label: 'Cancelado', value: generalEstadoPedidoPie.cancelado, color: 'fill-[#da3912]' },
        { label: 'Entregado', value: generalEstadoPedidoPie.entregado, color: 'fill-[#0023ff]' },
        { label: 'Cobrado', value: generalEstadoPedidoPie.cobrado, color: 'fill-[#10b981]' }
    ]);

    // 2. Calculamos el total sumando los valores del arreglo
    let dataSum = $derived(sum(chartData, (d) => d.value) || 1); // || 1 evita dividir entre cero
</script>

<div class="flex flex-col w-full mt-10 antialiased min-h-min pb-10">
    <h1 class="text-lg font-bold ml-10">Distribución de estados</h1>

	<div class="flex flex-col">

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

    <!-- 3. Pasamos el arreglo chartData y le decimos que el valor a graficar es "value" -->
    <Chart data={chartData} x="value" c="label" height={300}>
        <Layer center>
            <Pie>
                {#snippet children({ arcs })}
                    {#each arcs as arc}
                        <Arc
                            startAngle={arc.startAngle}
                            endAngle={arc.endAngle}
                            padAngle={arc.padAngle}
                            class={arc.data.color}
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