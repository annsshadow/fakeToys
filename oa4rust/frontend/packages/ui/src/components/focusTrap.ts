// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R301 t12 命令面板焦点陷阱：aria-modal 对话框必须把键盘焦点限制在面板内，
 * 关闭后归还给触发者。兼容动态过滤命令项（每次 Tab 实时查询可聚焦元素）。
 */
export type FocusTrapHandle = { update(): void; release(): void }

const FOCUSABLE = [
  'a[href]',
  'button:not([disabled])',
  'input:not([disabled])',
  'select:not([disabled])',
  'textarea:not([disabled])',
  '[tabindex]:not([tabindex="-1"])',
].join(',')

function focusable(root: HTMLElement): HTMLElement[] {
  return Array.from(root.querySelectorAll<HTMLElement>(FOCUSABLE))
}

export function installFocusTrap(root: HTMLElement, restoreTo?: HTMLElement | null): FocusTrapHandle {
  let released = false
  const doc = root.ownerDocument
  if (!root.hasAttribute('tabindex')) root.setAttribute('tabindex', '-1')
  const initial = focusable(root)
  if (initial[0]) initial[0].focus()
  else root.focus()

  function onKeydown(e: KeyboardEvent): void {
    if (released || e.key !== 'Tab') return
    const items = focusable(root)
    if (items.length === 0) {
      e.preventDefault()
      root.focus()
      return
    }
    const first = items[0]
    const last = items[items.length - 1]
    const active = doc.activeElement
    if (e.shiftKey && (active === first || !root.contains(active))) {
      e.preventDefault()
      last.focus()
    } else if (!e.shiftKey && (active === last || !root.contains(active))) {
      e.preventDefault()
      first.focus()
    }
  }

  doc.addEventListener('keydown', onKeydown, true)
  return {
    update() {},
    release() {
      if (released) return
      released = true
      doc.removeEventListener('keydown', onKeydown, true)
      restoreTo?.focus()
    },
  }
}
