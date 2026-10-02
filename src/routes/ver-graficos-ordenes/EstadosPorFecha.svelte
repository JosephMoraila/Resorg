<script lang="ts">
    import { BarChart, Tooltip } from 'layerchart';
    import type { ConteoEstadoPedidosPorFecha } from '$lib/types';
    import { formatearFechaBonito } from '$lib/utils/string_utils';

    interface Props {
        listEstadosPedidosFecha: ConteoEstadoPedidosPorFecha[];
    }

    let { listEstadosPedidosFecha }: Props = $props();

</script>

<div class="h-full w-full mt-10 antialiased">
	<h1 class="text-lg font-bold ml-10">Estados por día</h1>
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
	<BarChart 
		data={listEstadosPedidosFecha}
		x="fecha"
		series={[
			{ 
				key: 'pendiente', 
				color: '#92400e',
				props: { insets: { x: 0 } }
			},
			{
				key: 'finalizado',
				color: '#272120',
				props: { insets: { x: 0 } }
			},
			{
				key: 'cancelado', // <- Agregamos el tercer tipo
				color: '#da3912', // Puedes usar un color hex, tailwind o variable CSS
				props: { insets: { x: 0 } } // Insets más grandes para que quede más delgada y se vea en el centro
			},
			{
				key: 'entregado', // <- Agregamos el tercer tipo
				color: '#0023ff', // Puedes usar un color hex, tailwind o variable CSS
				props: { insets: { x: 0 } } // Insets más grandes para que quede más delgada y se vea en el centro
			},
			{
				key: 'cobrado', // <- Agregamos el tercer tipo
				color: '#10b981', // Puedes usar un color hex, tailwind o variable CSS
				props: { insets: { x: 0 } } // Insets más grandes para que quede más delgada y se vea en el centro
			},
		]}
		seriesLayout="group"
		height={300}
		props={{yAxis: { format: (value) => Number.isInteger(value) ? value : '' }}}
	>

        {#snippet tooltip({ context })}
            {@const data = context.tooltip.data as ConteoEstadoPedidosPorFecha}
    
            {#if data}
            <!-- 1. Usa Tooltip.Root en lugar de Tooltip solo -->
            <Tooltip.Root {context}>
                
                <div class="bg-white text-black dark:bg-gray-600 dark:text-white p-2 rounded shadow-lg">
					<div class="flex flex-col">

						<div class="flex flex-row items-center">
							<div class="w-4 h-4 bg-[#da3912] rounded-full mr-4"></div>
							<span>Cancelado: {data.cancelado}</span>
						</div>

						<div class="flex flex-row items-center">
							<div class="w-4 h-4 bg-[#92400e] rounded-full mr-4"></div>
							<span>Pendiente: {data.pendiente}</span>
						</div>

						<div class="flex flex-row items-center">
							<div class="w-4 h-4 bg-[#10b981] rounded-full mr-4"></div>
							<span>Cobrado: {data.cobrado}</span>
						</div>

						<div class="flex flex-row items-center">   
							<div class="w-4 h-4 bg-[#272120] rounded-full mr-4"></div>
							<span>Finalizado: {data.finalizado}</span>
						</div>

						<div class="flex flex-row items-center">
							<div class="w-4 h-4 bg-[#0023ff] rounded-full mr-4"></div>
							<span>Entregado: {data.entregado}</span>
						</div>

					</div>
                    <p>Fecha: {formatearFechaBonito(data.fecha)}</p>
                </div>
                
            </Tooltip.Root>
            {/if}
        {/snippet}
	
	</BarChart>
</div>