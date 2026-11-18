// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R217 搜索输入框可访问名称增强（WCAG 4.1.2 Name/Role/Value、3.3.2 Labels or Instructions）。
 *
 * 桌面端 40+ 个 CRUD 视图的搜索框只有 placeholder 文本：
 *   <input v-model="search" placeholder="搜索标题 / 分类..." class="search-input" />
 * placeholder 不是可访问名称——用户键入首字符后它即消失，多数读屏器也不会把它当 label。
 * 全局增强器在作者未提供 aria-label / aria-labelledby / title / 关联 label 时，
 * 用 placeholder 文本补出 aria-label；已有名称一律尊重，卸载时还原我们添加的属性。
 */

type A11yRoot = Document | Element | DocumentFragment

const SEARCH_SELECTOR = 'input.search-input, input.search-input-lg'

/** 判断输入控件是否已有任何形式的可访问名称。 */
function hasAccessibleName(input: HTMLInputElement): boolean {
  if (input.hasAttribute('aria-label') || input.hasAttribute('aria-labelledby') || input.title.trim()) return true
  const id = input.getAttribute('id')
  if (id && input.ownerDocument.querySelector('label[for="' + CSS.escape(id) + '"]')) return true
  // 包裹式 label：<label>搜索<input class="search-input" /></label>
  return Boolean(input.closest('label'))
}

export function installSearchFieldA11y(root: A11yRoot = document): () => void {
  let disposed = false
  const injections: HTMLInputElement[] = []
  const seen = new WeakSet<HTMLInputElement>()

  function apply(): void {
    if (disposed) return
    const inputs = Array.from(root.querySelectorAll<HTMLInputElement>(SEARCH_SELECTOR))
    for (const input of inputs) {
      if (seen.has(input) || hasAccessibleName(input)) continue
      const placeholder = input.getAttribute('placeholder')?.trim()
      if (!placeholder) continue
      input.setAttribute('aria-label', placeholder)
      seen.add(input)
      injections.push(input)
    }
  }

  const observer = new MutationObserver(apply)
  observer.observe(root, { childList: true, subtree: true, attributes: true, attributeFilter: ['placeholder'] })
  apply()

  return () => {
    if (disposed) return
    disposed = true
    observer.disconnect()
    for (const input of injections) input.removeAttribute('aria-label')
    injections.length = 0
  }
}
