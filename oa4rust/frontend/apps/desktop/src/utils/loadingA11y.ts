// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R215 全局加载状态无障碍增强（WCAG 4.1.3 Status Messages / 4.1.2）。
 *
 * 大量视图的加载指示器是纯骨架 DOM（<div class="loading-state"><div class="skel">
 * …），既无可见文本也无 role，读屏器在数据加载期间听不到任何状态变化。这里提供
 * 全局运行时增强器：为这类容器补 role="status" 与读屏可见的“加载中”文本，且
 * 尊重作者已有的 role / 文案。DOM 变化由 MutationObserver 驱动，兼容 Vue 的 v-if
 * 异步插入/移除；卸载时还原我们添加的属性与文本。
 */

type A11yRoot = Document | Element | DocumentFragment

const LOADING_SELECTOR = ['.loading-state', '.loading-indicator', '.loading-spinner', '.skeleton-loader'].join(', ')

const STATUS_TEXT = '加载中...'

type Injection = { element: HTMLElement; injectedText: boolean }

/**
 * 安装全局增强器；返回卸载函数。立即处理已存在的加载容器，并观察后续 DOM 变化。
 */
export function installLoadingA11y(root: A11yRoot = document): () => void {
  let disposed = false
  const injections: Injection[] = []
  const seen = new WeakSet<HTMLElement>()

  function apply(): void {
    if (disposed) return
    const containers = Array.from(root.querySelectorAll<HTMLElement>(LOADING_SELECTOR))
    for (const el of containers) {
      if (el.hasAttribute('role') || seen.has(el)) continue
      el.setAttribute('role', 'status')
      let injectedText = false
      if (!el.textContent?.trim()) {
        const span = el.ownerDocument.createElement('span')
        span.textContent = STATUS_TEXT
        el.appendChild(span)
        injectedText = true
      }
      seen.add(el)
      injections.push({ element: el, injectedText })
    }
  }

  const observer = new MutationObserver(apply)
  observer.observe(root, { childList: true, subtree: true })
  apply()

  return () => {
    if (disposed) return
    disposed = true
    observer.disconnect()
    for (const { element, injectedText } of injections) {
      element.removeAttribute('role')
      if (injectedText) element.textContent = ''
    }
    injections.length = 0
  }
}
