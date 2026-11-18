// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
const _queue: ToastOptions[] = []
const MAX_VISIBLE = 4

function getContainer(): HTMLDivElement {
  if (!container) {
    container = document.createElement('div')
    container.className = 'oa4-toast-container'
    container.setAttribute('role', 'status')
    container.setAttribute('aria-live', 'polite')
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
  // 无 DOM 环境（原生 App 壳 / SSR）下 toast 无处渲染，静默降级而非抛错。
  if (typeof document === 'undefined') return
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
  el.setAttribute('role', type === 'error' ? 'alert' : 'status')
  el.setAttribute('aria-live', type === 'error' ? 'assertive' : 'polite')
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
  const iconEl = document.createElement('span')
  iconEl.style.cssText = `color:${color};font-size:16px;font-weight:700;flex-shrink:0`
  iconEl.textContent = icon

  const messageEl = document.createElement('span')
  messageEl.style.cssText = 'flex:1;word-break:break-word'
  messageEl.textContent = message

  const statusEl = document.createElement('span')
  statusEl.style.cssText = `color:${color};font-size:10px;flex-shrink:0;opacity:0.7`
  statusEl.textContent = 'auto-hide'

  el.appendChild(iconEl)
  el.appendChild(messageEl)
  el.appendChild(statusEl)
  el.addEventListener('click', () => removeToast(el))
  container.appendChild(el)

  let timer = setTimeout(() => removeToast(el), duration)
  el.addEventListener('mouseenter', () => clearTimeout(timer))
  el.addEventListener('mouseleave', () => {
    clearTimeout(timer)
    timer = setTimeout(() => removeToast(el), duration / 2)
  })
}

function removeToast(el: HTMLElement): void {
  el.style.animation = 'oa4ToastOut 0.2s ease forwards'
  setTimeout(() => el.remove(), 200)
}

// Inject keyframes once（无 DOM 环境下跳过，模块须可被安全 import）
if (typeof document !== 'undefined' && !document.getElementById('oa4-toast-styles')) {
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

export async function copyText(text: string): Promise<boolean> {
  try {
    await navigator.clipboard.writeText(text)
    toast.success('已复制到剪贴板')
    return true
  } catch {
    toast.error('复制失败，请检查剪贴板权限')
    return false
  }
}

/**
 * 当前打开的确认框的关闭钩子。confirmMsg 是单例遮罩：重入时旧调用会被新调用抢占，
 * 必须让它的 Promise 以 false 落地并注销 keydown 监听，否则 await 的调用方会永久悬挂，
 * 且幽灵处理器会叠加在新弹窗上重复响应 Esc。
 */
let dismissOpenConfirm: (() => void) | null = null

/**
 * Sci-fi themed confirm dialog — replaces native window.confirm.
 * Returns true if user clicked "确认", false if "取消".
 */
export function confirmMsg(message: string, title = '确认操作'): Promise<boolean> {
  if (typeof document === 'undefined') return Promise.resolve(false)

  // 抢占式重入：先关闭仍打开的旧弹窗（false + 清理），再创建新的。
  dismissOpenConfirm?.()

  // Remember the element that triggered the dialog so focus can be restored on close.
  const opener = document.activeElement as HTMLElement | null

  return new Promise((resolve) => {
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
    box.setAttribute('role', 'dialog')
    box.setAttribute('aria-modal', 'true')
    box.setAttribute('aria-labelledby', 'oa4-confirm-title')
    box.setAttribute('aria-describedby', 'oa4-confirm-desc')
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
    titleEl.id = 'oa4-confirm-title'
    titleEl.textContent = `⚠ ${title}`

    const msgEl = document.createElement('div')
    msgEl.style.cssText = `
      font-size: 13px; color: #cbd5e1; line-height: 1.6;
      margin-bottom: 20px; white-space: pre-wrap;
    `
    msgEl.id = 'oa4-confirm-desc'
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
    cancelBtn.onmouseover = () => {
      cancelBtn.style.borderColor = '#94a3b8'
      cancelBtn.style.color = '#e2e8f0'
    }
    cancelBtn.onmouseout = () => {
      cancelBtn.style.borderColor = '#475569'
      cancelBtn.style.color = '#94a3b8'
    }
    let onKeyDown: (e: KeyboardEvent) => void
    let finished = false
    const finish = (result: boolean) => {
      if (finished) return
      finished = true
      document.removeEventListener('keydown', onKeyDown)
      if (dismissOpenConfirm === dismiss) dismissOpenConfirm = null
      overlay.remove()
      // Give focus back to the control that opened the dialog.
      if (opener && typeof opener.focus === 'function' && document.body.contains(opener)) opener.focus()
      resolve(result)
    }
    const dismiss = () => finish(false)

    cancelBtn.onclick = () => finish(false)

    const okBtn = document.createElement('button')
    okBtn.textContent = '确认'
    okBtn.style.cssText = `
      padding: 7px 18px; border-radius: 6px; border: none;
      background: #3b82f6; color: #fff; cursor: pointer;
      font-size: 13px; font-family: 'JetBrains Mono', monospace;
      font-weight: 600; transition: all 0.15s;
      box-shadow: 0 0 10px rgba(59,130,246,0.3);
    `
    okBtn.onmouseover = () => {
      okBtn.style.background = '#2563eb'
      okBtn.style.boxShadow = '0 0 15px rgba(37,99,235,0.5)'
    }
    okBtn.onmouseout = () => {
      okBtn.style.background = '#3b82f6'
      okBtn.style.boxShadow = '0 0 10px rgba(59,130,246,0.3)'
    }
    okBtn.onclick = () => finish(true)

    btnRow.appendChild(cancelBtn)
    btnRow.appendChild(okBtn)
    box.appendChild(titleEl)
    box.appendChild(msgEl)
    box.appendChild(btnRow)
    overlay.appendChild(box)
    document.body.appendChild(overlay)

    // Move focus into the dialog so keyboard/screen-reader users land on it.
    cancelBtn.focus()

    // Close on overlay click
    overlay.onclick = (e: MouseEvent) => {
      if (e.target === overlay) finish(false)
    }

    // Close on Escape; trap Tab inside the two action buttons.
    onKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        finish(false)
      } else if (e.key === 'Tab') {
        e.preventDefault()
        const active = document.activeElement
        if (e.shiftKey) {
          ;(active === cancelBtn ? okBtn : cancelBtn).focus()
        } else {
          ;(active === okBtn ? cancelBtn : okBtn).focus()
        }
      }
    }
    document.addEventListener('keydown', onKeyDown)
    dismissOpenConfirm = dismiss
  })
}
