type ToastColor = 'rojo' | 'azul' | 'verde' | 'amarillo'
type Toast = { id: number; message: string; color: ToastColor }

let toasts = $state<Toast[]>([])
let nextId = 0

function show(message: string, color: ToastColor = 'azul', duration = 3000) {
  const id = nextId++
  toasts.push({ id, message, color })

  setTimeout(() => {
    toasts = toasts.filter(t => t.id !== id)
  }, duration)
}

export const toast = {
  get all() { return toasts },
  show,
  rojo: (msg: string, duration?: number) => show(msg, 'rojo', duration),
  azul: (msg: string, duration?: number) => show(msg, 'azul', duration),
  verde: (msg: string, duration?: number) => show(msg, 'verde', duration),
  amarillo: (msg: string, duration?: number) => show(msg, 'amarillo', duration),
}