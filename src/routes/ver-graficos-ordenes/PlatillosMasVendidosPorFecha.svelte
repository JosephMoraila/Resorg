<script lang="ts">
    import { Chart, Svg, Axis, Tooltip } from 'layerchart';
    import { scaleBand } from 'd3-scale'; 
    import type { ConteoPlatillosMasVendidosPorFecha } from '$lib/types';
    import { formatearFechaBonito } from '$lib/utils/string_utils';

    interface Props {
        listPlatillosMasVendidosFecha: ConteoPlatillosMasVendidosPorFecha[];
    }

    let { listPlatillosMasVendidosFecha }: Props = $props();

    // 1. Obtener todos los nombres de platillos únicos (Eje Y)
    let platillosUnicos = $derived([
        ...new Set(
            listPlatillosMasVendidosFecha.flatMap((item) =>
                item.platillos.map((p) => p.nombre)
            )
        )
    ]);

    // 2. Aplanar datos: El Heatmap necesita 1 objeto por cada celda de la cuadrícula
    let chartData = $derived.by(() => {
        const flat = [];
        for (const item of listPlatillosMasVendidosFecha) {
            for (const platillo of platillosUnicos) {
                // Buscamos si ese platillo se vendió en esta fecha
                const venta = item.platillos.find((p) => p.nombre === platillo);
                flat.push({
                    fecha: item.fecha,
                    platillo: platillo,
                    cantidad: venta ? venta.cantidad_vendida : 0
                });
            }
        }
        return flat;
    });

    // 3. Encontrar el día de más ventas para calcular la intensidad del color
    let maxCantidad = $derived(Math.max(...chartData.map((d) => d.cantidad)) || 1);

    // 4. Función para pintar la celda (Usando colores Tailwind Amber)
    function getColor(cantidad: number, max: number) {
        if (cantidad === 0) return '#f3f4f6'; // gray-100 (Cero ventas)

        const intensidad = cantidad / max;
        if (intensidad > 0.8) return '#92400e'; // amber-800 (Top ventas)
        if (intensidad > 0.6) return '#d97706'; // amber-600
        if (intensidad > 0.4) return '#f59e0b'; // amber-500
        if (intensidad > 0.2) return '#fbbf24'; // amber-400
        return '#fde68a';                       // amber-200
    }
</script>

<div class="h-100 w-full mt-10 antialiased p-4">
    <h1 class="text-lg font-bold ml-10 mb-6">Mapa de Calor de Ventas</h1>

    <Chart
        data={chartData}
        x="fecha"
        xScale={scaleBand().paddingInner(0.05)}
        y="platillo"
        yScale={scaleBand().paddingInner(0.05)}
        padding={{ left: 80, bottom: 40, top: 10, right: 10 }}
    >
        {#snippet children({ context })}
            <Svg>
                <Axis placement="left" rule={false} tickLength={0} />
                <Axis 
                    placement="bottom" 
                    rule={false} 
                    tickLength={0} 
                    format={(d) => new Date(d).toLocaleDateString()} 
                />

                <g>
                    {#each chartData as d}
                        <rect
                            x={context.xScale(d.fecha) ?? 0}
                            y={context.yScale(d.platillo) ?? 0}
                            width={context.xScale.bandwidth?.() ?? 0}
                            height={context.yScale.bandwidth?.() ?? 0}
                            fill={getColor(d.cantidad, maxCantidad)}
                            rx={4} 
                            class="transition-colors duration-200 hover:stroke-black hover:stroke-2 outline-none"
                            
                            role="graphics-symbol"
                            tabindex="-1"
                            aria-label={`${d.platillo} el ${new Date(d.fecha).toLocaleDateString()}: ${d.cantidad} ventas`}
                            
                            onpointermove={(e) => context.tooltip.show(e, d)}
                            onpointerleave={() => context.tooltip.hide()}
                        />
                    {/each}
                </g>
            </Svg>

            <Tooltip.Root {context}>
                
                <!-- 2. Obtenemos la data tipándola manualmente para evitar el error 'any' -->
                {@const data = context.tooltip.data as { fecha: Date, platillo: string, cantidad: number }}
                
                {#if data}
                    <!-- 3. Cambiamos a min-w-30 y añadimos fondo/sombra -->
                    <div class="p-2 min-w-30 bg-white text-black dark:bg-gray-600 dark:text-white rounded shadow-lg">
                        <div class="font-bold border-b border-gray-300 dark:border-gray-500 pb-1 mb-2">
                            {formatearFechaBonito(data.fecha)}
                        </div>
                        
                        <div class="flex justify-between items-center gap-4">
                            <div class="flex items-center gap-2">
                                <div 
                                    class="w-3 h-3 rounded-full" 
                                    style="background-color: {getColor(data.cantidad, maxCantidad)}"
                                ></div>
                                <span>{data.platillo}</span>
                            </div>
                            <span class="font-bold">{data.cantidad}</span>
                        </div>
                    </div>
                {/if}
            </Tooltip.Root>
            
        {/snippet}
    </Chart>
</div>