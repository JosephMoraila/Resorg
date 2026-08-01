<script lang="ts">
  interface Props {
    texto: string;
    imagen?: string | null;
    visible?: boolean;
  }

  let { texto, imagen = null, visible = true }: Props = $props();
</script>

{#if visible}
  <div class="fixed bottom-4 left-4 w-80 bg-white dark:bg-[#1a1f2e] border border-gray-200 dark:border-white/10
    rounded-lg shadow-xl p-4 flex gap-3 items-start z-50">

    {#if imagen}
      <img src={imagen} alt="" class="w-40 h-40 rounded-md object-cover flex-shrink-0" />
    {/if}

    <p class="text-sm text-gray-900 dark:text-white whitespace-pre-line break-words">
      {texto}
    </p>
  </div>
{/if}