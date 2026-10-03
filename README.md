# Resorg 🍽️

**Resorg** es una aplicación de escritorio diseñada para facilitar la organización y gestión de pedidos en restaurantes. Permite registrar órdenes, dar seguimiento a estados (comedor, domicilio, recoger) y visualizar métricas mediante gráficos interactivos.

---

## 🚀 Características principales

* **Gestión de Órdenes:** Registro y búsqueda rápida de pedidos por cliente, tipo y estado.
* **Métricas y Gráficos:** Visualización clara de ventas e historial de órdenes.
* **Interfaz Fluida:** Desarrollada con SvelteKit y TypeScript sobre Tauri v2.
* **Integración Nativa:** Compatible con entornos Linux (Fedora/KDE, GNOME, Ubuntu, etc.).

---

## 📦 Instalación

Puedes descargar los paquetes e instaladores oficiales para Linux y Windows desde la sección de [Releases](../../releases):

### Linux
* **Fedora / RHEL / openSUSE:** Descarga e instala el paquete `.rpm`
* **Ubuntu / Debian / Mint:** Descarga e instala el paquete `.deb`
* **Universal (Cualquier distro):** Descarga el archivo `.AppImage` y dale permisos de ejecución:
  ```bash
  chmod +x Resorg_0.1.0_amd64.AppImage
  ./Resorg_0.1.0_amd64.AppImage