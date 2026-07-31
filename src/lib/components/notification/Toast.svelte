<script>
  import { toast } from "$lib/toast.svelte";

  const colorClasses = {
    rojo: 'bg-red-600',
    azul: 'bg-blue-600',
    verde: 'bg-green-600',
    amarillo: 'bg-yellow-500',
  }
</script>

<div class="fixed top-4 left-1/2 -translate-x-1/2 flex flex-col gap-2 z-50 pointer-events-none">
  {#each toast.all as t (t.id)}
    <div class="px-5 py-3 rounded-lg text-white shadow-lg {colorClasses[t.color]}">
      {t.message}
    </div>
  {/each}
</div>