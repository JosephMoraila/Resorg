<script lang="ts">
    import { sum } from 'd3-array';
    import { format } from '@layerstack/utils';
    import { Arc, Chart, Layer, Pie, Text } from 'layerchart';
    import type { GeneralTipoPedidoPie } from '$lib/types';

    interface Props {
        generalTipoPedidoPie: GeneralTipoPedidoPie;
    }

    let { generalTipoPedidoPie }: Props = $props();

    // 1. Transformamos el objeto único en un arreglo que LayerChart pueda entender
    // y le asignamos los colores exactos que usaste en el gráfico anterior
    let chartData = $derived([
        { label: 'Local', value: generalTipoPedidoPie.tipoLocal, color: 'fill-[#da3912]' },
        { label: 'Domicilio', value: generalTipoPedidoPie.tipoDomicilio, color: 'fill-[#0023ff]' },
        { label: 'Recoger', value: generalTipoPedidoPie.tipoRecoger, color: 'fill-[#10b981]' }
    ]);

    // 2. Calculamos el total sumando los valores del arreglo
    let dataSum = $derived(sum(chartData, (d) => d.value) || 1); // || 1 evita dividir entre cero
</script>

<div class="flex flex-col w-full mt-10 antialiased min-h-min pb-10">
    <h1 class="text-lg font-bold ml-10">Distribución de tipos</h1>

	<div class="flex flex-col">

		<div class="flex flex-row items-center">
			<div class="w-4 h-4 bg-[#da3912] rounded-full ml-10 mr-4"></div>
			<span>Tipo local</span>
		</div>

		<div class="flex flex-row items-center">
			<div class="w-4 h-4 bg-[#0023ff] rounded-full ml-10 mr-4"></div>
			<span>Tipo domicilio</span>
		</div>

		<div class="flex flex-row items-center">
			<div class="w-4 h-4 bg-[#10b981] rounded-full ml-10 mr-4"></div>
			<span>Tipo recoger</span>
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