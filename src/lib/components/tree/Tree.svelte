<script lang="ts" generics="T">
  import Tree from "./Tree.svelte";
  import type { NodoArbol, PropsArbol } from "$lib/types";

  let { nodo = null, nodos, onSeleccionar, selectedId = null }: PropsArbol<T> = $props();

  let expandido = $state(false);

  function alHacerClic() {
    if (!nodo) return;
    if (nodo.hijos && nodo.hijos.length > 0) {
      expandido = !expandido;
    }
    onSeleccionar?.(nodo);
  }
</script>

{#if nodos && nodos.length > 0}
  <!-- Modo "múltiples raíces": renderiza un Tree por cada nodo del arreglo -->
  <div class="w-full">
    {#each nodos as raiz (raiz.id)}
      <Tree
        nodo={raiz as unknown as NodoArbol<unknown>}
        onSeleccionar={onSeleccionar as unknown as ((nodo: NodoArbol<unknown>) => void) | undefined}
        {selectedId}
      />
    {/each}
  </div>
{:else if nodos && nodos.length === 0}
  <p class="w-full text-sm text-gray-400 dark:text-gray-500 italic px-2 py-1">
    Sin elementos
  </p>
{:else if nodo === null}
  <p class="w-full text-sm text-gray-400 dark:text-gray-500 italic px-2 py-1">
    Sin elementos
  </p>
{:else}
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
{/if}