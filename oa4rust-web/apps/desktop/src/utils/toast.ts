/**
 * Lightweight toast/notification utility matching the project's sci-fi theme.
 * No external dependencies — uses native CSS + DOM APIs.
 */

interface ToastOptions {
  type?: 'success' | 'error' | 'warning' | 'info'
  duration?: number
  message: string
}

let container: HTMLDivElement | null = null
let queue: ToastOptions[] = []
const MAX_VISIBLE = 4

function getContainer(): HTMLDivElement {
  if (!container) {
    container = document.createElement('div')
    container.className = 'oa4-toast-container'
    container.style.cssText = `
      position: fixed; top: 20px; right: 20px; z-index: 9999;
      display: flex; flex-direction: column; gap: 8px;
      pointer-events: none; max-width: 360px;
    `
    document.body.appendChild(container)
  }
  return container
}

function showToast(opts: ToastOptions): void {
  const { type = 'info', duration = 3000, message } = opts
  const container = getContainer()

  // Enforce max visible
  const visible = container.querySelectorAll('.oa4-toast').length
  if (visible >= MAX_VISIBLE) {
    const oldest = container.querySelector('.oa4-toast') as HTMLElement
    if (oldest) oldest.remove()
  }

  const colors = {
    success: '#10b981',
    error: '#ef4444',
    warning: '#f59e0b',
    info: '#3b82f6',
  }
  const icons = { success: '✓', error: '✗', warning: '⚠', info: 'ℹ' }
  const color = colors[type] || colors.info
  const icon = icons[type] || icons.info

  const el = document.createElement('div')
  el.className = 'oa4-toast'
  el.style.cssText = `
    display: flex; align-items: center; gap: 10px;
    padding: 10px 14px; border-radius: 8px;
    background: rgba(15, 23, 42, 0.95);
    border: 1px solid ${color}44;
    box-shadow: 0 4px 20px rgba(0,0,0,0.4), 0 0 0 1px ${color}22;
    color: #e2e8f0; font-size: 13px; font-family: 'JetBrains Mono', monospace;
    animation: oa4ToastIn 0.2s ease;
    pointer-events: auto; cursor: pointer;
    backdrop-filter: blur(8px);
  `
  el.innerHTML = `
    <span style="color:${color};font-size:16px;font-weight:700;flex-shrink:0">${icon}</span>
    <span style="flex:1;word-break:break-word">${message}</span>
    <span style="color:${color};font-size:10px;flex-shrink:0;opacity:0.7">auto-hide</span>
  `
  el.addEventListener('click', () => removeToast(el))
  container.appendChild(el)

  const timer = setTimeout(() => removeToast(el), duration)
  el.addEventListener('mouseenter', () => clearTimeout(timer))
  el.addEventListener('mouseleave', () => {
    setTimeout(() => removeToast(el), duration / 2)
  })
}

function removeToast(el: HTMLElement): void {
  el.style.animation = 'oa4ToastOut 0.2s ease forwards'
  setTimeout(() => el.remove(), 200)
}

// Inject keyframes once
if (!document.getElementById('oa4-toast-styles')) {
  const style = document.createElement('style')
  style.id = 'oa4-toast-styles'
  style.textContent = `
    @keyframes oa4ToastIn {
      from { opacity: 0; transform: translateX(40px); }
      to { opacity: 1; transform: translateX(0); }
    }
    @keyframes oa4ToastOut {
      from { opacity: 1; transform: translateX(0); }
      to { opacity: 0; transform: translateX(40px); }
    }
  `
  document.head.appendChild(style)
}

// Public API
export const toast = {
  success: (msg: string, duration = 3000) => showToast({ type: 'success', message: msg, duration }),
  error: (msg: string, duration = 4000) => showToast({ type: 'error', message: msg, duration }),
  warning: (msg: string, duration = 3000) => showToast({ type: 'warning', message: msg, duration }),
  info: (msg: string, duration = 3000) => showToast({ type: 'info', message: msg, duration }),
}

export function useToast() {
  return toast
}

/**
 * Sci-fi themed confirm dialog — replaces native window.confirm.
 * Returns true if user clicked "确认", false if "取消".
 */
export function confirmMsg(message: string, title = '确认操作'): Promise<boolean> {
  return new Promise(resolve => {
    if (typeof window !== 'undefined' && typeof window.confirm === 'function') {
      // Fallback for SSR or environments without our overlay
      resolve(window.confirm(message))
      return
    }

    // Remove existing overlay if any
    const existing = document.getElementById('oa4-confirm-overlay')
    if (existing) existing.remove()

    const overlay = document.createElement('div')
    overlay.id = 'oa4-confirm-overlay'
    overlay.style.cssText = `
      position: fixed; inset: 0; z-index: 10000;
      background: rgba(0,0,0,0.7);
      display: flex; align-items: center; justify-content: center;
    `

    const box = document.createElement('div')
    box.style.cssText = `
      background: rgba(15, 23, 42, 0.98);
      border: 1px solid rgba(59,130,246,0.4);
      border-radius: 12px; padding: 24px;
      min-width: 300px; max-width: 420px;
      box-shadow: 0 0 30px rgba(59,130,246,0.2), 0 8px 32px rgba(0,0,0,0.6);
      font-family: 'JetBrains Mono', monospace; color: #e2e8f0;
    `

    const titleEl = document.createElement('div')
    titleEl.style.cssText = `
      font-family: 'Orbitron', sans-serif; font-size: 14px;
      color: #3b82f6; margin-bottom: 12px; font-weight: 700;
      text-shadow: 0 0 10px rgba(59,130,246,0.5);
    `
    titleEl.textContent = `⚠ ${title}`

    const msgEl = document.createElement('div')
    msgEl.style.cssText = `
      font-size: 13px; color: #cbd5e1; line-height: 1.6;
      margin-bottom: 20px; white-space: pre-wrap;
    `
    msgEl.textContent = message

    const btnRow = document.createElement('div')
    btnRow.style.cssText = 'display: flex; justify-content: flex-end; gap: 10px;'

    const cancelBtn = document.createElement('button')
    cancelBtn.textContent = '取消'
    cancelBtn.style.cssText = `
      padding: 7px 18px; border-radius: 6px; border: 1px solid #475569;
      background: transparent; color: #94a3b8; cursor: pointer;
      font-size: 13px; font-family: 'JetBrains Mono', monospace;
      transition: all 0.15s;
    `
    cancelBtn.onmouseover = () => { cancelBtn.style.borderColor = '#94a3b8'; cancelBtn.style.color = '#e2e8f0' }
    cancelBtn.onmouseout = () => { cancelBtn.style.borderColor = '#475569'; cancelBtn.style.color = '#94a3b8' }
    cancelBtn.onclick = () => { overlay.remove(); resolve(false) }

    const okBtn = document.createElement('button')
    okBtn.textContent = '确认'
    okBtn.style.cssText = `
      padding: 7px 18px; border-radius: 6px; border: none;
      background: #3b82f6; color: #fff; cursor: pointer;
      font-size: 13px; font-family: 'JetBrains Mono', monospace;
      font-weight: 600; transition: all 0.15s;
      box-shadow: 0 0 10px rgba(59,130,246,0.3);
    `
    okBtn.onmouseover = () => { okBtn.style.background = '#2563eb'; okBtn.style.boxShadow = '0 0 15px rgba(37,99,235,0.5)' }
    okBtn.onmouseout = () => { okBtn.style.background = '#3b82f6'; okBtn.style.boxShadow = '0 0 10px rgba(59,130,246,0.3)' }
    okBtn.onclick = () => { overlay.remove(); resolve(true) }

    btnRow.appendChild(cancelBtn)
    btnRow.appendChild(okBtn)
    box.appendChild(titleEl)
    box.appendChild(msgEl)
    box.appendChild(btnRow)
    overlay.appendChild(box)
    document.body.appendChild(overlay)

    // Close on overlay click
    overlay.onclick = (e: MouseEvent) => {
      if (e.target === overlay) { overlay.remove(); resolve(false) }
    }

    // Close on Escape
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') { document.removeEventListener('keydown', onKeyDown); overlay.remove(); resolve(false) }
    }
    document.addEventListener('keydown', onKeyDown)
  })
}
