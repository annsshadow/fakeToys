// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R188 全局表单校验错误无障碍关联（WCAG 3.3.1 / 4.1.2）。
 *
 * 全仓没有统一 FormItem，错误文本散落在各视图；工作流表单统一由 XformRuntime 渲染。
 * 这里提供全局运行时增强器：任何带 data-oa-field-error 标记的错误文本，都会被关联到
 * 同字段控件——错误文本补 role=alert + 稳定 id，控件补 aria-invalid=true 与
 * aria-describedby。属性变更由 MutationObserver 驱动，兼容 Vue 的 v-if 异步插入/移除。
 */

/** 标准错误标记：表单错误文本节点用该属性声明自己是字段级错误。 */
export const FIELD_ERROR_MARKER = 'data-oa-field-error'

const CONTROL_SELECTOR = 'input, textarea, select, [contenteditable="true"]'
type A11yRoot = Document | Element | DocumentFragment

type OriginalAttrs = { invalid: string | null; describedby: string | null }
const originals = new WeakMap<HTMLElement, OriginalAttrs>()
let idSequence = 0

function remember(control: HTMLElement): void {
  if (!originals.has(control)) {
    originals.set(control, {
      invalid: control.getAttribute('aria-invalid'),
      describedby: control.getAttribute('aria-describedby'),
    })
  }
}

function restore(control: HTMLElement): void {
  const original = originals.get(control)
  if (!original) return
  if (original.invalid === null) control.removeAttribute('aria-invalid')
  else control.setAttribute('aria-invalid', original.invalid)
  if (original.describedby === null) control.removeAttribute('aria-describedby')
  else control.setAttribute('aria-describedby', original.describedby)
  originals.delete(control)
}

function ensureErrorId(error: HTMLElement): void {
  if (error.getAttribute('id')) return
  const doc = error.ownerDocument
  let id = ''
  do {
    idSequence += 1
    id = `oa-field-error-${String(idSequence).padStart(6, '0')}`
  } while (doc?.getElementById(id))
  error.setAttribute('id', id)
}

/** 从错误文本向上找最近的、含表单控件的字段容器，返回该容器内的控件。 */
function findControls(error: HTMLElement): HTMLElement[] {
  let node = error.parentElement
  while (node) {
    const controls = Array.from(node.querySelectorAll<HTMLElement>(CONTROL_SELECTOR))
    if (controls.length) {
      if (controls.length > 1) {
        const labelled = new Set(
          Array.from(node.querySelectorAll<HTMLLabelElement>('label[for]'))
            .map((label) => label.getAttribute('for'))
            .filter((value): value is string => Boolean(value)),
        )
        const preferred = controls.filter((control) => labelled.has(control.id))
        if (preferred.length) return preferred
      }
      return controls
    }
    node = node.parentElement
  }
  return []
}

/**
 * 安装全局增强器；返回卸载函数。立即处理已存在的错误，并观察后续 DOM 变化。
 */
export function installFormErrorA11y(root: A11yRoot = document): () => void {
  const associated = new Map<HTMLElement, Set<HTMLElement>>()
  let disposed = false

  function refresh(): void {
    if (disposed) return
    const errors = Array.from(root.querySelectorAll<HTMLElement>(`[${FIELD_ERROR_MARKER}]`))
    const next = new Map<HTMLElement, Set<HTMLElement>>()
    for (const error of errors) {
      if (!error.hasAttribute('role')) error.setAttribute('role', 'alert')
      ensureErrorId(error)
      for (const control of findControls(error)) {
        let set = next.get(control)
        if (!set) {
          set = new Set()
          next.set(control, set)
        }
        set.add(error)
      }
    }

    // 清理已失联控件（错误被移除或标记被撤销）。
    for (const control of associated.keys()) {
      if (!next.has(control)) restore(control)
    }
    associated.clear()

    for (const [control, set] of next) {
      remember(control)
      const original = originals.get(control)
      if (!original) continue
      const ids = new Set((original.describedby || '').split(/\s+/).filter(Boolean))
      for (const error of set) {
        const id = error.getAttribute('id')
        if (id) ids.add(id)
      }
      control.setAttribute('aria-invalid', 'true')
      control.setAttribute('aria-describedby', Array.from(ids).join(' '))
      associated.set(control, set)
    }
  }

  const observer = new MutationObserver(refresh)
  observer.observe(root, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: [FIELD_ERROR_MARKER],
  })
  refresh()

  return () => {
    if (disposed) return
    disposed = true
    observer.disconnect()
    for (const control of associated.keys()) restore(control)
    associated.clear()
  }
}
