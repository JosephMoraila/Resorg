<script>
  import "../app.css";
  import { themeStore } from "../lib/theme.svelte";
  import Toast from "$lib/components/notification/Toast.svelte";

  let { children } = $props();

  $effect(() => {
    themeStore.inicializar();
  });
</script>

<Toast />
{@render children()}