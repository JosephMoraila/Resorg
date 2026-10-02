<script lang="ts">
    import { BarChart, Tooltip } from 'layerchart';
    import type { ConteoPedidosPorTipo } from '$lib/types';
	import { formatearFechaBonito } from '$lib/utils/string_utils';

    interface Props {
        listTiposPedidosFecha: ConteoPedidosPorTipo[];
    }

    let { listTiposPedidosFecha }: Props = $props();
</script>


<div class="h-full w-full mt-10 antialiased">
	<h1 class="text-lg font-bold ml-10">Tipos por día</h1>
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
	<BarChart 
		data={listTiposPedidosFecha}
		x="fecha"
		series={[
			{ 
				key: 'tipoLocal', 
				color: '#da3912',
				props: { insets: { x: 0 } }
			},
			{
				key: 'tipoDomicilio',
				color: '#0023ff',
				props: { insets: { x: 10 } }
			},
			{
				key: 'tipoRecoger', // <- Agregamos el tercer tipo
				color: '#10b981', // Puedes usar un color hex, tailwind o variable CSS
				props: { insets: { x: 20 } } // Insets más grandes para que quede más delgada y se vea en el centro
			},
		]}
		seriesLayout="overlap"
		height={300}
		props={{yAxis: { format: (value) => Number.isInteger(value) ? value : '' }}}
	>

        {#snippet tooltip({ context })}
            {@const data = context.tooltip.data as ConteoPedidosPorTipo}
    
            {#if data}
            <!-- 1. Usa Tooltip.Root en lugar de Tooltip solo -->
            <Tooltip.Root {context}>
                
                <div class="bg-white text-black dark:bg-gray-600 dark:text-white p-2 rounded shadow-lg">
					<div class="flex flex-col">

						<div class="flex flex-row items-center">
							<div class="w-4 h-4 bg-[#da3912] rounded-full mr-4"></div>
							<span>Tipo local: {data.tipoLocal}</span>
						</div>

						<div class="flex flex-row items-center">
							<div class="w-4 h-4 bg-[#0023ff] rounded-full mr-4"></div>
							<span>Tipo domicilio: {data.tipoDomicilio}</span>
						</div>

						<div class="flex flex-row items-center">
							<div class="w-4 h-4 bg-[#10b981] rounded-full mr-4"></div>
							<span>Tipo recoger: {data.tipoRecoger}</span>
						</div>

					</div>
                    <p>Fecha: {formatearFechaBonito(data.fecha)}</p>
                </div>
                
            </Tooltip.Root>
            {/if}
        {/snippet}
	
	</BarChart>
</div>