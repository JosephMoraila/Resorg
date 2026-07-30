import { emit, listen } from "@tauri-apps/api/event";

type Theme = "claro" | "oscuro" | "sistema";

let theme = $state<Theme>("sistema");

function aplicarTema(t: Theme) {
  const esOscuro =
    t === "oscuro" ||
    (t === "sistema" && window.matchMedia("(prefers-color-scheme: dark)").matches);

  document.documentElement.classList.toggle("dark", esOscuro);
}

function inicializar() {
  const saved = localStorage.getItem("theme") as Theme | null;
  theme = saved ?? "sistema";
  aplicarTema(theme);

  // Escucha cambios que vengan de OTRAS ventanas
  listen<Theme>("theme-changed", (event) => {
    theme = event.payload;
    aplicarTema(theme);
  });
}

function setTheme(t: Theme) {
  theme = t;
  aplicarTema(t);
  localStorage.setItem("theme", t);

  // Avisa a TODAS las demás ventanas del cambio
  emit("theme-changed", t);
}

export const themeStore = {
  get value() {
    return theme;
  },
  setTheme,
  inicializar,
};