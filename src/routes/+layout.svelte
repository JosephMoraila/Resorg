<script>
  import "../app.css";
  import { themeStore } from "../lib/theme.svelte";

  let { children } = $props();

  $effect(() => {
    themeStore.inicializar();
  });
</script>

{@render children()}