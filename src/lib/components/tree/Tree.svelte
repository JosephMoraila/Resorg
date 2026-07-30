<script lang="ts" generics="T">
  import Tree from "./Tree.svelte";
  import type { NodoArbol, PropsArbol } from "$lib/types";

  let { nodo, onSeleccionar }: PropsArbol<T> = $props();

  let expandido = $state(false);

  function alHacerClic() {
    if (nodo.hijos && nodo.hijos.length > 0) {
      expandido = !expandido;
    }
    onSeleccionar?.(nodo);
  }
</script>

<div class="w-full">
  <button
    onclick={alHacerClic}
    class="w-full text-left px-2 py-1 text-sm hover:bg-black/10 dark:hover:bg-white/10 rounded transition-colors"
  >
    {#if nodo.hijos && nodo.hijos.length > 0}
      {expandido ? "▾" : "▸"}
    {/if}
    {nodo.label}
  </button>

  {#if expandido && nodo.hijos}
    <div class="w-full pl-3 border-l border-gray-300 dark:border-white/10">
      {#each nodo.hijos as hijo (hijo.id)}
        <Tree nodo={hijo} {onSeleccionar} />
      {/each}
    </div>
  {/if}
</div>