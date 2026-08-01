<!-- Tooltip.svelte -->
<script lang="ts">
  interface Props {
    texto: string;
    children: import('svelte').Snippet;
    posicion?: 'top' | 'bottom';
  }

  let { texto, children, posicion = 'top' }: Props = $props();
  let visible = $state(false);
</script>

<div role="tooltip"
  class="relative inline-block"
  onmouseenter={() => visible = true}
  onmouseleave={() => visible = false}
>
  {@render children()}

  {#if visible}
    <span
      class="absolute left-1/2 -translate-x-1/2 px-2 py-1 text-xs whitespace-nowrap
        bg-gray-900 text-white rounded shadow-lg pointer-events-none z-50
        {posicion === 'top' ? 'bottom-full mb-2' : 'top-full mt-2'}"
    >
      {texto}
    </span>
  {/if}
</div>