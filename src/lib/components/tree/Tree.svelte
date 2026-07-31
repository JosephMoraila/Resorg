<script lang="ts" generics="T">
  import Tree from "./Tree.svelte";
  import type { NodoArbol, PropsArbol } from "$lib/types";

  let { nodo, onSeleccionar, selectedId = null }: PropsArbol<T> = $props();

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
    class="w-full text-left px-2 py-1 text-sm rounded transition-colors
      {nodo.id === selectedId ? 'bg-blue-600 text-white' : 'hover:bg-black/10 dark:hover:bg-white/10'}"
    >
    {#if nodo.hijos && nodo.hijos.length > 0}
      {expandido ? "▾" : "▸"}
    {/if}
    {nodo.label}
  </button>

  {#if expandido && nodo.hijos}
    <div class="w-full pl-3 border-l border-gray-300 dark:border-white/10">
      {#each nodo.hijos as hijo (hijo.id)}
        <Tree
          nodo={hijo as unknown as NodoArbol<unknown>}
          onSeleccionar={onSeleccionar as unknown as ((nodo: NodoArbol<unknown>) => void) | undefined}
          {selectedId}
        />
      {/each}
    </div>
  {/if}
</div>